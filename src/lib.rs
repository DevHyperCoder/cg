pub mod data_parser;
pub mod hist_parser;

use std::io::Read;

use std::env;
use std::io::stdin;
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

pub fn run() {
    // TODO before release
    // find a way to get the history into the program
    // running through cmd does not work currently due to it being a built in
    // for now, reading stdin

    // Read stdin
    let mut all_hist = String::new();
    stdin().read_to_string(&mut all_hist).unwrap();

    let mut lines = vec![];

    // Remove whitespace and push to vector
    for line in all_hist.lines() {
        lines.push(line.trim().to_string());
    }

    println!("{:?}", hist_parser::parse_history(lines));
}
