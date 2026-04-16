// Serde handling for the TOML config structure
// Contained in this module to avoid clutter in main

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) files: Files,
    pub(crate) users: Users
}

#[derive(Deserialize)]
pub(crate) struct Files {
    pub(crate) web_dir: Option<String>,
    pub(crate) file_dir: Option<String>
}

#[derive(Deserialize)]
pub(crate) struct Users {

}