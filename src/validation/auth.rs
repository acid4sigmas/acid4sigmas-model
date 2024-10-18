use crate::{
    models::auth::{LoginIdentifier, RegisterRequest},
    to_string_,
    utils::deserializer::CustomDeserializable,
};

use regex::Regex;


impl RegisterRequest {
    pub fn validate(&self) -> Result<(), String> {
        Self::validate_username(&self.username)?;
        Self::validate_email(&self.email)?;
        Self::validate_password(&self.password)?;
        Ok(())
    }

    fn validate_username(username: &str) -> Result<(), String> {
        let username_regex = Regex::new(r"[a-zA-Z0-9-_]+$").unwrap();

        if !username_regex.is_match(username) {
            return Err(to_string_!(
                "only the following characters are allowed: a-z, A-Z, 0-9, _-"
            ));
        } else if !username.is_ascii() {
            return Err(to_string_!("only non-ascii characters are allowed."));
        }

        let username_len = username.len();

        if username_len < 3 {
            return Err(to_string_!("Username must be at least 3 characters long."));
        } else if username_len > 30 {
            return Err(to_string_!("Username cant be longer than 30 characters"));
        }

        Ok(())
    }

    fn validate_password(password: &str) -> Result<(), String> {
        let pw_len = password.len();

        if pw_len < 8 {
            return Err(to_string_!(
                "Your password must be at least 8 characters long!"
            ));
        } else if pw_len > 64 {
            return Err(to_string_!("your password is too long, max: 64 characters"));
        }

        let digit_regex = Regex::new(r"\d").unwrap();
        let uppercase_regex = Regex::new(r"[A-Z]").unwrap();
        let lowercase_regex = Regex::new(r"[a-z]").unwrap();
        let special_char_regex = Regex::new(r"[!@#$%^&*()\-=+?]").unwrap();

        if !digit_regex.is_match(password) {
            return Err(to_string_!("password must contain at least one digit."));
        }

        if !uppercase_regex.is_match(password) {
            return Err(to_string_!(
                "password must contain at least one uppercase letter."
            ));
        }

        if !lowercase_regex.is_match(password) {
            return Err(to_string_!(
                "password must contain at least one lowercase letter."
            ));
        }

        if !special_char_regex.is_match(password) {
            return Err(to_string_!("password must contain at least one special character. allowed special characters: !@#$%^&*()-_=+?"));
        }

        Ok(())
    }

    fn validate_email(email: &str) -> Result<(), String> {
        let max_local_part_length: usize = 64;
        let max_domain_length: usize = 255;

        let parts: Vec<String> = email.split('@').map(String::from).collect();

        if parts.len() != 2 {
            return Err(to_string_!("email does not contain an @"));
        } else {
            let local_part = parts[0].clone();

            let domain_part = parts[1].clone();
            if local_part.len() > max_local_part_length {
                return Err(to_string_!(
                    "the part in the email before the @ is too long"
                ));
            } else if domain_part.len() > max_domain_length {
                return Err(to_string_!("the domain part the email is too long"));
            }

            let regex = Regex::new(r"^[a-zA-Z0-9]+(?:\.[a-zA-Z0-9]+)*$").unwrap();

            if !regex.is_match(&local_part) {
                return Err(to_string_!(
                    "email in wrong format. example of an correct email: 'john.doe@example.com'"
                ));
            }
            if !regex.is_match(&domain_part) {
                return Err(to_string_!(
                    "email in wrong format. example of an correct email: 'john.doe@example.com"
                ));
            }

            return Ok(());
        }
    }
}

impl CustomDeserializable for LoginIdentifier {
    fn from_str(input: &str) -> Result<Self, String> {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

        if !input.is_ascii() {
            return Err(to_string_!("username is not in ascii"));
        }

        if email_regex.is_match(input) {
            Ok(LoginIdentifier::Email(to_string_!(input)))
        } else {
            Ok(LoginIdentifier::Username(to_string_!(input)))
        }
    }
}
