use std::{thread::sleep, time::Duration};

fn main() {
    loop{
        println!("Hello, world!");
        sleep(Duration::from_secs(2));
    }
}
