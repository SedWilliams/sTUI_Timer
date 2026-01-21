/*****************************************************
 * Program IO Event Handling
 *
 * Abstracted event related logic that other
 * modules can depend on.
 *****************************************************/

use crate::util::types::{EventReader, EventResult, StringResult};

use crossterm::event::{Event, KeyCode, KeyEventKind};

//Wait for a startup choice keypress and return it as a small string token.
//      inject EventReader from types/event_reader.rs
pub fn await_startup_choice<R: EventReader>(reader: &mut R) -> StringResult {
    //wait for a recognized keypress and store result in `result`
    let result: String = loop {
        if let Event::Key(key) = reader.read_event()? {
            match key.code {
                KeyCode::Char('s') | KeyCode::Char('S') => break "s".into(),
                KeyCode::Char('q') | KeyCode::Char('Q') => break "q".into(),
                KeyCode::Char('v') | KeyCode::Char('V') => break "v".into(),
                _ => continue,
            }
        }
    };

    Ok(result)
}

//Blocking: wait until key press event is received (KeyEventKind::Press).
//
//returns the key event encountered (wrapped as `Event::Key`) so callers can
//      optionally inspect it.
pub fn blocking_await_keypress<R: EventReader>(reader: &mut R) -> EventResult {
    println!("\rPress any key to exit...\r");

    loop {
        match reader.read_event()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                return Ok(Event::Key(key_event));
            }
            _ => continue,
        }
    }
}
