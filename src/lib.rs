pub mod data_parser;
pub mod config;
pub mod format;
pub mod hist_parser;

use std::collections::HashMap;
use std::env;
use std::io::stdin;
use std::io::Read;
use std::path::PathBuf;

use format::format;
use hist_parser::History;

const HIST_CMD_ENV: &str = "CG_HIST_CMD";
const DATA_DIR_ENV: &str = "CG_DATA_DIR";
const DATA_FILE_NAME: &str = "data";
const DEFAULT_HIST_CMD: &str = "history 0";
const DEFAULT_DATA_DIR: &str = "~/.local/share/cg/";


pub fn run() {
    let mut all_hist = String::new();
    stdin().read_to_string(&mut all_hist).unwrap();

    let hist_vec = get_hist_vec(all_hist);
    let mut cmd_map = generate_cmd_hash_map(hist_vec);
    let sorted_map = sort_cmd_hash_map(&mut cmd_map);

    format(sorted_map);
}

fn get_hist_vec(all_hist: String) -> Vec<History> {
    let mut lines = vec![];

    // Remove whitespace and push to vector
    for line in all_hist.lines() {
        lines.push(line.trim().to_string());
    }

    hist_parser::parse_history(lines)
}

fn sort_cmd_hash_map(hist_map: &mut HashMap<String, u32>) -> Vec<(&String, &u32)> {
    let mut hist_vec: Vec<(&String, &u32)> = hist_map.iter().collect();
    hist_vec.sort_by(|a, b| b.1.cmp(&a.1));
    hist_vec
}

fn generate_cmd_hash_map(hist_vec: Vec<History>) -> HashMap<String, u32> {
    let mut hash = HashMap::new();
    for hist in hist_vec {
        *hash.entry(hist.cmd).or_insert(0) += 1;
    }
    hash
}

pub fn get_history_cmd() -> String {
    env::var(HIST_CMD_ENV).unwrap_or_else(|_| DEFAULT_HIST_CMD.to_string())
}

pub fn get_data_dir() -> PathBuf {
    PathBuf::new().join(env::var(DATA_DIR_ENV).unwrap_or_else(|_| DEFAULT_DATA_DIR.to_string()))
}

pub fn get_data_file() -> PathBuf {
    PathBuf::new().join(get_data_dir()).join(DATA_FILE_NAME)
}
