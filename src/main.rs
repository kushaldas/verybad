#[macro_use]
extern crate rocket;
use std::process::Command;
use std::{fs, str};

#[get("/<path>")]
fn getpath(path: &str) -> String {
    println!("{}", path);
    let data = fs::read_to_string(path).expect("Can not read the given path");
    data
}

#[get("/<cmd>")]
fn exec(cmd: &str) -> String {
    let output = Command::new(cmd)
        .output()
        .expect("Failed to execute the command.");
    str::from_utf8(&output.stdout).unwrap().to_string()
}

#[get("/")]
fn getos() -> String {
    let data = fs::read_to_string("/etc/os-release").expect("Can not read the /etc/os-release.");
    data
}

#[get("/")]
fn index() -> &'static str {
    "Example of poorly written code.
    /getos -> will give the details of the OS.
    /filename -> will provide a file from the current directory
    /exec/date -> will give you the current date & time in the server.
    "
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/getos", routes![getos])
        .mount("/", routes![getpath])
        .mount("/exec/", routes![exec])
}
