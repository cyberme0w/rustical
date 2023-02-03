//! `rustical` provides a way to interact with vcalendar (.ics) files.
//!
//! Each type of 

mod vcalendar;
use vcalendar::*;


use std::path::Path;
use std::fs;

fn generate_rustical_data_dir() -> std::io::Result<()> {
    fs::create_dir("./rustical_data")?;
    Ok(())
}

fn main() {
    let prodid = String::from("prodid");
    let version = String::from("version");
    let v = Vcalendar::new_vcalendar(prodid, version);

    println!("Generated empty calendar!");
    v.print_vcalendar();

    //generate_rustical_data_dir();
}
