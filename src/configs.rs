



pub mod config {
    use std::{env, fs::OpenOptions, io::{Error, Read}};

    pub fn read_config(config_path: Option<&str>) -> Result<(), Error> {
    //! Reads from the home directory, e.g the user's name dir on Windows, and /home on linux.
    //! File name should be .fruityconf
        let mut config = OpenOptions::new().read(true).open(config_path.unwrap())?;
        let mut config_options = String::new();
        let possible_configs =  ["AUTOCOMPLETIONS","EXPERIMENTAL","PROMPT_NAME_COLOR","PROMPT_INPUT_COLOR"];
        config.read_to_string(&mut config_options)?;
        let parsed_config_vec = config_options.split("\n");
        for config_option in parsed_config_vec {
            for item in possible_configs {
                if config_option.contains(item) {
                    env::set_var(item, &config_option[item.len()..]);
                }
            }
        }
        Ok(())
    }
}