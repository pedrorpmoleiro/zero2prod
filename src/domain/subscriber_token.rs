use rand::Rng;
use rand::distr::Alphanumeric;

#[derive(Debug)]
pub struct SubscriberToken(String);

impl SubscriberToken {
    pub fn generate() -> Self {
        let mut rng = rand::rng();
        let subscriber_token: String = std::iter::repeat_with(|| rng.sample(Alphanumeric))
            .map(char::from)
            .take(25)
            .collect();

        Self(subscriber_token)
    }

    pub fn parse(s: String) -> Result<Self, String> {
        let input = s.trim();
        let is_empty_or_whitespace = input.is_empty();
        let is_25_chars = input.chars().count() == 25;
        let is_alphanumeric = input.chars().all(char::is_alphanumeric);

        if is_empty_or_whitespace || !is_alphanumeric || !is_25_chars {
            Err(format!("{} is not a valid subscriber token", input))
        } else {
            Ok(Self(input.into()))
        }
    }
}

impl AsRef<str> for SubscriberToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberToken;
    use claims::{assert_err, assert_ok};

    #[test]
    fn whitespace_only_tokens_are_rejected() {
        let token = " ".to_string();
        assert_err!(SubscriberToken::parse(token));
    }

    #[test]
    fn empty_string_is_rejected() {
        let token = "".to_string();
        assert_err!(SubscriberToken::parse(token));
    }

    #[test]
    fn token_with_more_than_25_chars_is_rejected() {
        let token = "a".repeat(26);
        assert_err!(SubscriberToken::parse(token));
    }

    #[test]
    fn token_with_less_than_25_chars_is_rejected() {
        let token = "a".repeat(24);
        assert_err!(SubscriberToken::parse(token));
    }

    #[test]
    fn token_with_non_alphanumeric_chars_is_rejected() {
        let token = format!("{}*?;-_", "a".repeat(20));
        assert_err!(SubscriberToken::parse(token));
    }

    #[test]
    fn a_valid_token_is_parsed_successfully() {
        let token = "a".repeat(25);
        assert_ok!(SubscriberToken::parse(token));
    }

    #[test]
    fn a_valid_token_with_whitespace_in_the_end_is_parsed_successfully() {
        let token = format!("{} ", "a".repeat(25));
        assert_ok!(SubscriberToken::parse(token));
    }
}
