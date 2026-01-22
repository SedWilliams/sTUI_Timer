// all logic for input/output handling
pub mod io;

// data types used in the program
pub mod types;

// timer main logic
pub mod timer;

use std::env;
use std::fs::{File, OpenOptions, exists};
use std::io::Write;

use crate::util::io::clear_terminal;
use crate::util::types::TimeLog;

//updates time_log.txt with new session details
pub fn update_time_log(session_details: &TimeLog) {
    //abstract file existence check into its own function

    let time_log_txt_path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("time_log.txt");
    //println!("Time log file path: {:?}", &time_log_txt_path);

    //Handles time_log.txt file appending and existence checking
    match exists(&time_log_txt_path) {
        Ok(true) => {
            println!("\rTime log file exists. Appending new entry...\r");
            let file = OpenOptions::new()
                .append(true)
                .open(&time_log_txt_path)
                .unwrap_or_else(|error| {
                    clear_terminal();
                    panic!("\n\rFailed to open time_log.txt for appending: {}", error);
                });

            let json_entry = serde_json::to_string(&session_details).unwrap_or_else(|error| {
                clear_terminal();
                panic!("\n\rFailed to serialize session details to JSON: {}", error);
            });

            writeln!(&file, "{}", json_entry)
                .expect("Failed to write session details to time_log.txt, and session was not stored. Please file an issue.");
        }

        Ok(false) => {
            println!("\rTime log file does not exist. Creating new file...\r");

            let file = File::create(&time_log_txt_path).unwrap_or_else(|error| {
                clear_terminal();
                panic!("\n\rFailed to create time_log.txt: {}", error);
            });

            let json_entry = serde_json::to_string(&session_details).unwrap_or_else(|error| {
                clear_terminal();
                panic!("\n\rFailed to serialize session details to JSON: {}", error);
            });

            writeln!(&file, "{}", json_entry).unwrap_or_else(|error| {
                clear_terminal();
                panic!("\n\rFailed to write session details to time_log.txt, and session was not stored. Please file an issue: {}", error);
            });
        }

        Err(error) => {
            clear_terminal();
            panic!("Error checking file existence: {}", error);
        }
    }

    //show the user the session details that were logged
    println!("\rLogged {}\n\r", &session_details);
}
