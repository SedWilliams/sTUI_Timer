/*********************************
 * Type Defs
 *********************************/

use std::error::Error;

mod time_log;
pub use time_log::TimeLog;

mod event_reader;
pub use event_reader::{EventReader, TerminalEventReader};

//function return types
pub type UnitResult = Result<(), Box<dyn Error>>;
pub type StringResult = Result<String, Box<dyn Error>>;
pub type EventResult = Result<crossterm::event::Event, Box<dyn Error>>;
pub type KeyEventResult = Result<crossterm::event::KeyEvent, Box<dyn Error>>;
pub type TimerCallback = fn() -> Result<(), Box<dyn Error>>;
