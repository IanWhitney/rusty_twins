use std::fmt;
pub struct Attendee {
    pub id: String,
    pub name: String,
}

impl fmt::Display for Attendee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.name)
    }
}

impl Attendee {
    pub fn all() -> Vec<Attendee> {
        vec![
            Attendee{ id: String::from("0"), name: String::from("Unpicked")},
            Attendee{ id: String::from("1"), name: String::from("Ian and Jon")},
            Attendee{ id: String::from("2"), name: String::from("Davin and Michelle")},
            Attendee{ id: String::from("3"), name: String::from("Beth")},
        ]
    }

    pub fn with_id(attendee_id: &String) -> Option<Attendee> {
        Attendee::all()
            .into_iter()
            .find(|g| g.id == *attendee_id)
    }
}


