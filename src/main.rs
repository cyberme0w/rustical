//! `rustical` provides a way to interact with vcalendar (.ics) files.

mod vcalendar;
use vcalendar::*;

fn main() {
    let prodid = "prodid".to_string();
    let version = "2.0".to_string();
    let mut vcalendar = Vcalendar::new_vcalendar(prodid, version);

    let dtstamp = "123123".to_string();
    let uid = "totally_unique_identifier".to_string();

    let mut vevent1 = vcalendar.new_vevent(dtstamp, uid);
    let mut vevent2 = vcalendar.new_vevent("second dtstamp".to_owned(), "second uid".to_owned());

    vevent1 = vevent1.set_dtstart("some_dtstart");
    vevent2 = vevent2.set_dtstart("some_other_dtstart");

    vcalendar.events.push(vevent1);
    vcalendar.events.push(vevent2);

    println!("{}", vcalendar);
    // println!("vevent: \n{}", event);

    //v.print_vcalendar();

    //generate_rustical_data_dir();
}

#[test]
pub fn test_vcalendar_setters() {
    assert_eq!(true, true);
}
#[test]
pub fn test_vevent_setters() {}
