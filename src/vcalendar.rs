//! Provides structures and functions to interact with calendar-related
//! objects, such as VCALENDARs and VEVENTs.

use chrono::{self, NaiveDateTime, Utc, TimeZone};
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

#[derive(Default)]
pub struct Vevent {
    // REQUIRED, but MUST NOT occur more than once
    dtstamp: NaiveDateTime,
    uid: String,
    // REQUIRED, if the parent Vcalendar does not contain the "METHOD" property
    dtstart: Option<NaiveDateTime>,
    // OPTIONAL, but MUST NOT occur more than once
    class: Option<VeventClass>,
    class_identifier: Option<String>,
    created: Option<NaiveDateTime>,
    description: Option<String>,
    geo: Option<String>,
    last_mod: Option<String>,
    location: Option<String>,
    organizer: Option<String>,
    // Really, all we need is a 0-9 value
    priority: Option<u8>,
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
    dtend: Option<NaiveDateTime>,
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
    pub(crate) fn set_dtstart(mut self, arg: NaiveDateTime) -> Vevent {
        // TODO: Validate input!!!
        self.dtstart = Some(arg);
        self
    }
    pub(crate) fn set_dtend(mut self, arg: NaiveDateTime) -> Vevent {
        // TODO: Validate input!!!
        self.duration = None;
        self.dtend = Some(arg);
        self
    }
    pub(crate) fn set_duration(mut self, arg: String) -> Vevent {
        // TODO: Validate input!!!
        self.duration = Some(arg);
        self.dtend = None;
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
    pub(crate) fn set_created(mut self, arg: NaiveDateTime) -> Vevent {
        // TODO: Validate input!!!
        self.created = Some(arg);
        self
    }
    pub(crate) fn set_class_identifier(mut self, arg: &str) -> Vevent {
        // TODO: Validation
        self.class_identifier = Some(String::from(arg));
        self
    }
    pub(crate) fn set_description(mut self, arg: &str) -> Vevent {
        // TODO: Validation
        self.description = Some(String::from(arg));
        self
    }
    pub(crate) fn set_geo(mut self, arg: &str) -> Vevent {
        // TODO: Validation
        self.geo = Some(String::from(arg));
        self
    }
    pub(crate) fn set_last_mod(mut self, arg: &str) -> Vevent {
        // TODO: Validation
        self.last_mod = Some(String::from(arg));
        self
    }
    pub(crate) fn set_location(mut self, arg: &str) -> Vevent {
        // TODO: Validation
        self.location = Some(String::from(arg));
        self
    }
    pub(crate) fn set_organizer(mut self, arg: &str) -> Vevent {
        // TODO: Validation
        self.organizer = Some(String::from(arg));
        self
    }
    pub(crate) fn set_priority(mut self, arg: u8) -> Vevent {
        if arg < 10 { self.priority = Some(arg); } else { panic!("PRIORITY must be a u8 between 0 and 9 inclusive"); }
        self
    }
    pub(crate) fn set_seq(mut self, arg: &str) -> Vevent {
        // TODO: Validation
        self.seq = Some(String::from(arg));
        self
    }
    pub(crate) fn set_status(mut self, arg: &str) -> Vevent {
        // TODO: Validation
        self.status = Some(String::from(arg));
        self
    }
    pub(crate) fn set_summary(mut self, arg: &str) -> Vevent {
        // TODO: Validation
        self.summary = Some(String::from(arg));
        self
    }
    pub(crate) fn set_transp(mut self, arg: &str) -> Vevent {
        // TODO: Validation
        self.transp = Some(String::from(arg));
        self
    }
    pub(crate) fn set_url(mut self, arg: &str) -> Vevent {
        // TODO: Validation
        self.url = Some(String::from(arg));
        self
    }
    pub(crate) fn set_recurid(mut self, arg: &str) -> Vevent {
        // TODO: Validation
        self.recurid = Some(String::from(arg));
        self
    }
}

impl fmt::Display for Vevent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("BEGIN:VEVENT\r\n");
        let dtstamp = self.dtstamp.format("%Y%m%dT%H%M%SZ").to_string();
        s = s + "DTSTAMP:" + dtstamp.as_str() + "\r\n";
        s = s + "UID:" + &self.uid + "\r\n";

        match &self.dtstart {
            Some(value) => {
                s = s + "DTSTART:" + value.format("%Y%m%dT%H%M%SZ").to_string().as_str() + "\r\n";
            }
            None => {}
        }
        match &self.class_identifier {
            Some(value) => { s = s + "CLASS:" + value + "\r\n"; }
            None => match &self.class {
                Some(value) => match value {
                    VeventClass::PUBLIC => s = s + "CLASS:PUBLIC\r\n",
                    VeventClass::PRIVATE => s = s + "CLASS:PRIVATE\r\n",
                    VeventClass::CONFIDENTIAL => s = s + "CLASS:CONFIDENTIAL\r\n",
                },
                None => {}
            },
        }
        match &self.created {
            Some(value) => {
                s = s + "CREATED:" + value.format("%Y%m%dT%H%M%SZ").to_string().as_str();
            }
            None => {}
        }
        match &self.description {
            Some(value) => { s = s + "DESCRIPTION:" + value + "\r\n"; }
            None => {}
        }
        match &self.geo {
            Some(value) => { s = s + "GEO:" + value + "\r\n"; }
            None => {}
        }
        match &self.last_mod {
            Some(value) => { s = s + "LAST-MODIFIED:" + value + "\r\n"; }
            None => {}
        }
        match &self.location {
            Some(value) => { s = s + "LOCATION:" + value + "\r\n"; }
            None => {}
        }
        match &self.organizer {
            Some(value) => { s = s + "ORGANIZER:" + value + "\r\n"; }
            None => {}
        }
        match &self.priority {
            Some(value) => { s = s + "PRIORITY:" + value.to_string().as_str() + "\r\n"; }
            None => {}
        }
        match &self.seq {
            Some(value) => { s = s + "SEQUENCE:" + value + "\r\n"; }
            None => {}
        }
        match &self.status {
            Some(value) => { s = s + "STATUS:" + value + "\r\n"; }
            None => {}
        }
        match &self.summary {
            Some(value) => { s = s + "SUMMARY:" + value + "\r\n"; }
            None => {}
        }
        match &self.transp {
            Some(value) => { s = s + "TRANSP:" + value + "\r\n"; }
            None => {}
        }
        match &self.url {
            Some(value) => { s = s + "URL:" + value + "\r\n"; }
            None => {}
        }
        match &self.recurid {
            Some(value) => { s = s + "RECURRENCE-ID:" + value + "\r\n"; }
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
#[derive(Default)]
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
            prodid: prodid.clone(),
            version: version.clone(),
            ..Default::default()
        }
    }

    /// Generate a new VEVENT object relative to the parent VCALENDAR, as the parent defines
    /// the context in which the VEVENT object is created.
    pub fn new_vevent(&self, dtstamp: NaiveDateTime, uid: String) -> Vevent {
        Vevent {
            dtstamp: dtstamp.clone(),
            uid: uid.clone(),
            ..Default::default()
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
    let version = "2.0".to_string();
    let cal = Vcalendar::new_vcalendar(prodid, version);

    let dtstamp = Utc.with_ymd_and_hms(2022, 12, 24, 0, 0, 0).unwrap().naive_local();
    let uid = "uid".to_string();
    let ev = cal.new_vevent(dtstamp, uid);

    assert_eq!(ev.dtstamp, dtstamp.clone());
    assert_eq!(ev.uid, "uid");
}

#[test]
fn test_vevent_class_setter() {
    let prodid = "prodid".to_string();
    let version = "2.0".to_string();
    let cal = Vcalendar::new_vcalendar(prodid, version);

    let dtstamp = Utc.with_ymd_and_hms(2022, 12, 24, 0, 0, 0).unwrap().naive_local();
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

    let dtstamp = Utc::now().naive_local();
    let uid = "uid".to_string();
    let mut ev = cal.new_vevent(dtstamp, uid);

    assert!(ev.created.is_none());

    let dt = Utc::now().naive_local();
    ev = ev.set_created(dt);

    assert!(ev.created == Some(dt));
}
