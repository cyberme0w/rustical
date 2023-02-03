mod vcalendar;

use vcalendar::*;

fn main() {
    let prodid = String::from("prodid");
    let version = String::from("version");
    let v = Vcalendar::new(prodid, version);

    println!("Generated empty calendar!");
    v.print();
}
