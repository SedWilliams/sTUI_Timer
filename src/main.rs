use rust_study_timer::*;

//note from crossterm docs:
//      New line character will not be processed therefore println! can’t be used, use write! instead

// program entry point
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    //display welcome message and set terminal
    util::io::set_terminal()
        .expect(
            "Please run 'cargo fetch'"
        );
    
    //handle user input for starting timer -> start timer if yes
    let result: String = util::io::await_yes_no()
        .unwrap_or_else(|error| {
            eprintln!("\n\rError while awaiting yes/no input: {}", error);
            util::io::clear_terminal().expect("Please run 'cargo fetch'");
            //pass String to make the compiler happy
            "program terminated".into()
        }
    );

    util::io::handle_yes_no(result, util::timer::timer)
        .unwrap_or_else(|error| {
            eprintln!("\n\rError handling yes/no input: {}", error);
            util::io::clear_terminal().expect("Please run 'cargo fetch'");
        }
    );

    util::io::clear_terminal().expect("Please run 'cargo fetch'");

    Ok(())
}
