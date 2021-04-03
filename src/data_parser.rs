use regex::Regex;
use std::fs;
use std::path::PathBuf;

// date -> dd:mm::yyyy 20-12-2021
#[derive(Debug, Eq, PartialEq)]
pub struct Data {
    pub hist_num: u32,
    pub date: String, // Find better data structure
}

// For now, we don't care abt the date regex.
pub fn parse_data(lines: Vec<&str>) -> Vec<Data> {
    let re = Regex::new("^([0-9]+)\\s([0-9]{2}-[0-9]{2}-[0-9]{4})").unwrap();

    let mut data_vec = vec![];

    for i in lines {
        let cap = re.captures(&i).unwrap();

        let num = cap.get(1).map_or("", |m| m.as_str());
        let num = num.parse::<u32>().unwrap();

        let date = cap.get(2).map_or("", |m| m.as_str());

        let data = Data {
            hist_num: num,
            date: date.to_string(),
        };

        data_vec.push(data);
    }
    data_vec
}
