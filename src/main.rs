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
mod time_in_words;

use crate::{template::Template, time_in_words::get_time_in_words};
use clap::{arg, command};
use std::{fs, path::Path, thread};
use time::{ext::NumericalStdDuration, format_description, Instant, OffsetDateTime};

// TODO implement structs properly
// - write Tests for special cases, testing randomness of prepositions, check some random times.
// - clean up 'err' and make fn panic
// - import language templates
// - default to english
// - add 'write to file'
// - load template files
// - handle Errors
fn main() {
    let matches = command!("Uhrwerk")
        .author("Johannes Mayrhofer <jm.spam@gmx.net>")
        .version(env!("CARGO_PKG_VERSION"))
        .about("prints current system time in words continuously")
        .args(&[
            arg!( -q --quit "Prints time in words only once."),
            arg!( -d --digital "Prints time as digital clock in HH:MM (24h)."),
            arg!( -l --language "Chose language. Available Languages see below"),
            arg!( -t --template "Specifiy a template path."),
            arg!( -o --output "Wirte output to file with specified path."),
            arg!([INPUT] "Sets the input path"),
            arg!([OUTPUT] "Sets the output path"),
        ])
        .get_matches();

    let template_path = matches.get_one::<String>("INPUT").map(String::as_str);
    let template = match template_path {
        Some(t) => load_template(Some(t.to_string())),
        _ => Template::default(),
    };
    if matches.get_flag("quit") {
        if matches.get_flag("digital") {
            println!("{}", get_simple_time());
        } else {
            println!(
                "{}",
                get_time_in_words(&template, OffsetDateTime::now_local().unwrap())
            );
        }
        return;
    };

    let earlier = get_sys_time();

    if matches.get_flag("digital") {
        println!("{}", get_simple_time());

        time_loop(earlier, &template)
    };

    println!(
        "{}",
        get_time_in_words(&template, OffsetDateTime::now_local().unwrap())
    );

    time_loop(earlier, &template)
}

fn time_loop(mut earlier: Instant, template: &Template) -> ! {
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

fn get_sys_time() -> Instant {
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

fn load_from_file(p: &str) -> Result<Template, Option<ron::de::SpannedError>> {
    if Path::new(&p).exists() {
        let contents = fs::read_to_string(&p);
        let template: Result<Template, ron::de::SpannedError> =
            ron::from_str(contents.unwrap().as_str());
        return match template {
            Ok(t) => Ok(t),
            Err(e) => Err(Some(e)),
        };
    }
    // This is a ugly hack until proper Errors get implemented
    Err(None)
}
