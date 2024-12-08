use std::{env, fs::{self, OpenOptions}, io::{self,Error, ErrorKind, Read, Write}, os::windows::fs::MetadataExt, process::exit};
use supports_color;
use colored::{ColoredString, Colorize};
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
        Ok(_) => route_to_cmd(&input).unwrap_or_else(|err| {println!("There was an error while running {}: {:#?}",&input,err)}),
        Err(err) => panic!("fruitysh had to exit since you did not provide valid input. :{err:#?}") 
    // allowing for shell to open without closing, repetitive, but might work
    }
    input.clear();
    while input != "quit" && input != " " {
        
        print!("{}{} ~{}~ {}  ","fruitysh@".red(),whoami::username().red(),env::current_dir()?.to_str().unwrap().bold(),">>".green());
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => route_to_cmd(&input).unwrap_or_else(|err| {println!("{}: There was an error while running {}: {}","[fruitysh]".green(),&input.bold(),return_normal_error_msg(&err.kind()))}),
            Err(err) => println!("fruitysh had to exit since you did not provide valid input. :{err:#?}") 
        }
        input.clear();
        
    }
    Ok(())
}

fn return_normal_error_msg(kind: &ErrorKind) -> ColoredString {
    match kind {
        ErrorKind::NotFound => return "Could not find directory.".bold(),
        ErrorKind::PermissionDenied => return "Permission was denied.".bold(),
        _ => return "Unspecified error.".bold()

    }
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
        if arguments.get(2).is_some() && arguments.get(1).is_some() {
            tee(arguments[1],arguments[2])?;
        } else if arguments.get(1).is_none() {
            println!("You did not specify a path to write to.")
        } else if arguments.get(2).is_none() {
            println!("You did not specify text to write to PATH.")
        }
    } else if cmd.starts_with("quit") {
        exit(0);
    } else if cmd.starts_with("explore") && !cmd.contains("|") {
        let arguments: Vec<&str> = cmd.split(" ").collect();
        ls(arguments[1])?;
    } else if cmd.starts_with("clear") {
        print!("\x1B[2J")
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
        println!("{} \n switchdir [PATH] \n Help page for view: \n switchdir is a command that allows you to navigate around your cwd, using relative paths.","[fruitysh@switchdir]:".green())
    } else if path.contains(&new_dir_path[0..1]) || path.starts_with("/") {
        // first and second characters are always Drive letter names on windows 
        env::set_current_dir(path.trim_end())?;
    } else {
        new_dir_path.push_str("\\");
        new_dir_path.push_str(path.trim_end());
        env::set_current_dir(new_dir_path)?;
    }
    Ok(())
}

fn tee(path: &str,text: &str) -> Result<(),Error> {
    if path.trim_end() == "help" && text.trim_end() == "help" {
        println!("{} \n write [PATH] [TEXT] \n Help page for write: \n write is a command that allows you to write TEXT into a file at PATH.","[fruitysh@write]:".green())
    } else {
        let mut curr_working_dir = env::current_dir()?.to_str().unwrap().to_owned();
        curr_working_dir.push_str("\\");
        curr_working_dir.push_str(path);
        let mut pathfl= OpenOptions::new().read(true).write(true).append(true).open(curr_working_dir)?;
        let buf = text.as_bytes();
        pathfl.write(buf)?;
    }
    Ok(())
}

fn ls(path: &str) -> Result<(),Error> {
    println!("| {0: <10} | {1: <10} | {2: <10} |","Name".bold(),"Size".bold(),"Read-Only?".bold());
    let mut cur_working_dir = env::current_dir()?.to_str().unwrap().to_owned();
    cur_working_dir.push_str("\\");
    cur_working_dir.push_str(path.trim_end());
    for entry in fs::read_dir(cur_working_dir)? {
        let entry = entry?;
        let mut name = entry.file_name().into_string().unwrap().to_owned();
        if name.len() > 9 {
            name = name[0..10].to_string();
        }
        println!("| {0: <10} | {1: <10} | {2: <10} |",name,entry.metadata()?.file_size(),entry.metadata()?.permissions().readonly())
    }
    Ok(())
}