use std::{io, time::SystemTime};
use colored::{self, Colorize};
use whoami;
mod cmds;
use chrono::{self,DateTime, Local};

static VERSION: &str = env!("CARGO_PKG_VERSION");
fn main() -> Result<(), io::Error> {
    println!("Welcome, {0}!",whoami::username().bold());
    let time = SystemTime::now();
    let datetime: DateTime<Local> = time.into();
    let formatted =  datetime.format("%r %Z UTC, %A, %B %d, %Y").to_string();
    println!("The time is {0}.",formatted.bold().bright_blue());
    println!("You are currently using fruitysh v{0}",VERSION.black().on_white().bold());
    cmds::running_loop()?;
    Ok(())
}


