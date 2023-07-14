use std::{collections::HashSet, sync::Arc};

use tokio::{fs, sync::Mutex};

use crate::Pitou;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub(crate) enum WhichTable {
    FAVORITES,
    HIDDENS,
    RECENTS,
}

macro_rules! map_table {
    ($tab:expr, $conn:expr) => {
        match $tab {
            WhichTable::FAVORITES => favorites::table
                .select(favorites::items)
                .load::<String>($conn),
            WhichTable::HIDDENS => hiddens::table.select(hiddens::items).load::<String>($conn),
            WhichTable::RECENTS => recents::table.select(recents::items).load::<String>($conn),
        }
    };
}

table! {
    favorites {
        id -> Integer,
        items -> Text,
    }
}

table! {
    hiddens {
        id -> Integer,
        items -> Text,
    }
}

table! {
    recents {
        id -> Integer,
        items -> Text,
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

fn create_tables(conn: &mut SqliteConnection) -> Result<(), diesel::result::Error> {
    diesel::sql_query("CREATE TABLE IF NOT EXISTS favorites (id INTEGER, items TEXT)")
        .execute(conn)?;
    diesel::sql_query("CREATE TABLE IF NOT EXISTS hiddens (id INTEGER, items TEXT)")
        .execute(conn)?;
    diesel::sql_query("CREATE TABLE IF NOT EXISTS recents (id INTEGER, items TEXT)")
        .execute(conn)?;

    Ok(())
}

pub async fn create_table_and_retry<O, F: FnOnce(&mut SqliteConnection) -> O>(
    conn: &mut SqliteConnection,
    f: F,
) -> O {
    create_tables(conn).unwrap();
    f(conn)
}

pub(crate) async fn get_from_database(
    conn: &mut SqliteConnection,
    table: WhichTable,
) -> Result<Vec<String>, diesel::result::Error> {
    let results = match map_table!(table, conn) {
        Ok(v) => v,
        Err(e) => match e {
            diesel::result::Error::DatabaseError(_, _) => {
                create_table_and_retry(conn, |conn| map_table!(table, conn)).await?
            }
            e => return Err(e),
        },
    };
    Ok(results)
}

static mut ONCE: Option<Arc<Mutex<Database>>> = None;
static CUR_DATABASE_URL: &str = "./temp/tempfile.db";

#[tokio::test]
async fn test_init() {
    let _ = get_or_init().await;
}

async fn get_or_init() -> Arc<Mutex<Database>> {
    unsafe {
        fs::create_dir_all(std::path::PathBuf::from(CUR_DATABASE_URL).parent().unwrap())
            .await
            .unwrap();
        let new_db = Arc::new(Mutex::new(Database::new(CUR_DATABASE_URL).unwrap()));
        ONCE.get_or_insert(new_db).clone()
    }
}

async fn get(table: WhichTable) -> Result<Vec<Pitou>, diesel::result::Error> {
    // Retrieve the JSON strings from the favorites table in the database
    let conn = get_or_init().await;
    let mut conn_lock = conn.lock().await;
    let connection = conn_lock.get_connection();

    let json_strings = get_from_database(connection, table).await?;

    // Deserialize the JSON strings into Pitou structs
    let mut res = Vec::new();
    for json_str in json_strings {
        let pitou = serde_json::from_str::<Pitou>(&json_str).unwrap();
        res.push(pitou);
    }

    Ok(res)
}

pub async fn mark_hidden(
    file: &Pitou,
    conn: &mut SqliteConnection,
) -> Result<(), diesel::result::Error> {
    let bytes = bincode::serialize(&file).unwrap();
    let json_str = serde_json::to_string(&bytes).unwrap();

    match find(&json_str, conn) {
        Ok(true) => Ok(()),
        Ok(false) => insert(&json_str, conn),
        Err(e) => match e {
            diesel::result::Error::DatabaseError(_, _) => {
                create_table_and_retry(conn, |conn| insert(&json_str, conn)).await
            }
            e => Err(e),
        },
    }
}

#[test]
fn test_serdejson_bincode() {}

macro_rules! json {
    ($file:expr) => {
        serde_json::to_string(&$file).unwrap()
    };
}

fn find(json: &str, db: &mut SqliteConnection) -> Result<bool, diesel::result::Error> {
    use crate::hiddens::dsl::*;
    let results = hiddens
        .filter(items.eq(json))
        .select(diesel::dsl::count_star())
        .first::<i64>(db)?;

    Ok(results > 0)
}

fn insert(json: &str, db: &mut SqliteConnection) -> Result<(), diesel::result::Error> {
    use crate::hiddens::dsl::*;
    diesel::insert_into(hiddens)
        .values(items.eq(json))
        .execute(db)?;

    Ok(())
}

pub async fn is_hidden(file: &Pitou, db: &mut Database) -> Result<bool, diesel::result::Error> {
    let json_str = json!(file);
    find(&json_str, &mut db.connection)
}

macro_rules! diesel_error {
    ($error:expr) => {{
        let error = $error;
        let kind = match &error {
            diesel::result::Error::InvalidCString(_) => std::io::ErrorKind::InvalidInput,
            diesel::result::Error::DatabaseError(_, _) => std::io::ErrorKind::Other,
            diesel::result::Error::NotFound => std::io::ErrorKind::NotFound,
            diesel::result::Error::QueryBuilderError(_) => std::io::ErrorKind::InvalidInput,
            diesel::result::Error::DeserializationError(_) => std::io::ErrorKind::InvalidData,
            diesel::result::Error::SerializationError(_) => std::io::ErrorKind::InvalidData,
            diesel::result::Error::RollbackErrorOnCommit {
                rollback_error: _,
                commit_error: _,
            } => std::io::ErrorKind::Other,
            diesel::result::Error::RollbackTransaction => std::io::ErrorKind::Other,
            diesel::result::Error::AlreadyInTransaction => std::io::ErrorKind::PermissionDenied,
            diesel::result::Error::NotInTransaction => std::io::ErrorKind::PermissionDenied,
            diesel::result::Error::BrokenTransactionManager => std::io::ErrorKind::BrokenPipe,
            _ => unimplemented!(),
        };

        std::io::Error::new(kind, error)
    }};
}

use super::{Favorites, Recents};

impl Favorites {
    pub async fn all() -> std::io::Result<Favorites> {
        let favorites = get(WhichTable::FAVORITES)
            .await
            .map_err(|e| diesel_error!(e))?;
        let hiddens = get(WhichTable::HIDDENS)
            .await
            .map_err(|e| diesel_error!(e))?
            .into_iter()
            .collect::<HashSet<Pitou>>();
        let values = favorites
            .into_iter()
            .filter(|fav| hiddens.contains(fav))
            .collect();

        Ok(Favorites { values })
    }
}

impl Recents {
    pub async fn all() -> std::io::Result<Recents> {
        let recents = get(WhichTable::RECENTS)
            .await
            .map_err(|e| diesel_error!(e))?;
        let hiddens = get(WhichTable::HIDDENS)
            .await
            .map_err(|e| diesel_error!(e))?
            .into_iter()
            .collect::<HashSet<Pitou>>();
        let values = recents
            .into_iter()
            .filter(|fav| hiddens.contains(fav))
            .collect();

        Ok(Recents { values })
    }
}
