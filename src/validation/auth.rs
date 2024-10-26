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
        println!("username: {}", username);

        let username_len = username.len();

        if username_len < 3 {
            return Err(to_string_!("Username must be at least 3 characters long."));
        } else if username_len > 30 {
            return Err(to_string_!("Username cant be longer than 30 characters"));
        }

        for c in username.chars() {
            if !(c.is_ascii_alphanumeric() || c == '-' || c == '_') {
                return Err(to_string_!(
                    "Only the following characters are allowed: a-z, A-Z, 0-9, _-"
                ));
            }
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

        let mut has_digit = false;
        let mut has_uppercase = false;
        let mut has_lowercase = false;
        let mut has_special_char = false;

        let special_characters: &[char] = &[
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '=', '+', '?',
        ];

        for c in password.chars() {
            if c.is_ascii_digit() {
                has_digit = true;
            } else if c.is_ascii_uppercase() {
                has_uppercase = true;
            } else if c.is_ascii_lowercase() {
                has_lowercase = true;
            } else if special_characters.contains(&c) {
                has_special_char = true;
            }

            if has_digit && has_uppercase && has_lowercase && has_special_char {
                return Ok(());
            } // exit early and stop the iteration if we have already met all con
        }

        if !has_digit {
            return Err(to_string_!("Password must contain at least one digit."));
        }
        if !has_uppercase {
            return Err(to_string_!(
                "Password must contain at least one uppercase letter."
            ));
        }
        if !has_lowercase {
            return Err(to_string_!(
                "Password must contain at least one lowercase letter."
            ));
        }
        if !has_special_char {
            return Err(to_string_!("Password must contain at least one special character. Allowed special characters: !@#$%^&*()-_=+?"));
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

            fn check_if_allowed(target: &str) -> Result<(), String> {
                for c in target.chars() {
                    if !(c.is_ascii_alphanumeric() || c == '.' || c == '-') {
                        return Err(to_string_!(
                            "Email format is wrong. Only letters, digits, dots, and hyphens are allowed."
                        ));
                    }
                }
                Ok(())
            }

            check_if_allowed(&local_part)?;
            check_if_allowed(&domain_part)?;

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
