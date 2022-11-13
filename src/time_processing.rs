use std::thread::sleep;

use crate::template::Template;
use rand::seq::SliceRandom;
use time::{ext::NumericalStdDuration, format_description, Instant, OffsetDateTime};

// TODO
// - clean up 'err' and make fn panic
// - import language templates
// - default to english
pub(crate) fn get_time_in_words(template: &Template, local: OffsetDateTime) -> String {
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
    } else if preposition == Some(&String::new()) {
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

pub(crate) fn time_loop_template(mut earlier: Instant, template: &Template) -> ! {
    loop {
        let local = OffsetDateTime::now_local().unwrap();
        // Print on every full minute and update immediatley if the last update happened more then a minute ago
        if local.second() == 59 || earlier.elapsed() > 61.std_seconds() {
            // TODO rewrite the following with a cooldown time (30s?) for updates
            println!("{}", get_time_in_words(template, local));
            earlier = get_sys_time();
        }
        sleep(1.std_seconds());
    }
}

pub(crate) fn time_loop_simple(mut earlier: Instant) -> ! {
    loop {
        let local = OffsetDateTime::now_local().unwrap();
        // Print on every full minute and update immediatley if the last update happened more then a minute ago
        if local.second() == 59 || earlier.elapsed() > 61.std_seconds() {
            // TODO rewrite the following with a cooldown time (30s?) for updates
            println!("{}", get_simple_time());
            earlier = get_sys_time();
        }
        sleep(1.std_seconds());
    }
}

pub(crate) fn get_sys_time() -> Instant {
    Instant::now()
}

pub(crate) fn get_simple_time() -> String {
    let format = format_description::parse("[hour]:[minute]").unwrap();
    OffsetDateTime::now_local()
        .unwrap()
        .time()
        .format(&format)
        .unwrap()
}
