use rust_study_timer::util::{
    io,
    timer
};
use rust_study_timer::util::types::UnitResult;

//note from crossterm docs:
//      New line character will not be processed therefore println! can’t be used, use write! instead

// program entry point
fn main() -> UnitResult {
    
    //display welcome message and set terminal
    io::set_terminal()
        .expect(
            "Please run 'cargo fetch'"
        );
    
    //handle user input for starting timer -> start timer if yes
    let result: String = io::await_yes_no()
        .unwrap_or_else(|error| {
            eprintln!("\n\rError while awaiting yes/no input: {}", error);
            io::clear_terminal().expect("Please run 'cargo fetch'");
            //pass String to make the compiler happy
            "program terminated".into()
        }
    );

    io::handle_yes_no(result, timer::timer)
        .unwrap_or_else(|error| {
            eprintln!("\n\rError handling yes/no input: {}", error);
            io::clear_terminal().expect("Please run 'cargo fetch'");
        }
    );

    io::clear_terminal().expect("Please run 'cargo fetch'");

    Ok(())
}
