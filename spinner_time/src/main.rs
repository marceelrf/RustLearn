use std::{thread, time::Duration};

fn main() {
    let frames = ["-", "\\", "|", "/"];
    for _ in 0..10 {
        for &frame in &frames {
            print!("\r{}", frame);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    }
}
