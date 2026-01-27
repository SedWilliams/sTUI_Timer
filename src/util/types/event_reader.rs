/*********************************
 * EventReader Types
 *********************************/

use crossterm::event::{self, Event};

use super::{EventResult, KeyEventResult};

pub trait EventReader {
    fn read_event(&self) -> EventResult;
    fn poll_event(&self) -> KeyEventResult;
}

pub struct TerminalEventReader;

impl TerminalEventReader {
    pub fn new() -> Self {
        TerminalEventReader
    }
}

impl EventReader for TerminalEventReader {
    fn read_event(&self) -> EventResult {
        Ok(crossterm::event::read()?)
    }

    fn poll_event(&self) -> KeyEventResult {
        loop {
            if crossterm::event::poll(std::time::Duration::from_millis(1000))? {
                if let Event::Key(key) = event::read()? {
                    return Ok(key);
                }
            }
        }
    }
}
