pub mod hist_parser;

use std::env;
use std::path::PathBuf;

const HIST_CMD_ENV: &str = "CG_HIST_CMD";
const DATA_DIR_ENV: &str = "CG_DATA_DIR";
const DATA_FILE_NAME: &str = "data";
const DEFAULT_HIST_CMD: &str = "history 0";
const DEFAULT_DATA_DIR: &str = "~/.local/share/cg/";

pub fn get_history_cmd() -> String {
    env::var(HIST_CMD_ENV).unwrap_or_else(|_| DEFAULT_HIST_CMD.to_string())
}

pub fn get_data_dir() -> PathBuf {
    PathBuf::new().join(env::var(DATA_DIR_ENV).unwrap_or_else(|_| DEFAULT_DATA_DIR.to_string()))
}

pub fn get_data_file() -> PathBuf {
    PathBuf::new().join(get_data_dir()).join(DATA_FILE_NAME)
}
