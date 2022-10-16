// Uhrwerk is a CLI tool to print 'time in words'
// It is heavily inspired by TickeTack (www.ticketack.de) and actually uses their language strings currently
// Copyright 2021, Johannes Mayrhofer
// License MIT
extern crate time;

use clap::{App, Arg};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, thread};
use time::{ext::NumericalStdDuration, format_description, Instant, OffsetDateTime};

// TODO implement structs properly
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
struct Template {
    pub language: String,
    pub hours: Hours,
    pub minutes: Minutes,
    pub prepositions: Prepositions,
    pub special_cases: SpecialCases,
    pub start_sentence: Vec<String>,
    pub end_sentence: Vec<String>,
    pub on_the_hour_template: Vec<String>,
    pub normal_template: Vec<String>,
}

impl Default for Template {
    fn default() -> Self {
        let language = String::from("default_deutsch");
        let minutes: Minutes = Minutes {
            five_past: String::from("f\u{fc}nf nach"),
            ten_past: String::from("zehn nach"),
            quarter_past: String::from("viertel nach"),
            twenty_past: String::from("zehn vor halb"),
            twenty_five_past: String::from("f\u{fc}nf vor halb"),
            half_past: String::from("halb"),
            twenty_five_to: String::from("f\u{fc}nf nach halb"),
            twenty_to: String::from("zehn nach halb"),
            quarter_to: String::from("dreiviertel"),
            ten_to: String::from("zehn vor"),
            five_to: String::from("f\u{fc}nf vor"),
            mini_err: String::from("Deine Minuten sind kaputt."),
        };

        let hours: Hours = Hours {
            one: String::from("eins"),
            two: String::from("zwei"),
            three: String::from("drei"),
            four: String::from("vier"),
            five: String::from("f\u{fc}nf"),
            six: String::from("sechs"),
            seven: String::from("sieben"),
            eight: String::from("acht"),
            nine: String::from("neun"),
            ten: String::from("zehn"),
            eleven: String::from("elf"),
            twelve: String::from("zw\u{f6}lf"),
            hour_err: String::from("Was ist mit deinen Stunden los?"),
        };
        let special_cases: SpecialCases = SpecialCases {
            before_midnight: String::from("Es ist gleich Mitternacht."),
            midnight: String::from("Es ist Mitternacht."),
            after_midnight: String::from("Es ist nach Mitternacht."),
            two_to_one: String::from("Es ist demn\u{e4}chst ein Uhr."),
            one_to_one: String::from("Es ist kurz vor ein Uhr."),
            exactly_one: String::from("Es ist ein Uhr."),
            one_past_one: String::from("Es ist kurz nach ein Uhr."),
            two_past_one: String::from("Es ist kurz nach ein Uhr."),
            noon: String::from("Es ist Mittag."),
        };
        let prepositions: Prepositions = Prepositions {
            almost: vec![
                String::from("gleich"),
                String::from("fast"),
                String::from("in Kl\u{fc}rze"),
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
                String::from("ungef\u{e4}hr"),
            ],
            prepo_err: vec![String::from("Bitte was??")],
        };
        let start_sentence = vec![String::from("Es ist")];
        let end_sentence = vec![String::from("Uhr")];
        let on_the_hour_template = vec![
            String::from("start_sentence"),
            String::from("prepostition"),
            String::from("hour"),
            String::from("end_sentence"),
        ];
        let normal_template = vec![
            String::from("start_sentence"),
            String::from("prepostition"),
            String::from("minute"),
            String::from("hour"),
        ];
        //println!("Default template loaded.");
        Template {
            language,
            hours,
            minutes,
            prepositions,
            special_cases,
            start_sentence,
            end_sentence,
            on_the_hour_template,
            normal_template,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Prepositions {
    almost: Vec<String>,
    exactly: Vec<String>,
    roughly: Vec<String>,
    prepo_err: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

// TODO
// - clean up 'err' and make fn panic
// - import language templates
// - default to english
fn get_time_in_words(template: &Template, local: OffsetDateTime) -> String {
    let delta_minute = local.minute() % 5;
    // print!("Modulo 5 Minuten: {}, ", delta_minute);
    let start_sentence = template.start_sentence.choose(&mut rand::thread_rng());
    let preposition = match delta_minute {
        3 | 4 => template.prepositions.almost.choose(&mut rand::thread_rng()),
        0 => template
            .prepositions
            .exactly
            .choose(&mut rand::thread_rng()),
        1 | 2 => template
            .prepositions
            .roughly
            .choose(&mut rand::thread_rng()),
        _ => template
            .prepositions
            .prepo_err
            .choose(&mut rand::thread_rng()),
    };
    // println!("Minuten: {}", local.minute());
    let minuten = match local.minute() % 5 {
        3 | 4 => (local.minute() + 5) / 5,
        _ => local.minute() / 5,
    };
    // println!("Minuten: {}", minuten);
    let stunden = match minuten {
        4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => (local.hour() % 12 + 1) % 12,
        _ => local.hour() % 12,
    };
    let mini_string = match minuten {
        1 => &template.minutes.five_past,
        2 => &template.minutes.ten_past,
        3 => &template.minutes.quarter_past,
        4 => &template.minutes.twenty_past,
        5 => &template.minutes.twenty_five_past,
        6 => &template.minutes.half_past,
        7 => &template.minutes.twenty_five_to,
        8 => &template.minutes.twenty_to,
        9 => &template.minutes.quarter_to,
        10 => &template.minutes.ten_to,
        11 | 12 => &template.minutes.five_to,
        _ => &template.minutes.mini_err,
    };
    // println!("Stunden: {}", stunden);
    let hour_string = match stunden {
        0 | 12 => &template.hours.twelve,
        1 => &template.hours.one,
        2 => &template.hours.two,
        3 => &template.hours.three,
        4 => &template.hours.four,
        5 => &template.hours.five,
        6 => &template.hours.six,
        7 => &template.hours.seven,
        8 => &template.hours.eight,
        9 => &template.hours.nine,
        10 => &template.hours.ten,
        11 => &template.hours.eleven,
        _ => &template.hours.hour_err,
    };
    let special_cases = match &*format!("{}:{}", local.hour(), local.minute()) {
        "23:58" | "23:59" => Some(&template.special_cases.before_midnight),
        "00:00" => Some(&template.special_cases.midnight),
        "00:01" | "00:02" => Some(&template.special_cases.after_midnight),
        "00:58" | "12:58" => Some(&template.special_cases.two_to_one),
        "00:59" | "12:59" => Some(&template.special_cases.one_to_one),
        "01:00" | "13:00" => Some(&template.special_cases.exactly_one),
        "01:01" | "13:01 " => Some(&template.special_cases.one_past_one),
        "01:02" | "13:02" => Some(&template.special_cases.two_past_one),
        "12:00" => Some(&template.special_cases.noon),
        _ => None,
    };
    if let Some(is_special_case) = special_cases {
        is_special_case.to_string()
    } else if minuten == 0 {
        format!(
            "{} {} {}.",
            start_sentence.unwrap(),
            preposition.unwrap(),
            hour_string
        )
    } else if preposition == Some(&String::from("")) {
        format!(
            "{} {} {}.",
            start_sentence.unwrap(),
            mini_string,
            hour_string
        )
    } else {
        format!(
            "{} {} {} {}.",
            start_sentence.unwrap(),
            preposition.unwrap(),
            mini_string,
            hour_string
        )
    }
}

fn get_sys_time() -> Instant {
    // SysTime::now()
    Instant::now()
}

fn get_simple_time() -> String {
    let format = format_description::parse("[hour]:[minute]").unwrap();
    OffsetDateTime::now_local()
        .unwrap()
        .time()
        .format(&format)
        .unwrap()
        .to_string()
}

fn load_template(template_path: Option<String>) -> Template {
    load_from_file(&template_path.unwrap())
        .map_err(|err| println!("ERROR loading template: {:?}", err))
        .unwrap_or_default()
}

fn load_from_file(p: &str) -> Result<Template, Option<ron::de::Error>> {
    if Path::new(&p).exists() {
        let contents = fs::read_to_string(&p);
        let template: Result<Template, ron::de::Error> = ron::from_str(contents.unwrap().as_str());
        return match template {
            Ok(t) => Ok(t),
            Err(e) => Err(Some(e)),
        };
    }
    // This is a ugly hack until proper Errors get implemented
    Err(None)
}

// TODO:
// - add 'write to file'
// - load template files
// - handle Errors
fn main() {
    let matches = App::new("Uhrwerk")
        .author("Johannes Mayrhofer <jm.spam@gmx.net>")
        .version(env!("CARGO_PKG_VERSION"))
        .about("prints current system time in words continuously")
        .arg(
            Arg::with_name("quit")
                .short("q")
                .long("quit")
                .help("Prints time in words only once."),
        )
        .arg(
            Arg::with_name("digital")
                .short("d")
                .long("digital")
                .help("Prints time as digital clock in HH:MM (24h)."),
        )
        .arg(
            Arg::with_name("language")
                .short("l")
                .long("language")
                .help("Chose language. Available Languages see below")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("template")
                .short("t")
                .long("template")
                .help("Specifiy a template path.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Wirte output to file with specified path.")
                .takes_value(true),
        )
        .get_matches();

    let template_path = matches.value_of("template");
    let template = match template_path {
        Some(t) => load_template(Some(t.to_string())),
        _ => Template::default(),
    };
    if matches.occurrences_of("quit") == 1 {
        if matches.occurrences_of("digital") == 1 {
            println!("{}", get_simple_time());
        } else {
            println!(
                "{}",
                get_time_in_words(&template, OffsetDateTime::now_local().unwrap())
            );
        }
        return;
    };

    let mut earlier = get_sys_time();

    if matches.occurrences_of("digital") == 1 {
        println!("{}", get_simple_time());

        loop {
            let local = OffsetDateTime::now_local().unwrap();
            // Print on every full minute and update immediatley if the last update happened more then a minute ago
            if local.second() == 59 || earlier.elapsed() > 61.std_seconds() {
                // TODO rewrite the following with a cooldown time (30s?) for updates
                println!("{}", get_simple_time());
                earlier = get_sys_time();
            }
            thread::sleep(1.std_seconds());
        }
    };

    println!(
        "{}",
        get_time_in_words(&template, OffsetDateTime::now_local().unwrap())
    );

    loop {
        let local = OffsetDateTime::now_local().unwrap();
        // Print on every full minute and update immediatley if the last update happened more then a minute ago
        if local.second() == 59 || earlier.elapsed() > 61.std_seconds() {
            // TODO rewrite the following with a cooldown time (30s?) for updates
            println!("{}", get_time_in_words(&template, local));
            earlier = get_sys_time();
        }
        thread::sleep(1.std_seconds());
    }
}

// TODO write Tests for special cases, testing randomness of prepositions, check some random times.
