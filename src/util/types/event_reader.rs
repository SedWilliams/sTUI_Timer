/*********************************
 * EventReader Types
 *********************************/

use super::EventResult;

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
