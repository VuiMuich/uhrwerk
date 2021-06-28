use chrono::prelude::*;
use clap::{App, Arg};
use rand::seq::SliceRandom;
use std::{thread, time};

struct Hours {
    one: String,
    two: String,
    three: String,
    four: String,
    five: String,
    six: String,
    seven: String,
    eight: String,
    nine: String,
    ten: String,
    eleven: String,
    twelve: String,
    hour_err: String,
}
struct Minutes {
    five_past: String,
    ten_past: String,
    quarter_past: String,
    twenty_past: String,
    twenty_five_past: String,
    half_past: String,
    twenty_five_to: String,
    twenty_to: String,
    quarter_to: String,
    ten_to: String,
    five_to: String,
    mini_err: String,
}
struct Prepositions {
    almost: Vec<String>,
    exactly: Vec<String>,
    roughly: Vec<String>,
    prepo_err: Vec<String>,
}
struct SpecialCases {
    before_midnight: String,
    after_midnight: String,
    midnight: String,
    two_to_one: String,
    one_to_one: String,
    exactly_one: String,
    one_past_one: String,
    two_past_one: String,
    noon: String,
}

fn get_time_in_words(local: DateTime<Local>) -> String {
    let minutes = Minutes {
        five_past: String::from("fünf nach"),
        ten_past: String::from("zehn nach"),
        quarter_past: String::from("viertel nach"),
        twenty_past: String::from("zehn vor halb"),
        twenty_five_past: String::from("fünf vor halb"),
        half_past: String::from("halb"),
        twenty_five_to: String::from("fünf nach halb"),
        twenty_to: String::from("zehn nach halb"),
        quarter_to: String::from("dreiviertel"),
        ten_to: String::from("zehn vor"),
        five_to: String::from("fünf vor"),
        mini_err: String::from("Deine Minuten sind kaputt."),
    };

    let hours = Hours {
        one: String::from("eins"),
        two: String::from("zwei"),
        three: String::from("drei"),
        four: String::from("vier"),
        five: String::from("fünf"),
        six: String::from("sechs"),
        seven: String::from("sieben"),
        eight: String::from("acht"),
        nine: String::from("neun"),
        ten: String::from("zehn"),
        eleven: String::from("elf"),
        twelve: String::from("zwölf"),
        hour_err: String::from("Was ist mit deinen Stunden los?"),
    };
    let special_cases = SpecialCases {
        before_midnight: String::from("Es ist gleich Mitternacht."),
        midnight: String::from("Es ist Mitternacht."),
        after_midnight: String::from("Es ist nach Mitternacht."),
        two_to_one: String::from("Es ist demnächst ein Uhr."),
        one_to_one: String::from("Es ist kurz vor ein Uhr."),
        exactly_one: String::from("Es ist ein Uhr."),
        one_past_one: String::from("Es ist kurz nach ein Uhr."),
        two_past_one: String::from("Es ist kurz nach ein Uhr."),
        noon: String::from("Es ist Mittag."),
    };
    let prepositions = Prepositions {
        almost: vec![
            String::from("gleich"),
            String::from("fast"),
            String::from("in Kürze"),
            String::from("bald"),
            String::from("beinahe"),
        ],
        exactly: vec![
            String::from("exakt"),
            String::from("genau"),
            String::from("jetzt"),
            String::from(""),
        ],
        roughly: vec![
            String::from("circa"),
            String::from("etwa"),
            String::from("ungefähr"),
        ],
        prepo_err: vec![String::from("Bitte was??")],
    };
    let delta_minute = local.minute() % 5;
    // print!("Modulo 5 Minuten: {}, ", delta_minute);
    let preposition = match delta_minute {
        3 | 4 => prepositions.almost.choose(&mut rand::thread_rng()),
        0 => prepositions.exactly.choose(&mut rand::thread_rng()),
        1 | 2 => prepositions.roughly.choose(&mut rand::thread_rng()),
        _ => prepositions.prepo_err.choose(&mut rand::thread_rng()),
    };
    // println!("Minuten: {}", local.minute());
    let minuten = match local.minute() % 5 {
        3 | 4 => (local.minute() + 5) / 5,
        _ => local.minute() / 5,
    };
    // println!("Minuten: {}", minuten);
    let stunden = match minuten {
        4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => (local.hour12().1 + 1) % 12,
        _ => local.hour12().1,
    };
    let mini_string = match minuten {
        1 => minutes.five_past,
        2 => minutes.ten_past,
        3 => minutes.quarter_past,
        4 => minutes.twenty_past,
        5 => minutes.twenty_five_past,
        6 => minutes.half_past,
        7 => minutes.twenty_five_to,
        8 => minutes.twenty_to,
        9 => minutes.quarter_to,
        10 => minutes.ten_to,
        11 | 12 => minutes.five_to,
        _ => minutes.mini_err,
    };
    // println!("Stunden: {}", stunden);
    let hour_string = match stunden {
        0 | 12 => hours.twelve,
        1 => hours.one,
        2 => hours.two,
        3 => hours.three,
        4 => hours.four,
        5 => hours.five,
        6 => hours.six,
        7 => hours.seven,
        8 => hours.eight,
        9 => hours.nine,
        10 => hours.ten,
        11 => hours.eleven,
        _ => hours.hour_err,
    };
    let special_cases = match &*format!("{}:{}", local.hour(), local.minute()) {
        "23:58" | "23:59" => Some(special_cases.before_midnight),
        "00:00" => Some(special_cases.midnight),
        "00:01" | "00:02" => Some(special_cases.after_midnight),
        "00:58" | "12:58" => Some(special_cases.two_to_one),
        "00:59" | "12:59" => Some(special_cases.one_to_one),
        "01:00" | "13:00" => Some(special_cases.exactly_one),
        "01:01" | "13:01 " => Some(special_cases.one_past_one),
        "01:02" | "13:02" => Some(special_cases.two_past_one),
        "12:00" => Some(special_cases.noon),
        _ => None,
    };
    if special_cases.is_some() {
        special_cases.unwrap()
    } else if minuten == 0 {
        return format!("Es ist {} {}.", preposition.unwrap(), hour_string);
    } else if preposition == Some(&String::from("")) {
        return format!("Es ist {} {}.", mini_string, hour_string);
    } else {
        return format!(
            "Es ist {} {} {}.",
            preposition.unwrap(),
            mini_string,
            hour_string
        );
    }
}

fn get_sys_time() -> time::SystemTime {
    time::SystemTime::now()
}

fn main() {
    let matches = App::new("Uhrwerk")
        .author("Johannes Mayrhofer")
        .version(env!("CARGO_PKG_VERSION"))
        .about("prints current system time in words continuously")
        .arg(
            Arg::with_name("quit")
                .short("q")
                .long("quit")
                .help("Prints time in words only once."),
        )
        .get_matches();

    println!("{}", get_time_in_words(Local::now()));
    if matches.occurrences_of("quit") == 1 {
        return;
    };

    let mut earlier = get_sys_time();
    loop {
        let local: DateTime<Local> = Local::now();
        if get_sys_time().duration_since(earlier).unwrap() > time::Duration::from_secs(59)
            || local.second() == 0
        {
            println!("{}", get_time_in_words(local));
            earlier = get_sys_time();
        }
        thread::sleep(time::Duration::from_secs(1));
    }
}
