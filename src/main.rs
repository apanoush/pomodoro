use std::ops::Add;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;
use std::time::SystemTime;

fn main() {
    
    let mut min: u32 = 0;
    let mut sec: u32 = 0;

    let second: Duration = Duration::from_secs(1);

    let current_time = SystemTime::now();
    let finish_time = current_time.add(Duration::from_secs_f32(200.0));

    println!("current time: {:?}, finish_time: {:?}", current_time, finish_time);

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
