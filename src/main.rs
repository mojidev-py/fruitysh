use std::{io, process::exit, time::SystemTime};
use colored::{self, Colorize};
mod cmds;
mod configs;
mod completions;
use chrono::{self,DateTime, Local};
use configs::config::read_config;
use whoami::fallible::username;

static VERSION: &str = env!("CARGO_PKG_VERSION");
fn main() -> Result<(), io::Error> {
    let mut path = String::new();
    if std::env::consts::OS == "windows" {
        let mut win_curr_dir = "C:\\Users\\".to_string();
        win_curr_dir.push_str(&username()?);
        path.push_str(&win_curr_dir);
    } else {
        let lin_curr_dir = "/home\\".to_string();
        path.push_str(&lin_curr_dir);
    }
    if let Err(err) = read_config(Some(&path)) {
        println!("{}: {:#?}","[CRITICAL]".red(),err);
        exit(1);
    }
    println!("Welcome, {0}!",whoami::username().bold());
    let time = SystemTime::now();
    let datetime: DateTime<Local> = time.into();
    let formatted =  datetime.format("%r %Z UTC, %A, %B %d, %Y").to_string();
    println!("The time is {0}.",formatted.bold().bright_blue());
    println!("You are currently using fruitysh v{0}",VERSION.black().on_white().bold());
    cmds::running_loop()?;
    Ok(())
}


