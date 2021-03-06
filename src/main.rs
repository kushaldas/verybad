#[macro_use]
extern crate rocket;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::{env, fs, str};

#[get("/<path>")]
fn getpath(path: &str) -> String {
    println!("GETPATH: {}", path);
    let data = fs::read_to_string(path).expect("Can not read the given path");
    data
}

#[get("/<cmd>")]
fn exec(cmd: &str) -> String {
    println!("CMD: {}", cmd);
    let mut cmds = cmd.split_whitespace().into_iter();
    let mut command = Command::new(cmds.next().unwrap());
    for c in cmds {
        command.arg(c);
    }

    let output = command.output().expect("Failed to execute the command.");
    str::from_utf8(&output.stdout).unwrap().to_string()
}

#[get("/")]
fn getos() -> String {
    let data = fs::read_to_string("/etc/os-release").expect("Can not read the /etc/os-release.");
    data
}

#[get("/")]
fn index() -> String {
    let path = env::current_dir().unwrap();
    let ps = path.display();
    format!(
        "Example of poorly written code.
    GET /getos -> will give the details of the OS.
    GET /filename -> will provide a file from the current directory
    GET /exec/date -> will give you the current date & time in the server.
    POST /filename -> Saves the data in filename.
    Code is running in: {}
    ",
        ps
    )
}

#[post("/<filename>", data = "<input>")]
fn new(filename: &str, input: Vec<u8>) -> String {
    println!("POST: filename: {}", filename);
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
}
