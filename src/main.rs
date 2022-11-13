// Uhrwerk is a CLI tool to print 'time in words'
// It is heavily inspired by TickeTack (www.ticketack.de) and actually uses their language strings currently
// Copyright 2021, Johannes Mayrhofer
// License MIT
extern crate time;

mod default;
mod hours;
mod minutes;
mod prepositions;
mod special_cases;
mod template;

use crate::template::Template;
use clap::{App, Arg};
use rand::seq::SliceRandom;
use std::{fs, path::Path, thread};
use time::{ext::NumericalStdDuration, format_description, Instant, OffsetDateTime};

// TODO implement structs properly

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
    let mins_string = match minuten {
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
            mins_string,
            hour_string
        )
    } else {
        format!(
            "{} {} {} {}.",
            start_sentence.unwrap(),
            preposition.unwrap(),
            mins_string,
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
