/*****************************************************
 * Program IO Event Handling
 *
 * Abstracted event related logic that other
 * modules can depend on.
 *****************************************************/

use crate::util::types::{EventReader, EventResult, StringResult};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

//Wait for a startup choice keypress and return it as a small string token.
//      inject EventReader from types/event_reader.rs
pub fn await_choice<R: EventReader>(reader: &R) -> StringResult {
    //wait for a recognized keypress and store result in `result`
    //
    if event::poll(std::time::Duration::from_millis(500))? {
        if let Event::Key(key) = reader.read_event()? {
            match key.code {
                KeyCode::Char('s') | KeyCode::Char('S') => return Ok("s".into()),
                KeyCode::Char('q') | KeyCode::Char('Q') => return Ok("q".into()),
                KeyCode::Char('v') | KeyCode::Char('V') => return Ok("v".into()),
                _ => {}
            }
        }
    }

    Ok("None".into())
}

//Blocking: wait until key press event is received (KeyEventKind::Press).
//
//returns the key event encountered (wrapped as `Event::Key`) so callers can
//      optionally inspect it.
pub fn blocking_await_keypress<R: EventReader>(reader: &R) -> EventResult {
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
/*
pub fn poll_event<R: EventReader>(reader: &R) -> StringResult {
    Ok(String::from(reader.poll_event()?))
}
*/
