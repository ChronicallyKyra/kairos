use chrono::NaiveDateTime;
use chrono::NaiveDate;
use chrono::NaiveTime;
use chrono::Timelike;
use chrono::offset::Utc;
use chrono::offset::Local;
use regex::Regex;

pub struct DiscordTimestamp {
    date: NaiveDateTime
}

impl DiscordTimestamp {
    pub fn to_short_time(&self) -> String {
        format!("<t:{}:t>", self.date.timestamp())
    }
    pub fn to_short_date_time(&self) -> String {    
        format!("<t:{}:f>", self.date.timestamp())
    }
    pub fn to_relative_time(&self) -> String {    
        format!("<t:{}:R>", self.date.timestamp())
    }
    pub fn to_short_date(&self) -> String {
        format!("<t:{}:d>", self.date.timestamp())
    }
}


pub fn parse_time(message: &str) -> Option<DiscordTimestamp> {
    let now = Utc::now();
    // Parse string to NaiveDateTime for conversions
    // Return new NaiveDateTime

    let time: NaiveTime;

    let time_pattern = Regex::new(r"\d+:\d{2}").unwrap();


    if time_pattern.is_match(&message) {
        println!("Parsing {}", message);

        time = NaiveTime::parse_from_str(message, "%R").unwrap();
        
    } else {
        return None;
    }

    let date_time = NaiveDateTime::new(now.date_naive(), time);

    Some(DiscordTimestamp { date: date_time })
}