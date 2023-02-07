use chrono::{DateTime, prelude::*};

pub fn short_time(date: &String) -> String {

    let time = parse_to_date(date);

    format!("<t:{}:t>", time.timestamp())
}
pub fn short_date_time(date: &String) -> String {
    let time = parse_to_date(date);

    format!("<t:{}:f>", time.timestamp())
}
pub fn relative_time(date: &String) -> String {
    let time = parse_to_date(date);

    format!("<t:{}:R>", time.timestamp())
}

fn parse_to_date(date: &String) -> NaiveDateTime {
    // Parse string to NaiveDateTime for conversions
    todo!()
}