#[derive(Eq, PartialEq, Debug)]
pub struct History {
    pub hist_num: u32,
    pub cmd: String,
}
use regex::Regex;

pub fn parse_history(lines: Vec<String>) -> Vec<History> {
    let hist_regex = Regex::new("^([0-9]*)\\s([0-9a-zA-z-]+)").unwrap();

    let mut hist_vec = vec![];
    for i in lines {
        let cap = hist_regex.captures(&i).unwrap(); // TODO Potential bug

        let num = cap.get(1).map_or("", |m| m.as_str());
        let num = num.parse::<u32>().unwrap();

        let cmd = cap.get(2).map_or("", |m| m.as_str());

        let hist = History {
            cmd: cmd.to_string(),
            hist_num: num,
        };

        hist_vec.push(hist);
    }

    hist_vec
}
