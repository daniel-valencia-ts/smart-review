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
}

pub fn review_item(grade: u8, item: &mut Item) {
    if grade >= 3 {
        if item.successful_recalls == 0 {
            item.time_interval = 1
        } else if item.successful_recalls == 1 {
            item.time_interval = 6;
        } else if item.successful_recalls == 1 {
            item.time_interval =
                ((item.time_interval as f32) * item.easiness_factor).round() as u16;
        }
        item.successful_recalls += 1
    } else {
        item.successful_recalls = 0;
        item.time_interval = 1;
    }

    item.easiness_factor = update_easiness_factor(item.easiness_factor, grade as f32);
    item.review_date = (Local::now() + Duration::days(item.time_interval as i64))
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
