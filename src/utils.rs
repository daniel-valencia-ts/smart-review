use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::{DateTime, Duration, Local};
use clap::{Parser, Subcommand};
use colored::Colorize;
use rand::random;

pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

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
        writeln!(f, "{}: {}", "title".yellow().bold(), self.title)?;
        match &self.description {
            Some(desc) => {
                writeln!(f, "{}: {}", "description".green().bold(), desc)?;
            }
            None => (),
        }

        let date_to_display = match DateTime::from_timestamp(self.review_date, 0) {
            Some(utc) => {
                let raw: DateTime<Local> = DateTime::from(utc);
                let local = raw.format("%b-%d-%Y").to_string();
                if local == Local::now().format("%b-%d-%Y").to_string() {
                    String::from("Today")
                } else {
                    local
                }
            }
            None => String::from("invalid date"),
        };
        writeln!(f, "{}: {}", "review date".white().bold(), date_to_display)?;
        write!(f, "{}: {:x}", "id".red(), self.id)
    }
}

/// Review information with the spaced-repetition technique from your terminal.
#[derive(Parser)]
#[command(version, styles = CLAP_STYLING)]
pub struct Cli {
    #[command(subcommand)]
    pub operations: Operations,
}

#[derive(Subcommand)]
pub enum Operations {
    /// Create a new item with the given data
    New {
        /// Title of the item
        title: String,

        /// Optional description of the item
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Review an item, use the --help command for more details
    ///
    /// Explanation of each grade number:
    ///     0 - Total blackout
    ///     1 - Incorrect, but felt familiar afterwards
    ///     2 - Wrong, but easy after reviewing it
    ///     3 - Right answer, hard to remember
    ///     4 - Correct, but hesitated before answering
    ///     5 - Perfectly recalled
    #[command(verbatim_doc_comment)]
    Review {
        /// The ID or title of the target item
        raw_pattern: String,
        /// The self-evaluation of the quality of the review
        review_grade: u8,
    },
    /// Remove an item with the matching pattern
    Remove {
        /// The item's ID or title
        raw_pattern: String,
    },
    /// Show an item
    Show {
        /// The pattern can be omitted to enter the display mode
        #[arg(short = 'p', long = "pattern")]
        raw_pattern: Option<String>,
    },
}
