use stui_timer::util::{self, io, types};

//note from crossterm docs:
//      New line character will not be processed therefore println! can’t be used, use write! instead

// program entry point
fn main() -> types::UnitResult {
    //display welcome message and set terminal (crossterm -> ratatui)
    //      error handling done in function def
    io::set_terminal();

    //abstracted trait for getting terminal input
    //      (dependency injection)
    let mut terminal_event_reader = types::TerminalEventReader::new();

    //handle user input for starting timer -> start timer if yes
    /*
    let result: String =
        util::io::await_startup_choice(&mut terminal_event_reader).unwrap_or_else(|error| {
            io::clear_terminal();
            panic!("\n\rError while awaiting startup input: {}.", error);
        });
    */

    //give callback to call based on result
    /*
    util::io::handle_startup_choice(result, util::timer::timer).unwrap_or_else(|error| {
        io::clear_terminal();
        panic!("\n\rError handling startup input: {}", error);
    });
    */

    //function that blocks thread from terminating on program end before user presses a key
    util::io::event::blocking_await_keypress(&mut terminal_event_reader).unwrap_or_else(|error| {
        io::clear_terminal();
        panic!("\n\rError while awaiting exit keypress: {}.", error);
    });

    //error handling done in function def
    util::io::clear_terminal();

    Ok(())
}
