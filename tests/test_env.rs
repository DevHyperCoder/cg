use cg::*;
use std::env;

#[test]
fn test_hist_cmd() {
    env::remove_var("CG_HIST_CMD");

    // No env
    assert_eq!("history 0".to_string(), get_history_cmd());

    // With env
    env::set_var("CG_HIST_CMD", "history --all");
    assert_eq!("history --all".to_string(), get_history_cmd());
}

#[test]
fn test_data_dir_and_file() {
    env::remove_var("CG_DATA_DIR");

    assert_eq!(
        "~/.local/share/cg/".to_string(),
        get_data_dir().to_str().unwrap_or("")
    );
    assert_eq!(
        "~/.local/share/cg/data".to_string(),
        get_data_file().to_str().unwrap_or("")
    );

    // With env
    env::set_var("CG_DATA_DIR", "/tmp/cg/");

    assert_eq!(
        "/tmp/cg/".to_string(),
        get_data_dir().to_str().unwrap_or("")
    );
    assert_eq!(
        "/tmp/cg/data".to_string(),
        get_data_file().to_str().unwrap_or("")
    );
}
