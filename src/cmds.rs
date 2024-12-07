use std::{fs::{self, OpenOptions}, io::{self, Error, Read}};
use supports_color;
use colored::Colorize;
use whoami;

fn RunningLoop() {
    //! Internal function that runs the shell until an error occurs or a panic
    if !supports_color::on(supports_color::Stream::Stdout).is_some() {
        println!("Your current terminal does not support color. Visibility and readability may be impacted.")
    }
    let mut input = String::new();
    while input != "quit" {
        print!("{}{}{}  ","fruitysh@".red(),whoami::username().red(),">>".green());
        match io::stdin().read_line(&mut input) {
            Ok(_) => route_to_cmd(&input).expect("yuh"),
            Err(err) => panic!("fruitysh had to exit since you did not provide valid input. :{err:#?}") 
        }
        
    }

    
        
        
        
}



fn route_to_cmd(cmd: &str) -> Result<(), Error> {
    //! Internal function that routes plain strings to functions that are implemented for the cmd, else returns `Not found`
    if cmd.contains("view") {
        let arguments: Vec<&str> = cmd.split(" ").collect();
        cat(arguments[1])?;
    }
    Ok(())
    
}

fn cat(path: &str) -> Result<(),Error> {
    let mut pathfl = OpenOptions::new().read(true).open(path)?;
    let mut output = String::new();
    let metadata = pathfl.metadata()?.modified()?;
    pathfl.read_to_string(&mut output)?;
    print!("{}{:#?} \n {}","[fruitysh@view]:".green(),metadata,&output);
    Ok(())
}