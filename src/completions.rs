


pub mod autocompletions {
    use std::{error::Error, fs::{File, OpenOptions}, io::{self, Read}, iter::zip};
    use colored::{self, Colorize, CustomColor};

    pub fn create_autocomp_file() -> Result<(), io::Error> {
        let mut path = "C:\\Users\\".to_string();
        path.push_str(&whoami::username());
        path.push_str("\\.autocompletes");
        let _ = OpenOptions::new().write(true).create_new(true).open(path)?;
        Ok(())
    }


    pub fn show_autocomplete(mut autocomp_file: &File,curr_input: &str) -> Result<(),Box<dyn Error>> {
        let mut parsed = String::new();
        autocomp_file.read_to_string(&mut parsed)?;
        let vectored = parsed.split("\n");

        for string in vectored {
            if !string.contains(curr_input) {
                continue;
            }
            zip(string.chars().enumerate(),curr_input.chars()).for_each(|((idx,_),_)| {
                print!("{}",&string[idx..].custom_color(CustomColor::new(128,128,128)));
                print!("{}","\x08".repeat(string[idx..].len()))
            });
        }

        Ok(()) 
    }
}