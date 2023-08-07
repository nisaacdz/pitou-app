use yew::prelude::*;

mod props;

pub use props::*;
mod clicks;

pub use clicks::*;

#[function_component]
pub fn DirIcon() -> Html {
    html! {
        <img src="./public/icons/main/opened_folder.png" alt="folder" width = "80%"/>
    }
}

#[function_component]
pub fn FileIcon() -> Html {
    html! {
        <img src="./public/icons/main/file.svg" alt="file" width = "80%"/>
    }
}

#[function_component]
pub fn SymLinkIcon() -> Html {
    html! {
        <img src="./public/icons/main/symlink.png" alt="symlink" width = "80%"/>
    }
}

#[function_component]
pub fn LoadingIcon() -> Html {
    html! {
        <img src="./public/icons/main/cantfindanything.png" alt="icon" />
    }
}

#[function_component]
pub fn DetailsIcon() -> Html {
    html! {
        <img height = "100%" src="./public/icons/top/details.png" alt="details" />
    }
}

#[function_component]
pub fn RefreshIcon() -> Html {
    html! {
        <img height = "100%" src="./public/icons/top/refresh2.png" alt="refresh" />
    }
}

#[function_component]
pub fn LoadingDisplay() -> Html {
    html! {
        <img width = "10%" src = "./public/icons/main/loading.gif" alt = "loading!" />
    }
}

#[function_component]
pub fn ClipboardIcon() -> Html {
    html! {
        <img height = "100%" src = "./public/icons/top/clipboard.png" alt = "clipboard" />
    }
}

#[function_component]
pub fn SettingsIcon() -> Html {
    html! {
        <img src="./public/icons/side/settings.png" alt="settings" width = "80%"/>
    }
}

#[function_component]
pub fn HomeIcon() -> Html {
    html! {
        html! {
            <img src="./public/icons/side/home.png" alt="home" width = "80%" />
        }
    }
}

#[function_component]
pub fn BackIcon() -> Html {
    html! {
        <img src="./public/icons/side/back_arrow.png" alt="back" width = "80%"/>
    }
}

#[function_component]
pub fn SearchIcon() -> Html {
    html! {
        <img height = "100%" src="./public/icons/top/search.png" alt="search" />
    }
}

#[function_component]
pub fn LockedIcon() -> Html {
    html! {
        <img src="./public/icons/side/locked.png" alt="locked" width = "80%"/>
    }
}

#[function_component]
pub fn CloudIcon() -> Html {
    html! {
        <img src="./public/icons/side/cloud_dir.png" alt="cloud" width = "80%"/>
    }
}

#[function_component]
pub fn CloudStorageIcon() -> Html {
    html! {
        <img src="./public/icons/side/cloud_dir.png" alt="cloud storage" width = "80%"/>
    }
}

#[function_component]
pub fn PasteIcon() -> Html {
    html! {
        <img height = "100%" src="./public/icons/top/paste.png" alt="paste" />
    }
}

#[function_component]
pub fn CutIcon() -> Html {
    html! {
        <img height = "100%" src="./public/icons/top/cut.png" alt="cut" />
    }
}

#[function_component]
pub fn CopyIcon() -> Html {
    html! {
        <img height = "100%" src="./public/icons/top/copy.png" alt="copy" />
    }
}

#[function_component]
pub fn BookmarksIcon() -> Html {
    html! {
        <img src="./public/icons/side/bookmark.png" alt="bookmarks" width = "80%"/>
    }
}

#[function_component]
pub fn HistoryIcon() -> Html {
    html! {
        <img src="./public/icons/side/history.png" alt="history" width = "80%"/>
    }
}

#[function_component]
pub fn AddFolder() -> Html {
    html! {
        <img height = "100%" src="./public/icons/top/add_folder.png" alt="new folder" />
    }
}

#[function_component]
pub fn AddFile() -> Html {
    html! {
        <img height = "100%" src="./public/icons/top/add_file.png" alt="new file" />
    }
}

#[function_component]
pub fn DeleteIcon() -> Html {
    html! {
        <img height = "100%" src="./public/icons/top/delete.png" alt="delete" />
    }
}

#[function_component]
pub fn RenameIcon() -> Html {
    html! {
        <img height = "100%" src="./public/icons/top/add_file.png" alt="new file" />
    }
}
