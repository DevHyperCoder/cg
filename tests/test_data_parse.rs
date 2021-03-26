use cg::data_parser::{parse_data, Data};

#[test]
fn test_data_parse() {
    let data_line = vec!["0 20-03-2020".to_owned(), "69 21-03-2020".to_owned()];

    let expected_data = vec![
        Data {
            hist_num: 0,
            date: "20-03-2020".to_owned(),
        },
        Data {
            hist_num: 69,
            date: "21-03-2020".to_owned(),
        },
    ];

    assert_eq!(expected_data, parse_data(data_line))
}
