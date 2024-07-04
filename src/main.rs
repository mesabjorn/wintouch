use std::env;
use std::fs::File;
use std::io::Error;

fn main() -> Result<(), Error> {
    // Collect the command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Skip the first argument which is the program's name
    for arg in &args[1..] {
        // Create an empty file for each argument
        match File::create(arg) {
            Ok(_) => println!("Created file: {}", arg),
            Err(e) => eprintln!("Failed to create file {}: {}", arg, e),
        }
    }
    
    Ok(())
}
