//! Provides structures and functions to interact with calendar-related
//! objects, such as VCALENDARs and VEVENTs.


/// A VEVENT represented as a named-field struct.
pub struct Vevent {
    pub veventid: String,
}

/// A VCALENDAR represented as a named-field struct.
pub struct Vcalendar {
    pub prodid: String,
    pub version: String,
    pub description: String,
    pub events: Vec<Vevent>,
}

impl Vcalendar {
    /// Generate a new empty VCALENDAR object with the mandatory PRODID
    /// and VERSION fields.
    pub fn new(prodid: String, version: String) -> Vcalendar {
        Vcalendar {
            prodid: prodid.clone(),
            version: version.clone(),
            description: String::from(""),
            events: Vec::<Vevent>::new(),
        }
    }

    /// Print the contents of VCALENDAR to stdout.
    pub fn print(self) {
        println!("<Vcalendar:\n  Prodid: {}\n  Version: {}\n  Description: {}\n>",
                 self.prodid, self.version, self.description);
    }
}

#[test]
fn test_new_vcalendar() {
    let v = Vcalendar::new(String::from("prodid"), String::from("version"));
    assert_eq!(v.prodid, "prodid");
    assert_eq!(v.version, "version");
}
