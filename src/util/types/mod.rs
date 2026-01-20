/*********************************
 * Type Defs
 *********************************/

use std::error::Error;

mod time_log;
pub use time_log::TimeLog;

//function return types
pub type UnitResult = Result<(), Box<dyn Error>>;
pub type StringResult = Result<String, Box<dyn Error>>;
pub type EventResult = Result<crossterm::event::Event, Box<dyn Error>>;
pub type TimerCallback = fn() -> Result<(), Box<dyn Error>>;

pub trait EventReader {
    fn read_event(&mut self) -> EventResult;
}

pub struct TerminalEventReader;

impl TerminalEventReader {
    pub fn new() -> Self {
        TerminalEventReader
    }
}

impl EventReader for TerminalEventReader {
    fn read_event(&mut self) -> EventResult {
        Ok(crossterm::event::read()?)
    }
}
