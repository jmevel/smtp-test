use std::fmt::Display;

use crate::email::Email;

#[derive(Debug, Clone)]
pub struct Contact<'a> {
    pub name: &'a str,
    pub email: Email<'a>,
}

impl Display for Contact<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", &self.name, self.email.as_ref())
    }
}
