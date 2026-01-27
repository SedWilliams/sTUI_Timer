/*****************************************************
 * Program IO functions
 *****************************************************/
use crate::util::types::{TimerCallback, UnitResult};
use ui::app::App;
pub mod event;
pub mod ui;

use crossterm::style::Stylize;

/*****************************************************
 * INPUT (handling) FUNCTIONS
 *****************************************************/

pub use event::await_choice;

//handle yes_no()
//
//Params:
//      callback function
//      yes or no string (pass ownership)
//
//if yes -> callback()
//if no -> terminate prgm
pub fn handle_startup_choice(result: String, callback: TimerCallback) -> UnitResult {
    if result == "s" {
        println!("");
        println!("\rStarting timer...\r");
        callback()?;
    } else {
        exit_message();
    }
    Ok(())
}

//pub use event::blocking_await_keypress;

/*****************************************************
 * OUTPUT FUNCTIONS
 *****************************************************/

//welcome message, displays on program start
pub fn welcome_message() {
    println!("--------------------\r");
    println!("|   {}    |\r", " STUI Timer".blue().bold());
    println!("--------------------\r");
    println!("Press 's' to start the timer, 'v' to view logs, or 'q' to quit: \r");
}

//exit msg, displays on program close
pub fn exit_message() {
    println!("\rExiting the program. Goodbye!\r");
}

//sets up terminal for program use
pub fn set_terminal() {
    /*
    execute!(
        io::stdout(),
        SetTitle("Study Timer"),
        //enter alternate terminal buffer; MAX: 2 buffers
        EnterAlternateScreen,
        //SetBackgroundColor(Color::DarkGrey),
        //SetForegroundColor(Color::Blue),
        MoveTo(0, 0),
    ).unwrap_or_else(|error| {
        panic!("\n\rCrossterm error while setting terminal: {}. Please restart terminal and run 'cargo fetch' to install Crossterm.", error);
    });

    enable_raw_mode().unwrap_or_else(|error| {
        panic!("\n\rCrossterm error while enabling raw mode: {}. Please restart terminal and run 'cargo fetch' to install Crossterm.", error);
    });
    */

    ratatui::run(|terminal| App::init().run(terminal)).unwrap();
    //welcome_message();
}

//clears terminal and resets to normal screen
pub fn clear_terminal() {
    /*
    execute!(io::stdout(), LeaveAlternateScreen).unwrap_or_else(|error| {
        panic!("\n\rCrossterm error while clearing terminal: {}. Please restart terminal and run 'cargo fetch' to install Crossterm.", error);
    });

    disable_raw_mode().unwrap_or_else(|error| {
        panic!("\n\rCrossterm error while disabling raw mode: {}. Please restart terminal and run 'cargo fetch' to install Crossterm.", error);
    });
    */
}

//static spinner animation
pub fn spinner_animation() {
    unimplemented!();
}
