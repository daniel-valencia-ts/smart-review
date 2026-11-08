use std::fmt::Display;

use chrono::{Duration, Local};
use rand::random;

#[derive(Clone)]
pub struct Item {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub review_date: String,
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
            review_date: Local::now().format("%b-%d-%Y").to_string(),
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
        self.review_date = (Local::now() + Duration::days(self.time_interval as i64))
            .format("%b-%d-%Y")
            .to_string();
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
        writeln!(f, "{}", self.title)?;
        match &self.description {
            Some(desc) => {
                writeln!(f, "{}", desc)?;
            }
            None => (),
        }
        write!(f, "{}", self.review_date)
    }
}
