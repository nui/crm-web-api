use std::convert::Infallible;
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

use tokidator::token::ToTokenStr;

pub struct BearerToken(Option<String>);

impl FromStr for BearerToken {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BearerToken(Some(s.to_string())))
    }
}

impl ToTokenStr for BearerToken {
    fn to_token_str(&self) -> Option<&str> {
        const PATTERN: &str = r"^Bearer (\S+)$";
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(PATTERN).unwrap());
        self.0
            .as_deref()
            .and_then(|s| RE.captures(s))
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_token_str() {
        let token = BearerToken(Some("Bearer 123.456".to_owned()));
        assert_eq!(token.to_token_str(), Some("123.456"));

        let token = BearerToken(Some("123.456".to_owned()));
        assert_eq!(token.to_token_str(), None);

        let token = BearerToken(Some("Bearer".to_owned()));
        assert_eq!(token.to_token_str(), None);

        let some_valid_token = "CAEQ4a2mzo4uGOGtps6OLiDhioLQji4qAmAw.-jurx0SU22tB0hTmpbmINOequ5-0dcXUJZga2gR5jp0G1eNvd9UozTVbYh5baPiLkGrZUeByncm4dvDxe3YeAA";
        let token = BearerToken(Some(format!("Bearer {}", some_valid_token)));
        assert_eq!(token.to_token_str(), Some(some_valid_token));

        // all base64 characters include dot should be extractable
        let ascii_lower = "abcdefghijklmnopqrstuvwxyz";
        let ascii_upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let digits = "0123456789";
        let base64_chars = format!(r"{}{}{}+/-_=", ascii_lower, ascii_upper, digits);
        let token_chars = format!(r"{}.", base64_chars);
        let token = BearerToken(Some(format!("Bearer {}", token_chars)));
        assert_eq!(token.to_token_str(), Some(token_chars.as_str()))
    }
}
