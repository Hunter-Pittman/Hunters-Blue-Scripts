use std::io;
use winreg::enums::*;
use winreg::RegKey;
use serde::{Serialize, Deserialize};

fn main() {
    let x = autorun_programs().unwrap();
    println!("{}", x);
}

#[derive(Serialize, Deserialize)]
struct Autorun {
    keyname: String,
    keyvalue: String
}

fn autorun_programs() -> io::Result<std::string::String> {
    let set_as_run = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run")?;
        
        let mut autorun_output = vec![];

        for (name, value) in set_as_run.enum_values().map(|x| x.unwrap()) {
            let value = Autorun {
                keyname: name,
                keyvalue: value.to_string()
            };

            autorun_output.push(value)
        }

        let autorun_json = serde_json::to_string_pretty(&autorun_output)?;

    Ok(autorun_json)
}