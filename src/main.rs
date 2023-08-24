use std::ops::Add;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;
use chrono::{self, Timelike};

fn main() {
    
    // counters for seconds and minutes
    let mut min: u32 = 0;
    let mut sec: u32 = 0;

    // duration of a second
    let second: Duration = Duration::from_secs(1);

    // getting the local time and adding 20mins to get the finishing time
    let current_time = chrono::Local::now();
    let finish_time = current_time.add(chrono::Duration::minutes(20));
    // printing them
    println!("current time: {:02}:{:02}, finish time: {:02}:{:02}", current_time.hour(), current_time.minute(), finish_time.hour(), finish_time.minute());

    while min < 20 {

        print!("\r{:02}:{:02}", min, sec);
        std::io::stdout().flush().expect("flush error");
        sleep(second);

        sec += 1;

        if sec > 59 {
            sec = 0;
            min += 1;
        }
    }
}
