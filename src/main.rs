mod logic;
mod types;

// program entry point
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    logic::io::program_welcome();
    
    //parse a yes or no response, else display exit message
    let response = logic::io::get_yes_no();

    if response == "y" {
        println!("Starting study timer...");
        logic::timer();
    } else {
        logic::io::exit_message();
    }

    Ok(())
}
