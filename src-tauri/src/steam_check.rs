
use sysinfo::System;
use registry::{Hive, Security};
use std::process::Command;

fn is_steam_launched() -> bool {

    let mut sys = System::new_all();
    sys.refresh_all();

    for (pid, proc) in sys.processes() {

        if proc.name() == "steam.exe" {
            return true;
        }

    }

    false

}

pub fn find_steam_path() -> Result<String, &'static str> {

    let reg_key = Hive::LocalMachine.open(r"SOFTWARE\WOW6432Node\Valve\Steam", Security::Read);
    match reg_key {
        Ok(key) => {
        return Ok(key.value("InstallPath").map_err(|_| "Could not find InstallPath in the registry")?.to_string());
        },
        Err(err) => {
            println!("{}", err); 
            return Err("Could not find Steam in the registry"); 
        }
    }

}

pub fn launch_steam() {

    if is_steam_launched() {
        println!("Steam is already launched");
        return;
    }
    
    match find_steam_path() {

        Ok(path) => {

            println!("Launching Steam from path: {}", path);

            match Command::new(path + r"\steam.exe").spawn() {
                Ok(_) => {},
                Err(err) => {
                    println!("{}", err);
                }
            }

        },
        Err(err) => {
            println!("{}", err);
        }

    }

}