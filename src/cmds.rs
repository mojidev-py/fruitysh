use std::{env, fs::OpenOptions, io::{self, Error, Read, Write}, process::exit};
use supports_color;
use colored::Colorize;
use whoami;






pub(crate) fn running_loop() -> Result<(),Error> {
    //! Internal function that runs the shell until an error occurs or a panic
    if !supports_color::on(supports_color::Stream::Stdout).is_some() {
        println!("Your current terminal does not support color. Visibility and readability may be impacted.")
    }
    let mut input = String::new();
    print!("{}{} ~{}~ {}  ","fruitysh@".red(),whoami::username().red(),env::current_dir()?.to_str().unwrap(),">>".green());
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut input) {
        Ok(_) => route_to_cmd(&input).expect("yuh"),
        Err(err) => panic!("fruitysh had to exit since you did not provide valid input. :{err:#?}") 
    // allowing for shell to open without closing, repetitive, but might work
    }
    input.clear();
    while input != "quit" && input != " " {
        
        print!("{}{} ~{}~ {}  ","fruitysh@".red(),whoami::username().red(),env::current_dir()?.to_str().unwrap(),">>".green());
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => route_to_cmd(&input).expect("Message for Quit:"),
            Err(err) => panic!("fruitysh had to exit since you did not provide valid input. :{err:#?}") 
        }
        input.clear();
        
    }
    Ok(())


        
        
}


fn route_to_cmd(cmd: &str) -> Result<(), Error> {
    //! Internal function that routes plain strings to functions that are implemented for the cmd, else returns `Not found`
    if cmd.contains("view") && !cmd.contains("|") {
        let arguments: Vec<&str> = cmd.split(" ").collect();
        cat(arguments[1])?;
    } else if cmd.contains("switchdir") && !cmd.contains("|") {
        let arguments: Vec<&str> = cmd.split(" ").collect();
        cd(arguments[1])?;
    } else if cmd.contains("write") && !cmd.contains("|") {
        let arguments: Vec<&str> = cmd.split(" ").collect();
        tee(arguments[1],arguments[2])?;
    } else if cmd.contains("quit") {
        exit(0);
    } else {
        println!("{}: Command {} was not found.","[fruitysh]".green(),cmd.bold())
    }
    Ok(())
    
}

fn cat(path: &str) -> Result<(),Error> {
    if path.trim_end() == "help" {
        println!("{} \n view [PATH] \n Help page for view: \n view is a command that allows you to view the contents of a file, if it's encoded in utf-8, which is likely the case. \n view searches for files relative to the cwd.","[fruitysh@view]".green())
    } else { 
    let mut output = String::new();
    let mut current_dir = env::current_dir()?.to_str().unwrap().to_owned();
    current_dir.push_str("\\");
    current_dir.push_str(path.trim_end());
    let mut pathfl = OpenOptions::new().read(true).open(current_dir)?;
    let metadata = pathfl.metadata()?.modified()?;
    pathfl.read_to_string(&mut output)?;
    print!("{} File modified at {:#?}. \n Contents: \n {}","[fruitysh@view]:".green(),metadata,&output);
    }
    Ok(())
}

fn cd(path: &str) -> Result<(), Error> {
    let mut new_dir_path = env::current_dir()?.to_str().unwrap().to_owned();
    if path.trim_end() == "help" {
        println!("{} \n switchdir [RELPATH] \n Help page for view: \n switchdir is a command that allows you to navigate around your cwd, using relative paths.","[fruitysh@switchdir]:".green())
    } else {
    new_dir_path.push_str(path.trim_end());
    env::set_current_dir(new_dir_path)?;
    }
    Ok(())
}

fn tee(path: &str,text: &str) -> Result<(),Error> {
    if path.trim_end() == "help" {
        println!("{} \n write [PATH] [TEXT] \n Help page for write: \n write is a command that allows you to write TEXT into a file at PATH.","[fruitysh@write]:".green())
    }
    let mut pathfl= OpenOptions::new().read(true).write(true).append(true).open(path)?;
    let buf = text.as_bytes();
    pathfl.write(buf)?;
    Ok(())
}