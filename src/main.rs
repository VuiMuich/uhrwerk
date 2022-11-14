// Uhrwerk is a CLI tool to print 'time in words'
// It is heavily inspired by TickeTack (www.ticketack.de) and actually uses their language strings currently
// Copyright 2021, Johannes Mayrhofer
// License MIT

mod default;
mod file_handler;
mod hours;
mod minutes;
mod prepositions;
mod special_cases;
mod template;
mod time_processing;

use crate::{
    template::Template,
    time_processing::{
        get_simple_time, get_sys_time, get_time_in_words, time_loop_simple, time_loop_template,
    },
};
use clap::{arg, command};
use time::OffsetDateTime;

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
        Some(t) => file_handler::load_template(Some(t.to_string())),
        _ => Template::default(),
    };

    if matches.get_flag("quit") {
        if matches.get_flag("digital") {
            println!("{}", get_simple_time());
        } else {
            println!(
                "{}",
                get_time_in_words(&template, OffsetDateTime::now_local().unwrap().time())
            );
        }
        return;
    };

    let earlier = get_sys_time();

    if matches.get_flag("digital") {
        println!("{}", get_simple_time());

        time_loop_simple(earlier)
    };

    println!(
        "{}",
        get_time_in_words(&template, OffsetDateTime::now_local().unwrap().time())
    );

    time_loop_template(earlier, &template)
}
