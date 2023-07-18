#[derive(Debug)]
pub struct SubscriberEmail(String);

fn is_valid_email(s: &str) -> bool {
    return validator::validate_email(s);
}

impl TryFrom<String> for SubscriberEmail {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if is_valid_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid email.", s))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claims::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::try_from(email));
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::try_from(email));
    }

    #[test]

    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::try_from(email));
    }
    // [...]
    #[test]
    fn valid_emails_are_parsed_successfully() {
        let email: String = SafeEmail().fake();
        assert_ok!(SubscriberEmail::try_from(email));
    }
}
