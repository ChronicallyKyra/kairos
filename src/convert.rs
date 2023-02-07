use std::time::SystemTime;

use chrono::NaiveDateTime;
use chrono::NaiveDate;
use chrono::NaiveTime;
use chrono::Timelike;
use chrono::offset::Local;
use regex::Regex;

pub struct DiscordTimestamp {
    date: NaiveDateTime
}

impl DiscordTimestamp {
    pub fn short_time(&self) -> String {
        format!("<t:{}:t>", self.date.timestamp())
    }
    pub fn short_date_time(&self) -> String {    
        format!("<t:{}:f>", self.date.timestamp())
    }
    pub fn relative_time(&self) -> String {    
        format!("<t:{}:R>", self.date.timestamp())
    }
    pub fn short_date(&self) -> String {
        format!("<t:{}:d>", self.date.timestamp())
    }
}


pub fn parse(message: &str) -> Option<DiscordTimestamp> {
    let now = Local::now();
    // Parse string to NaiveDateTime for conversions
    // Return new NaiveDateTime

    let date: NaiveDate;

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();


    if re.is_match(message) {
        println!("Parsing {}", message);

        date = NaiveDate::parse_from_str(message, "%Y-%m-%d").expect("Date from string");
    } else {
        return None;
    }

    let date_time = date.and_hms_opt(now.hour(), now.minute(), now.second()).unwrap();

    Some(DiscordTimestamp { date: date_time })
}