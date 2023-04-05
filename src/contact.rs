use std::fmt::Display;

use crate::email::Email;

#[derive(Debug, Clone)]
pub struct Contact {
    pub name: String,
    pub email: Email,
}

impl Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", &self.name, self.email.as_ref())
    }
}
