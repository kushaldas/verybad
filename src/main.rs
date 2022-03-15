#[macro_use]
extern crate rocket;
use std::io::Write;
use std::process::Command;
use std::{fs, str};
use tempfile::NamedTempFile;

#[get("/<path>")]
fn getpath(path: &str) -> String {
    println!("{}", path);
    let data = fs::read_to_string(path).expect("Can not read the given path");
    data
}

#[get("/<cmd>")]
fn exec(cmd: &str) -> String {
    println!("CMD: {}", cmd);
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

#[post("/<filename>", data = "<input>")]
fn new(filename: &str, input: Vec<u8>) -> String {
    let file = NamedTempFile::new().unwrap();

    let mut tfile = file.persist(format!("/tmp/{}", filename)).unwrap();
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
