use regex::Regex;

fn usage() -> ! {
    eprintln!("Usage: \n\tcargo day\n\tcargo day 6");
    std::process::exit(-1)
}

fn find_highest_day() -> Option<i32> {
    let dir = std::fs::read_dir("./src/bin").unwrap();
    let mut highest_day = None;
    let day_regex = Regex::new("^day([0-9])+$").unwrap();
    for a in dir.map(Result::unwrap) {
        if !a.file_type().unwrap().is_dir() {
            continue;
        }
        let filename = a.file_name();
        let Some(filename) = filename.to_str() else {
            continue;
        };
        let Some(captures) = day_regex.captures(filename) else {
            continue;
        };
        let Some(day_num) = captures.get(1) else {
            continue;
        };
        let day_num = day_num.as_str().parse().unwrap();
        highest_day = highest_day.max(Some(day_num));
    }
    highest_day
}

fn main() {
    let mut args = std::env::args().skip(1);
    let arg1 = args.next();
    let arg2 = args.next();
    let day = match (arg1.as_deref(), arg2.as_deref()) {
        (None, _) => format!(
            "day{}",
            dbg!(find_highest_day().expect("Error: no days found."))
        ),
        (Some(day), None) if day.parse::<u32>().is_ok() => format!("day{day}"),
        _ => usage(),
    };

    let mut cmd = std::process::Command::new("cargo")
        .args(["run", "--bin", day.as_str()])
        .spawn()
        .expect("Failed to spawn day");
    let _status = cmd.wait().expect("Failed to run day");
}
