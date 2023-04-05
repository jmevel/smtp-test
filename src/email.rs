use validator::validate_email;

#[derive(Debug, Clone)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Email, String> {
        if validate_email(s.clone()) {
            Ok(Email(s))
        } else {
            Err(format!("{s} is not a valid email"))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Email;
    use claims::assert_err;
    use fake::{faker::internet::en::SafeEmail, Fake};
    use rand::{rngs::StdRng, SeedableRng};

    // Both `Clone` and `Debug` are required by `quickcheck`
    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let email = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        dbg!(&valid_email.0);
        Email::parse(valid_email.0).is_ok()
    }

    #[test]
    fn email_with_whitespace_is_rejected() {
        assert_err!(Email::parse("toto @test.com".to_string()));
    }

    #[test]
    fn empty_string_is_rejected() {
        assert_err!(Email::parse("".to_string()));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        assert_err!(Email::parse("toto.domain.com".to_string()));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        assert_err!(Email::parse("@domain.com".to_string()));
    }

    #[test]
    fn email_missing_domain_is_rejected() {
        assert_err!(Email::parse("toto@".to_string()));
    }
}
