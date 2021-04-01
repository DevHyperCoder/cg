pub mod data_parser;
pub mod hist_parser;

use std::io::Read;
use std::env;
use std::io::stdin;
use std::path::PathBuf;
use std::collections::HashMap;

use hist_parser::History;
use data_parser::{parse_data, Data};

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

    let hist_vec = hist_parser::parse_history(lines);

    let mut cmd_map = generate_cmd_hash_map(hist_vec);

    let sorted_map = sort_cmd_hash_map(&mut cmd_map);

    print_beauty(sorted_map);
}

fn print_beauty(lines: Vec<(&String, &u32)>) {
    for i in lines {
        println!("{} {}", i.0, i.1)
    }
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


fn get_data(path: PathBuf) -> Vec<Data> {
    use std::fs;

    let file_content = fs::read_to_string(path).expect("Unable to read the file");

    parse_data(file_content.split("\n").collect::<Vec<&str>>())
}
