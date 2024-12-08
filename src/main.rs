use std::io;

mod cmds;

fn main() -> Result<(), io::Error> {
    cmds::running_loop()?;
    Ok(())
}


