use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::{DateTime, Duration, Local};
use colored::{ColoredString, Colorize};
use rand::random;

#[derive(Clone)]
pub struct Item {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub review_date: i64, // Store the review date in POSIX time
    pub successful_recalls: u16,
    pub time_interval: u16,
    pub easiness_factor: f32,
}

impl Item {
    pub fn new(title: String, description: Option<String>) -> Self {
        Item {
            id: random::<u32>(),
            title,
            description,
            review_date: match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => n.as_secs() as i64,
                Err(_) => 0,
            },
            successful_recalls: 0,
            time_interval: 0,
            easiness_factor: 2.5,
        }
    }

    pub fn review(&mut self, grade: u8) {
        if grade >= 3 {
            if self.successful_recalls == 0 {
                self.time_interval = 1
            } else if self.successful_recalls == 1 {
                self.time_interval = 6;
            } else {
                self.time_interval =
                    ((self.time_interval as f32) * self.easiness_factor).round() as u16;
            }
            self.successful_recalls += 1
        } else {
            self.successful_recalls = 0;
            self.time_interval = 1;
        }

        self.easiness_factor = Self::update_easiness_factor(self.easiness_factor, grade as f32);
        let new_review_date = match DateTime::from_timestamp(self.review_date, 0) {
            Some(dt) => (dt + Duration::days(self.time_interval as i64)).timestamp(),
            None => self.time_interval as i64,
        };
        self.review_date = new_review_date;
    }

    fn update_easiness_factor(easiness_factor: f32, grade: f32) -> f32 {
        let mut new_ef = easiness_factor + (0.1 - (5.0 - grade) * (0.08 + (5.0 - grade) * 0.02));
        if new_ef < 1.3 {
            new_ef = 1.3
        }
        new_ef
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}: {}",
            ColoredString::from("title").yellow().bold(),
            self.title
        )?;
        match &self.description {
            Some(desc) => {
                writeln!(
                    f,
                    "{}: {}",
                    ColoredString::from("description").green().bold(),
                    desc
                )?;
            }
            None => (),
        }

        let date_to_display = match DateTime::from_timestamp(self.review_date, 0) {
            Some(utc) => {
                let local: DateTime<Local> = DateTime::from(utc);
                local.format("%b-%d-%Y").to_string()
            }
            None => String::from("invalid date"),
        };
        write!(
            f,
            "{}: {}",
            ColoredString::from("review date").white().bold(),
            date_to_display
        )
    }
}
