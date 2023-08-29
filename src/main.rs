use std::ops::Add;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;
use chrono::{self, Timelike};
use std::env;
use colored::Colorize;

fn main() {

    // collecting user argument if any
    let args: Vec<String> = env::args().collect();
    
    // error handling (parsing the argument)
    let waiting_time: i64 = ( if args.len() > 1 { 
        match args[1].parse::<i64>() {
            Ok(x) => if x > 0 {Ok(x)} else {panic!("Usage: {} ARG, ARG must be a positive integer", args[0])},
            Err(e) => Err(e)
        }
    } else {Ok(20)} ).expect("error in the argument, must be a positive integer");
  
    // counters for seconds and minutes
    let mut min: i64 = 0;
    let mut sec: i64 = 0;

    // duration of a second
    let second: Duration = Duration::from_secs(1);

    // getting the local time and adding 20mins to get the finishing time
    let current_time = chrono::Local::now();
    let finish_time = current_time.add(chrono::Duration::minutes(waiting_time));
    // printing them
    println!("current time: {:02}:{:02}, finish time: {:02}:{:02}", current_time.hour(), current_time.minute(), finish_time.hour(), finish_time.minute());

    while min < waiting_time {

        // printing the time and formatting it
        let print: String = format!("\r{:02}:{:02}", min, sec);
        print!("{}", print.bold());
        std::io::stdout().flush().expect("flush error");
        sleep(second);

        sec += 1;

        if sec > 59 {
            sec = 0;
            min += 1;
        }
    }
    println!("\nwell done!!:)")
}
