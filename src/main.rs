#[macro_use]
extern crate rocket;
use rocket_client_addr::ClientRealAddr;

use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::{env, fs, str};

#[get("/<path>")]
fn getpath(path: &str, client_addr: &ClientRealAddr) -> String {
    println!(
        "{} : GETPATH: {}",
        client_addr.get_ipv4_string().unwrap(),
        path
    );
    let data = fs::read_to_string(path).expect("Can not read the given path");
    data
}

#[get("/<cmd>")]
fn exec(cmd: &str, client_addr: &ClientRealAddr) -> String {
    println!("{} : CMD: {}", client_addr.get_ipv4_string().unwrap(), cmd);
    let mut cmds = cmd.split_whitespace().into_iter();
    let mut command = Command::new(cmds.next().unwrap());
    for c in cmds {
        command.arg(c);
    }

    let output = command.output().expect("Failed to execute the command.");
    str::from_utf8(&output.stdout).unwrap().to_string()
}

#[get("/")]
fn getos(client_addr: &ClientRealAddr) -> String {
    println!("{} : getos", client_addr.get_ipv4_string().unwrap());
    let data = fs::read_to_string("/etc/os-release").expect("Can not read the /etc/os-release.");
    data
}

#[get("/.well-known/security.txt")]
fn getsecuritytxt(client_addr: &ClientRealAddr) -> String {
    println!("{} : security.txt", client_addr.get_ipv4_string().unwrap());
    let data = fs::read_to_string("/etc/security.txt").expect("Can not read the /etc/security.txt");
    data
}

#[get("/")]
fn index(client_addr: &ClientRealAddr) -> String {
    println!("{} : /", client_addr.get_ipv4_string().unwrap());
    let path = env::current_dir().unwrap();
    let ps = path.display();
    format!(
        "Example of poorly written code.
    GET /getos -> will give the details of the OS.
    GET /filename -> will provide a file from the current directory
    GET /exec/date -> will give you the current date & time in the server.
    POST /filename -> Saves the data in filename.
    Code is running in: {}
    Source code is available at: https://github.com/kushaldas/verybad
    
    HACK THE PLANET!!

",
        ps
    )
}

#[post("/<filename>", data = "<input>")]
fn new(filename: &str, input: Vec<u8>, client_addr: &ClientRealAddr) -> String {
    println!(
        "{} : POST: filename: {}",
        client_addr.get_ipv4_string().unwrap(),
        filename
    );
    let mut tfile = File::create(filename).unwrap();
    tfile.write_all(&input).unwrap();
    "Okay".to_owned()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![getpath])
        .mount("/", routes![new])
        .mount("/getos", routes![getos])
        .mount("/exec/", routes![exec])
        .mount("/", routes![getsecuritytxt])
}
