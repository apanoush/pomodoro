use std::thread::sleep;
use std::time::Duration;
use std::io::Write;

fn main() {
    
    let mut min: u32 = 0;
    let mut sec: u32 = 0;

    let second: Duration = Duration::from_secs(1);

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
