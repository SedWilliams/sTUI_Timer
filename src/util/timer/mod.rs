/*******************************************
 * Timer logic
 *******************************************/

use std::time;

use crate::util::types::{TimeLog, UnitResult};
use crate::util::update_time_log;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
};

pub fn timer() -> UnitResult {
    println!("\rType 'q' to stop the timer.\r");

    let start_time = std::time::Instant::now();

    let spinner_frames: [&str; 4] = ["|", "/", "-", "\\"];
    let mut current_frame: usize = 0;

    loop {
        if current_frame >= spinner_frames.len() {
            current_frame = 0;
        }
        execute!(
            std::io::stdout(),
            crossterm::cursor::MoveToColumn(0),
            crossterm::style::Print(format!(
                "\rElapsed time: {} seconds {} ",
                start_time.elapsed().as_secs(),
                &spinner_frames[current_frame]
            ))
        )
        .expect("Crossterm error, run 'cargo fetch'");
        current_frame += 1;

        if event::poll(time::Duration::from_millis(100))
            .expect("Crossterm error, run 'cargo fetch'")
        {
            if let Event::Key(key) = event::read().expect("Crossterm error, run 'cargo fetch'") {
                if key.code == KeyCode::Char('q') {
                    println!("\n\rTimer stopped.");
                    println!("");
                    break;
                } else {
                    continue;
                }
            }
        }
    }

    let formatted_time: TimeLog = TimeLog::from(start_time.elapsed());
    update_time_log(&formatted_time);

    Ok(())
}
