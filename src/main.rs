//! `rustical` provides a way to interact with vcalendar (.ics) files.

mod vcalendar;

use chrono::{Utc, TimeZone};
use clap::Parser;
use vcalendar::*;

/// A simple struct to hold arguments.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    new: bool,
    #[arg(short, long)]
    load: bool,
    #[arg(short, long)]
    path: Option<std::path::PathBuf>,
}

fn main() {
    // Get arguments and check if there are any invalid values or combinations.
    let args = Args::parse();
    println!("Arguments:");
    println!("  new: {:?}", args.new);
    println!("  load: {:?}", args.load);
    println!("  path: {:?}", args.path);

    // TODO: Make sure we are either creating a new vcalendar, OR loading a file. Not both.
    // if args.new == args.load { panic!() }

    // Generate a small VCALENDAR.
    let prodid = "prodid".to_string();
    let version = "2.0".to_string();
    let mut vcalendar = Vcalendar::new_vcalendar(prodid, version);

    // Create two VEVENTs based on the VCALENDAR.
    let dtstamp1 = Utc.with_ymd_and_hms(2022, 12, 24, 23, 59, 59).unwrap().naive_local();
    let dtstamp2 = Utc.with_ymd_and_hms(2023, 2, 2, 2, 2, 2).unwrap().naive_local();
    let uid = "totally_unique_identifier".to_string();
    let mut vevent1 = vcalendar.new_vevent(dtstamp1, uid);
    let mut vevent2 = vcalendar.new_vevent(dtstamp2, String::from("second uid"));

    // Set some of the VEVENTs parameters.
    let dtstart = chrono::Utc::now().naive_local();
    vevent1 = vevent1.set_dtstart(dtstart);
    vevent2 = vevent2.set_dtstart(dtstart);

    // Push the two VEVENTs onto the VCALENDAR.
    vcalendar.events.push(vevent1);
    vcalendar.events.push(vevent2);

    // Print out the VCALENDAR, which should now include the two VEVENTS.
    println!("{}", vcalendar);

    // Now remove the second event...
    vcalendar.events.pop();

    // ...and add the remaining fields to the first event.
    vevent1 = vcalendar.events.pop().unwrap();
    vevent1 = vevent1.set_class("public");
    vevent1 = vevent1.set_description("This is an example description");

    let dtstart = Utc.with_ymd_and_hms(2022, 12, 31, 12, 30, 00).unwrap().naive_local();
    let dtend = Utc.with_ymd_and_hms(2023, 1, 1, 2, 30, 00).unwrap().naive_local();

    vevent1 = vevent1.set_dtstart(dtstart);
    vevent1 = vevent1.set_dtend(dtend);

    vevent1 = vevent1.set_geo("37.386013;-122.082932");
    vevent1 = vevent1.set_organizer("Someone important");
    vevent1 = vevent1.set_priority(1);
    vevent1 = vevent1.set_summary("Morning coffee");

    vevent1 = vevent1.set_location("Starbucks");
    vevent1 = vevent1.set_url("http://github.com/cyberme0w/rustical");
    vevent1 = vevent1.set_status(VeventStatus::CANCELLED);
    vevent1 = vevent1.set_status(VeventStatus::TENTATIVE);
    vevent1 = vevent1.set_status(VeventStatus::CONFIRMED);
    vevent1 = vevent1.add_comment("Make sure to bring your laptop to look cool!");
    vevent1 = vevent1.set_transp(VeventTransp::TRANSPARENT);
    vevent1 = vevent1.set_transp(VeventTransp::OPAQUE);

    vcalendar.events.push(vevent1);

    println!("{}", vcalendar);
}
