use std::{fs, io};

use start::start;

mod enums;
mod runner;
mod start;
mod tokenizer;
fn main() {
    let example = r#"
    #
    # Print hello world and do a simple calculation
    #
    
    .0 "hello world" # Set memory location 0 to string "hello world"

    $print .0 # Print memory location 0
    
    _d .0 # Delete memory location 0
    
    .0 1 # Set memory location 0 to integer 1
    .1 2 # Set memory location 1 to integer 2
    .2 0 # Set memory location 2 to integer 0 (To store the result of the calculation)
    
    _a .0 .1 .2 # Add memory location 0 and 1 and store the result in memory location 2
    
    $print .2 # Print memory location 2
    
    _d .0 # Delete memory location 0
    _d .1 # Delete memory location 1
    _d .2 # Delete memory location 2
    "#;

    println!(
        r#"
                     _____              
                     |  __ \             
  _ __   _____      _| |__) |____      __
 | '_ \ / _ \ \ /\ / /  ___/ _ \ \ /\ / /
 | |_) | (_) \ V  V /| |  | (_) \ V  V / 
 | .__/ \___/ \_/\_/ |_|   \___/ \_/\_/  
 | |                                     
 |_|                                     
  

 Assembly like programming language
        Written in Rust
"#
    );

    println!("Type a file name to run or :h to get help:");

    loop {
        let mut file_name = String::new();
        io::stdin()
            .read_line(&mut file_name)
            .expect(":( Failed to read line");
        let file_name = file_name.trim();

        match file_name {
            ":h" => {
                println!("\nHELP: ");
                println!(":h - Show this help");
                println!(":q - Quit");
                println!(":s - Show syntax");
                println!(":se - Save example to example.r");
                println!("(file name) - Run a file");
            }
            ":q" => {
                println!("Bye! Press enter to exit");
                io::stdin().read_line(&mut String::new()).unwrap();
                break;
            }
            ":s" => {
                println!("Showing basic syntax");

                println!("{}", example);
            }
            ":se" => {
                println!("... Saving example to example.r");
                fs::write("example.r", example).expect(":( Failed to write file");
            }
            _ => {
                println!("... Selecting file: {}", file_name);

                match fs::read_to_string(file_name) {
                    Ok(file) => {
                        println!("... Running file: {}\n\n", file_name);
                        start(&file);
                    }
                    Err(_) => {
                        println!("\nERROR: File does not exist or could not be read\n");
                    }
                }
            }
        }
    }
}
