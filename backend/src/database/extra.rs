use std::sync::Arc;

use tokio::sync::Mutex;

use crate::Pitou;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

const DATABSE_PATH: &'static str = "./target/temp/database/database.db";

#[derive(Clone, Copy)]
pub(crate) enum WhichTable {
    Bookmarks,
    #[allow(dead_code)]
    Locked,
    History,
}

impl WhichTable {
    fn name(self) -> String {
        match self {
            WhichTable::Bookmarks => String::from("bookmarks"),
            WhichTable::History => String::from("history"),
            WhichTable::Locked => String::from("locked"),
        }
    }
}

impl std::fmt::Display for WhichTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write! {f, "{}", self.name()}
    }
}

macro_rules! map_table {
    ($tab:expr, $conn:expr) => {
        match $tab {
            WhichTable::Bookmarks => all_tables::bookmarks::table
                .select(all_tables::bookmarks::items)
                .load::<String>($conn),
            WhichTable::Locked => all_tables::locked::table
                .select(all_tables::locked::items)
                .load::<String>($conn),
            WhichTable::History => all_tables::history::table
                .select(all_tables::history::items)
                .load::<String>($conn),
        }
    };
}

mod all_tables {
    use super::table;

    table! {
        bookmarks {
            id -> Integer,
            items -> Text,
        }
    }

    table! {
        locked {
            id -> Integer,
            items -> Text,
        }
    }

    table! {
        history {
            id -> Integer,
            items -> Text,
        }
    }
}

pub struct Database {
    connection: SqliteConnection, // Assuming SQLite as the database
}

impl Database {
    pub fn new(database_url: &str) -> Result<Self, diesel::ConnectionError> {
        let connection = SqliteConnection::establish(database_url)?;
        Ok(Database { connection })
    }

    #[inline]
    pub fn get_connection(&mut self) -> &mut SqliteConnection {
        &mut self.connection
    }
}

async fn create_tables_or_do_nothing() {
    for table in [WhichTable::History, WhichTable::Bookmarks, WhichTable::Locked] {
        let str_query =
            format!("CREATE TABLE IF NOT EXISTS {table} (id INTEGER PRIMARY KEY, items TEXT)");
        diesel::sql_query(str_query)
            .execute(get_or_init().lock().await.get_connection())
            .expect(&format! {"couldn't create table {table}"});
    }
}

pub(crate) async fn get_from_database(
    table: WhichTable,
) -> Result<Vec<String>, diesel::result::Error> {
    match map_table!(table, get_or_init().lock().await.get_connection()) {
        Ok(v) => Ok(v),
        Err(e) => Err(e),
    }
}

static mut ONCE: Option<Arc<Mutex<Database>>> = None;

#[tokio::test]
async fn test_init() {
    let _ = get_or_init();
}

fn get_or_init() -> Arc<Mutex<Database>> {
    unsafe {
        ONCE.get_or_insert_with(|| {
            std::fs::create_dir_all(std::path::PathBuf::from(DATABSE_PATH).parent().unwrap())
                .unwrap();
            Arc::new(Mutex::new(Database::new(DATABSE_PATH).unwrap()))
        })
        .clone()
    }
}

async fn get_all(table: WhichTable) -> Vec<Pitou> {
    create_tables_or_do_nothing().await;
    get_from_database(table)
        .await
        .expect("couldn't retrieve table contents")
        .into_iter()
        .map(|json_str| serde_json::from_str::<Pitou>(&json_str).unwrap())
        .collect()
}

macro_rules! json {
    ($file:expr) => {
        serde_json::to_string(&$file).unwrap()
    };
}

async fn append(pitou: &Pitou, table: WhichTable) -> Result<(), diesel::result::Error> {
    let json = json!(pitou);
    create_tables_or_do_nothing().await;

    match table {
        WhichTable::Bookmarks => {
            diesel::insert_into(all_tables::bookmarks::table)
                .values(all_tables::bookmarks::dsl::items.eq(json))
                .execute(&mut get_or_init().lock().await.connection)?;
        }
        WhichTable::Locked => {
            diesel::insert_into(all_tables::locked::table)
                .values(all_tables::locked::dsl::items.eq(json))
                .execute(&mut get_or_init().lock().await.connection)?;
        }
        WhichTable::History => {
            diesel::insert_into(all_tables::history::table)
                .values(all_tables::history::dsl::items.eq(json))
                .execute(&mut get_or_init().lock().await.connection)?;
        }
    }
    Ok(())
}

async fn last_item(table: WhichTable) -> Option<Pitou> {
    create_tables_or_do_nothing().await;
    match table {
        WhichTable::History => {
            let pos = all_tables::history::table
                .count()
                .get_result::<i64>(get_or_init().lock().await.get_connection())
                .unwrap();

            if pos == 0 {
                return None;
            }
            let val: String = all_tables::history::table
                .select(all_tables::history::items)
                .filter(all_tables::history::id.eq(pos as i32))
                .first(get_or_init().lock().await.get_connection())
                .expect("couldn't read row item to text");

            Some(serde_json::from_str(&val).expect("couldn't change db text to Pitou"))
        }
        WhichTable::Bookmarks => {
            let pos = all_tables::bookmarks::table
                .count()
                .get_result::<i64>(get_or_init().lock().await.get_connection())
                .unwrap();
            if pos == 0 {
                return None;
            }
            let val: String = all_tables::bookmarks::table
                .select(all_tables::bookmarks::items)
                .filter(all_tables::bookmarks::id.eq(pos as i32))
                .first(get_or_init().lock().await.get_connection())
                .unwrap();

            Some(serde_json::from_str(&val).expect("couldn't change db text to Pitou"))
        }
        WhichTable::Locked => unimplemented!(),
    }
}

pub mod bookmarks {
    use super::{get_all, Pitou, WhichTable};

    pub async fn all() -> Vec<Pitou> {
        get_all(WhichTable::Bookmarks).await
    }

    pub async fn append(pitou: &Pitou) {
        super::append(pitou, WhichTable::Bookmarks)
            .await
            .expect("couldn't append to bookmarks")
    }
}

pub mod history {
    use super::{get_all, Pitou, WhichTable};

    pub async fn all() -> Vec<Pitou> {
        get_all(WhichTable::History).await
    }

    pub async fn append(pitou: &Pitou) {
        super::append(pitou, WhichTable::History)
            .await
            .expect("couldn't append to history")
    }

    pub async fn last() -> Option<Pitou> {
        super::last_item(WhichTable::History).await
    }
}
