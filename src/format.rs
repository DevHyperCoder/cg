pub fn format(hist_vec: Vec<(&String, &u32)>) {
    let mut total = 0;
    for i in &hist_vec {
        total += i.1;
    }

    println!("{}", total);
    let mut percev: Vec<(&String, u32)> = vec![];
    let mut largest = 0;
    for i in hist_vec {
        let percentage = ((*i.1 as f64 / total as f64) * 100.0).round();
        let percentage = percentage as u32;
        percev.push((i.0, percentage));

        // get the largest lenght
        if i.0.len() > largest {
            largest = i.0.len()
        }
    }

    for i in percev {
        if i.1 == 0 {
            continue;
        }

        let str_len = i.0.len();
        let mut space_str = String::new();
        let mut a = largest - str_len;

        while a > 0 {
            space_str.push(' ');
            a -= 1;
        }

        println!("{}:{}{} ({}%)", i.0, space_str, get_nice(&i.1),i.1);
    }
}

fn get_nice(num: &u32) -> String {
    let mut st = String::new();
    let mut i = 0;
    while &i < num {
        st.push('|');
        i += 1;
    }
    st
}
