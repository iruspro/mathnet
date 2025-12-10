use chrono::prelude::*;

#[derive(Debug)]
pub enum DateAndTime{
    MathNetDate(String)
}
pub fn display_current_date_and_time() -> DateAndTime{
    let time_here_and_now = Local::now();
    let formated_time_here_and_now = time_here_and_now.format("%F, %X").to_string();
    DateAndTime::MathNetDate(formated_time_here_and_now)
}

impl DateAndTime{
pub fn date_and_time_to_string(&self) -> String{
    match self {
        DateAndTime::MathNetDate(time) => time.clone(),
    }
}}

/* It should be considered what to do when users travel and switch time zones.
Current implementation means that if you send a message and then travel abroad and
change time zone in the process and then send a new message, you will have messages 
from different time zones, which is weird and confusing. */
/* Also get time stamps from database because it is weird that it refreshes every time
the page is refreshed. */