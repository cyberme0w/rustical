//! Provides structures and functions to interact with calendar-related
//! objects, such as VCALENDARs and VEVENTs.

pub enum DtendDuration {
    Dtend,
    Duration,
    None,
}

pub enum Vcomponent {
    Vevent,
    Vjournal,
    Vtodo,
    Valarm,
}

pub struct Vevent {
    // REQUIRED, but MUST NOT occur more than once
    dtstamp: String,
    uid: String,
    // REQUIRED, if the parent Vcalendar does not contain the "METHOD" property
    dtstart: String,
    // OPTIONAL, but MUST NOT occur more than once
    class: String,
    created: String,
    description: String,
    geo: String,
    last_mod: String,
    location: String,
    organizer: String,
    priority: String,
    seq: String,
    status: String,
    summary: String,
    transp: String,
    url: String,
    recurid: String,
    // OPTIONAL, but SHOULD NOT occur more than once
    rrule: String,
    // Either 'dtend' or 'duration' MAY appear once in a 'VEVENT',
    // but 'dtend' and 'duration' MUST NOT occur in the same 'VEVENT'
    dtend_or_duration: DtendDuration,
    // OPTIONAL, and MAY occur more than once
    attach: Vec<String>,
    attendee: Vec<String>,
    categories: Vec<String>,
    comment: Vec<String>,
    contact: Vec<String>,
    exdate: Vec<String>,
    rstatus: Vec<String>,
    related: Vec<String>,
    resources: Vec<String>,
    rdate: Vec<String>,
    x_prop: Vec<String>,
    iana_prop: Vec<String>,
    // OPTIONAL, and MAY occur omore than once
    alarms: Vec<Valarm>,
}

struct Vjournal {
    parent: Vcomponent,
}

struct Vtodo {
    parent: Vcomponent,
}

struct Valarm {
    parent: Vcomponent,
}

/// A VCALENDAR represented as a named-field struct.
pub struct Vcalendar {
    // REQUIRED, but MUST NOT occur more than once
    pub version: String,
    pub prodid: String,
    // OPTIONAL, but MUST NOT occur more than once
    pub calscale: String,
    pub method: String,
    // OPTIONAL, and MAY occur more than once
    pub x_prop: String,
    pub iana_prop: String,
    // Components (may be of any type included in the Vcomponent enum)
    pub components: Vec<Vcomponent>,
}

impl Vcalendar {
    /// Generate a new empty VCALENDAR object with the mandatory PRODID
    /// and VERSION fields.
    pub fn new_vcalendar(prodid: String, version: String) -> Vcalendar {
        Vcalendar {
            // REQUIRED, but MUST NOT occur more than once
            prodid: prodid.clone(),
            version: version.clone(),
            // OPTIONAL, but MUST NOT occur more than once
            calscale: "TODO".to_string(),
            method: "TODO".to_string(),
            // OPTIONAL, and MAY occur more than once
            x_prop: "TODO".to_string(),
            iana_prop: "TODO".to_string(),
            // Components (may be of any type included in the Vcomponent enum)
            components: Vec::<Vcomponent>::new(),
        }
    }

    pub fn new_vevent(dtstamp: String, uid: String) -> Vevent {
        Vevent {
            // REQUIRED, but MUST NOT occur more than once
            dtstamp: dtstamp.clone(),
            uid: uid.clone(),
            // REQUIRED, if the parent Vcalendar does not contain the "METHOD" property
            dtstart: "TODO".to_string(),
            // OPTIONAL, but MUST NOT occur more than once
            class: "TODO".to_string(),
            created: "TODO".to_string(),
            description: "TODO".to_string(),
            geo: "TODO".to_string(),
            last_mod: "TODO".to_string(),
            location: "TODO".to_string(),
            organizer: "TODO".to_string(),
            priority: "TODO".to_string(),
            seq: "TODO".to_string(),
            status: "TODO".to_string(),
            summary: "TODO".to_string(),
            transp: "TODO".to_string(),
            url: "TODO".to_string(),
            recurid: "TODO".to_string(),
            // OPTIONAL, but SHOULD NOT occur more than once
            rrule: "TODO".to_string(),
            // Either 'dtend' or 'duration' MAY appear once in a 'VEVENT',
            // but 'dtend' and 'duration' MUST NOT occur in the same 'VEVENT'
            dtend_or_duration: DtendDuration::None,
            // OPTIONAL, and MAY occur more than once
            attach: Vec::<String>::new(),
            attendee: Vec::<String>::new(),
            categories: Vec::<String>::new(),
            comment: Vec::<String>::new(),
            contact: Vec::<String>::new(),
            exdate: Vec::<String>::new(),
            rstatus: Vec::<String>::new(),
            related: Vec::<String>::new(),
            resources: Vec::<String>::new(),
            rdate: Vec::<String>::new(),
            x_prop: Vec::<String>::new(),
            iana_prop: Vec::<String>::new(),
            // OPTIONAL, and MAY occur omore than once
            alarms: Vec::<Valarm>::new(),
        }
    }

    /// Print the contents of VCALENDAR to stdout.
    pub fn print_vcalendar(self) {
        println!(
            "VCALENDAR:\n\
             - prodid: {}\n\
             - version: {}\n\
             - calscale: {}\n\
             - method: {}\n\
             - x-prop: {}\n\
             - iana-prop: {}",
            self.prodid,
            self.version,
            self.calscale,
            self.method,
            self.x_prop,
            self.iana_prop,
        );
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn test_new_vcalendar() {
        let cal = Vcalendar::new_vcalendar(String::from("prodid"), String::from("version"));
        assert_eq!(cal.prodid, "prodid");
        assert_eq!(cal.version, "version");
        assert_eq!(cal.calscale, "");
        assert_eq!(cal.method, "");
        assert_eq!(cal.x_prop, "");
        assert_eq!(cal.iana_prop, "");
    }
    #[test]
    fn test_new_vevent() {
        let cal = Vcalendar::new_vcalendar(String::from("prodid"), String::from("version"));
        let ev = cal.new_vevent("dtstamp", "uid");
        assert_eq!(ev.dtstamp, "dtstamp");
        assert_eq!(ev.uid, "uid");
    }
}