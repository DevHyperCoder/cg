use cg::hist_parser::{parse_history, History};

#[test]
fn test_history_parse() {
    let history_lines = vec![
        "0 cd ../".to_owned(),
        "1 git commit -m \"asd\"".to_owned(),
        "2 dmenu_run".to_owned(),
        "3 head -n 10".to_owned(),
        "4 sed11q-wrapper".to_owned(),
        "5 audio-control -i".to_owned(),
        "6 ../".to_owned(),
        "7 /tmp -i".to_owned(),
        "8 /mnt/old -i".to_owned(),
    ];

    let expected_history = vec![
        History {
            hist_num: 0,
            cmd: "cd".to_owned(),
        },
        History {
            hist_num: 1,
            cmd: "git".to_owned(),
        },
        History {
            hist_num: 2,
            cmd: "dmenu_run".to_owned(),
        },
        History {
            hist_num: 3,
            cmd: "head".to_owned(),
        },
        History {
            hist_num: 4,
            cmd: "sed11q-wrapper".to_owned(),
        },
        History {
            hist_num: 5,
            cmd: "audio-control".to_owned(),
        },
        History {
            hist_num: 6,
            cmd: "../".to_owned(),
        },
        History {
            hist_num: 7,
            cmd: "/tmp".to_owned(),
        },
        History {
            hist_num: 8,
            cmd: "/mnt/old".to_owned(),
        },
    ];

    assert_eq!(expected_history, parse_history(history_lines));
}
