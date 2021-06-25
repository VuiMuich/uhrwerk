use chrono::prelude::*;

fn main() {
    let local: DateTime<Local> = Local::now();
    let delta_minute = local.minute() % 5;
    // print!("Modulo 5 Minuten: {}, ", delta_minute);
    let preposition = match delta_minute {
        3 | 4 => "gleich",
        0 => "genau",
        1 | 2 => "circa",
        _ => "Bitte was??",
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
    let minuten = match minuten {
        1 => "fünf nach",
        2 => "zehn nach",
        3 => "viertel nach",
        4 => "zehn vor halb",
        5 => "fünf vor halb",
        6 => "halb",
        7 => "fünf nach halb",
        8 => "zehn nach halb",
        9 => "dreiviertel",
        10 => "zehn vor",
        11 | 12 => "fünf vor",
        _ => "Deine Minuten sind kaputt!",
    };
    // println!("Stunden: {}", stunden);
    let stunden = match stunden {
        0 | 12 => "zwölf",
        1 => "eins",
        2 => "zwei",
        3 => "drei",
        4 => "vier",
        5 => "fünf",
        6 => "sechs",
        7 => "sieben",
        8 => "acht",
        9 => "neun",
        10 => "zehn",
        11 => "elf",
        _ => "Keine Ahnung wie späte es ist!",
    };
    let special_cases = match &*format!("{}:{}", local.hour(), local.minute()) {
        "23:58" | "23:59" => Some("Es ist gleich Mitternacht."),
        "00:00" => Some("Es ist Mitternacht."),
        "00:01" | "00:02" => Some("Es ist nach Mitternacht."),
        "00:58" => Some("Es ist demnächst ein Uhr."),
        "00:59" => Some("Es ist kurz vor ein Uhr."),
        "01:00" => Some("Es ist ein Uhr."),
        "01:01" => Some("Es ist kurz nach ein Uhr."),
        "01:02" => Some("Es ist kurz nach ein Uhr."),
        "12:00" => Some("Es ist Mittag."),
        "12:58" => Some("Es ist demnächst ein Uhr."),
        "12:59" => Some("Es ist kurz vor ein Uhr."),
        "13:00" => Some("Es ist ein Uhr."),
        "13:01" => Some("Es ist kurz nach ein Uhr."),
        "13:02" => Some("Es ist kurz nach ein Uhr."),
        _ => None,
    };
    if special_cases.is_some() {
        println!("{}", special_cases.unwrap());
    } else if minuten == "genau" {
        println!("Es ist {} {}.", preposition, stunden);
    } else {
        println!("Es ist {} {} {}.", preposition, minuten, stunden);
        // println!("Stunden: {}, Minuten: {}", local.hour12().1, local.minute());
    }
}
