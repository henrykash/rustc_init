pub fn give_us_the_time() {
    println!("The current time is: {}", chrono::Local::now());
}

pub fn give_us_the_time2() {
    println!("The current time is: {}", chrono::Utc::now());
}