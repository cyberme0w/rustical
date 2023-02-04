//! Provides structures and functions to interact with calendar-related
//! objects, such as VCALENDARs and VEVENTs.

use datetime;
use std::fmt;

pub enum Vcomponent {
    Vevent,
    // TODO: Vjournal,
    // TODO: Vtodo,
    // TODO: Valarm,
}

#[derive(Debug, PartialEq)]
pub enum VeventClass {
    PUBLIC,
    PRIVATE,
    CONFIDENTIAL,
}

pub struct Vevent {
    // REQUIRED, but MUST NOT occur more than once
    dtstamp: String,
    uid: String,
    // REQUIRED, if the parent Vcalendar does not contain the "METHOD" property
    dtstart: Option<String>,
    // OPTIONAL, but MUST NOT occur more than once
    class: Option<VeventClass>,
    class_identifier: Option<String>,
    created: Option<datetime::Instant>,
    description: Option<String>,
    geo: Option<String>,
    last_mod: Option<String>,
    location: Option<String>,
    organizer: Option<String>,
    priority: Option<String>,
    seq: Option<String>,
    status: Option<String>,
    summary: Option<String>,
    transp: Option<String>,
    url: Option<String>,
    recurid: Option<String>,
    // OPTIONAL, but SHOULD NOT occur more than once
    rrule: Vec<String>,
    // Either 'dtend' or 'duration' MAY appear once in a 'VEVENT',
    // but 'dtend' and 'duration' MUST NOT occur in the same 'VEVENT'
    dtend: Option<datetime::Instant>,
    duration: Option<String>,
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
    // TODO: OPTIONAL, and MAY occur omore than once
    // alarms: Vec<Valarm>,
}

impl Vevent {
    pub(crate) fn set_dtstart(mut self, arg: &str) -> Vevent {
        // TODO: Validate input!!!
        self.dtstart = Some(String::from(arg));
        self
    }

    pub(crate) fn set_class(mut self, arg: &str) -> Vevent {
        let upcase_arg = arg.to_ascii_uppercase();

        match upcase_arg.as_str() {
            "PUBLIC" => {
                self.class = Some(VeventClass::PUBLIC);
                self.class_identifier = None;
            }
            "PRIVATE" => {
                self.class = Some(VeventClass::PRIVATE);
                self.class_identifier = None;
            }
            "CONFIDENTIAL" => {
                self.class = Some(VeventClass::CONFIDENTIAL);
                self.class_identifier = None;
            }
            _ => {
                self.class = Some(VeventClass::PRIVATE);
                self.class_identifier = Some(String::from(upcase_arg));
            }
        }

        self
    }

    pub(crate) fn set_created(mut self, arg: datetime::Instant) -> Vevent {
        // TODO: Validate input!!!
        self.created = Some(arg);
        self
    }
}

impl fmt::Display for Vevent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("BEGIN:VEVENT\r\n");
        s = s + "DTSTAMP:" + &self.dtstamp + "\r\n";
        s = s + "UID:" + &self.uid + "\r\n";

        match &self.dtstart {
            Some(value) => {
                s = s + "DTSTART:" + value + "\r\n";
            }
            None => {}
        }

        s = s + "END:VEVENT\r\n";
        write!(f, "{}", s)
    }
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
    pub calscale: Option<String>,
    pub method: Option<String>,
    // OPTIONAL, and MAY occur more than once
    pub x_prop: Vec<String>,
    pub iana_prop: Vec<String>,
    // Components (may be of any type included in the Vcomponent enum)
    // TODO: Replace events by components, or add more vectors for components such as VFREEBUSY, VJOURNAL, VTODO, etc
    pub events: Vec<Vevent>,
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
            calscale: None,
            method: None,
            // OPTIONAL, and MAY occur more than once
            x_prop: Vec::<String>::new(),
            iana_prop: Vec::<String>::new(),
            // Components (may be of any type included in the Vcomponent enum)
            events: Vec::<Vevent>::new(),
        }
    }

    /// Generate a new VEVENT object relative to the parent VCALENDAR, as the parent defines
    /// the context in which the VEVENT object is created.
    pub fn new_vevent(&self, dtstamp: String, uid: String) -> Vevent {
        Vevent {
            // REQUIRED, but MUST NOT occur more than once
            dtstamp: dtstamp.clone(),
            uid: uid.clone(),
            // REQUIRED, if the parent Vcalendar does not contain the "METHOD" property
            dtstart: Some("TODO".to_string()),
            // OPTIONAL, but MUST NOT occur more than once
            class: None,
            class_identifier: None,
            created: None,
            description: None,
            geo: None,
            last_mod: None,
            location: None,
            organizer: None,
            priority: None,
            seq: None,
            status: None,
            summary: None,
            transp: None,
            url: None,
            recurid: None,
            // OPTIONAL, but SHOULD NOT occur more than once
            rrule: Vec::<String>::new(),
            // Either 'dtend' or 'duration' MAY appear once in a 'VEVENT',
            // but 'dtend' and 'duration' MUST NOT occur in the same 'VEVENT'
            dtend: None,
            duration: None,
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
            // TODO: OPTIONAL, and MAY occur omore than once
            // alarms: Vec::<Valarm>::new(),
        }
    }
}

impl fmt::Display for Vcalendar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("BEGIN:VCALENDAR\r\n");
        s = s + "VERSION:" + &self.version + "\r\n";
        s = s + "PRODID:" + &self.prodid + "\r\n";

        self.events.iter().for_each(|event| {
            s = format!("{}{}", s, event);
        });

        let s = s + "END:VCALENDAR\r\n";
        write!(f, "{}", s)
    }
}

#[test]
fn test_new_vcalendar() {
    let cal = Vcalendar::new_vcalendar(String::from("prodid"), String::from("version"));
    assert_eq!(cal.prodid, "prodid");
    assert_eq!(cal.version, "version");
    assert_eq!(cal.calscale, None);
    assert_eq!(cal.method, None);
    assert_eq!(cal.x_prop, Vec::<String>::new());
    assert_eq!(cal.iana_prop, Vec::<String>::new());
}

#[test]
fn test_new_vevent() {
    let prodid = "prodid".to_string();
    let version = "version".to_string();
    let cal = Vcalendar::new_vcalendar(prodid, version);

    let dtstamp = "dtstamp".to_string();
    let uid = "uid".to_string();
    let ev = cal.new_vevent(dtstamp, uid);

    assert_eq!(ev.dtstamp, "dtstamp");
    assert_eq!(ev.uid, "uid");
}

#[test]
fn test_vevent_class_setter() {
    let prodid = "prodid".to_string();
    let version = "2.0".to_string();
    let cal = Vcalendar::new_vcalendar(prodid, version);

    let dtstamp = "dtstamp".to_string();
    let uid = "uid".to_string();
    let mut ev = cal.new_vevent(dtstamp, uid);

    assert!(ev.class.is_none());
    assert!(ev.class_identifier.is_none());

    ev = ev.set_class("public");
    assert_eq!(ev.class, Some(VeventClass::PUBLIC));
    assert_eq!(ev.class_identifier, None);

    ev = ev.set_class("private");
    assert_eq!(ev.class, Some(VeventClass::PRIVATE));
    assert_eq!(ev.class_identifier, None);

    ev = ev.set_class("confidential");
    assert_eq!(ev.class, Some(VeventClass::CONFIDENTIAL));
    assert_eq!(ev.class_identifier, None);

    ev = ev.set_class("something else");
    assert_eq!(ev.class, Some(VeventClass::PRIVATE));
    assert_eq!(ev.class_identifier, Some("SOMETHING ELSE".to_string()));

    ev = ev.set_class("public");
    assert_eq!(ev.class, Some(VeventClass::PUBLIC));
    assert_eq!(ev.class_identifier, None);
}

#[test]
fn test_vevent_created_setter() {
    let prodid = "prodid".to_string();
    let version = "2.0".to_string();
    let cal = Vcalendar::new_vcalendar(prodid, version);

    let dtstamp = "dtstamp".to_string();
    let uid = "uid".to_string();
    let mut ev = cal.new_vevent(dtstamp, uid);

    assert!(ev.created.is_none());

    let dt = datetime::Instant::now();
    ev = ev.set_created(dt);

    assert!(ev.created == Some(dt));
}