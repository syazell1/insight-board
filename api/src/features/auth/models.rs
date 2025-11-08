use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Serialize)]
pub struct AuthResponse<'a> {
    pub id: Uuid,
    pub access_token: &'a str,
}
#[derive(Deserialize, Validate)]
pub struct LoginFormData {
    #[validate(custom(function = "validate_fields"))]
    pub username: String,
    #[validate(custom(function = "validate_fields"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RegisterFormData {
    #[validate(custom(function = "validate_fields"))]
    pub username: String,
    #[validate(custom(function = "validate_fields"))]
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::{LoginFormData, RegisterFormData};
    use claims::{assert_err, assert_ok};
    use validator::Validate;

    #[test]
    fn validate_login_form_data_succeeds_with_basic_fields() {
        let form = LoginFormData {
            username: "alice".to_string(),
            password: "secret".to_string(),
        };

        let res = form.validate();

        assert_ok!(res);
    }

    #[test]
    fn validate_login_form_data_fails_with_empty_username() {
        let form = LoginFormData {
            username: "".to_string(),
            password: "secret".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_login_form_data_fails_with_empty_password() {
        let form = LoginFormData {
            username: "alice".to_string(),
            password: "".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_login_form_data_fails_with_both_empty_fields() {
        let form = LoginFormData {
            username: "".to_string(),
            password: "".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_login_form_data_succeeds_with_max_length_username() {
        let form = LoginFormData {
            username: "a".repeat(12),
            password: "secret".to_string(),
        };

        let res = form.validate();

        assert_ok!(res);
    }

    #[test]
    fn validate_login_form_data_succeeds_with_max_length_password() {
        let form = LoginFormData {
            username: "alice".to_string(),
            password: "a".repeat(12),
        };

        let res = form.validate();

        assert_ok!(res);
    }

    #[test]
    fn validate_login_form_data_fails_with_username_exceeding_max_length() {
        let form = LoginFormData {
            username: "a".repeat(13),
            password: "secret".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_login_form_data_fails_with_password_exceeding_max_length() {
        let form = LoginFormData {
            username: "alice".to_string(),
            password: "a".repeat(13),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_login_form_data_fails_with_username_containing_exclamation() {
        let form = LoginFormData {
            username: "alice!".to_string(),
            password: "secret".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_login_form_data_fails_with_username_containing_at() {
        let form = LoginFormData {
            username: "alice@".to_string(),
            password: "secret".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_login_form_data_fails_with_username_containing_dollar() {
        let form = LoginFormData {
            username: "alice$".to_string(),
            password: "secret".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_login_form_data_fails_with_username_containing_hash() {
        let form = LoginFormData {
            username: "alice#".to_string(),
            password: "secret".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_login_form_data_fails_with_password_containing_exclamation() {
        let form = LoginFormData {
            username: "alice".to_string(),
            password: "secret!".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_login_form_data_fails_with_password_containing_at() {
        let form = LoginFormData {
            username: "alice".to_string(),
            password: "secret@".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_login_form_data_fails_with_password_containing_dollar() {
        let form = LoginFormData {
            username: "alice".to_string(),
            password: "secret$".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_login_form_data_fails_with_password_containing_hash() {
        let form = LoginFormData {
            username: "alice".to_string(),
            password: "secret#".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_login_form_data_succeeds_with_single_character_fields() {
        let form = LoginFormData {
            username: "a".to_string(),
            password: "b".to_string(),
        };

        let res = form.validate();

        assert_ok!(res);
    }

    #[test]
    fn validate_login_form_data_succeeds_with_alphanumeric_fields() {
        let form = LoginFormData {
            username: "user123".to_string(),
            password: "pass456".to_string(),
        };

        let res = form.validate();

        assert_ok!(res);
    }

    #[test]
    fn validate_login_form_data_succeeds_with_underscores_and_dashes() {
        let form = LoginFormData {
            username: "user_name".to_string(),
            password: "pass-word".to_string(),
        };

        let res = form.validate();

        assert_ok!(res);
    }

    #[test]
    fn validate_register_form_data_succeeds_with_basic_fields() {
        let form = RegisterFormData {
            username: "alice".to_string(),
            password: "secret".to_string(),
        };

        let res = form.validate();

        assert_ok!(res);
    }

    #[test]
    fn validate_register_form_data_succeeds_with_empty_username() {
        let form = RegisterFormData {
            username: "".to_string(),
            password: "secret".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_register_form_data_succeeds_with_empty_password() {
        let form = RegisterFormData {
            username: "alice".to_string(),
            password: "".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_register_form_data_succeeds_with_both_empty_fields() {
        let form = RegisterFormData {
            username: "".to_string(),
            password: "".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_register_form_data_succeeds_with_long_username() {
        let form = RegisterFormData {
            username: "a".repeat(100),
            password: "secret".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_register_form_data_succeeds_with_long_password() {
        let form = RegisterFormData {
            username: "alice".to_string(),
            password: "a".repeat(100),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_register_form_data_succeeds_with_username_containing_special_chars() {
        let form = RegisterFormData {
            username: "alice!@#$%^&*()".to_string(),
            password: "secret".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_register_form_data_succeeds_with_password_containing_special_chars() {
        let form = RegisterFormData {
            username: "alice".to_string(),
            password: "secret!@#$%^&*()".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_register_form_data_succeeds_with_single_character_fields() {
        let form = RegisterFormData {
            username: "a".to_string(),
            password: "b".to_string(),
        };

        let res = form.validate();

        assert_ok!(res);
    }

    #[test]
    fn validate_register_form_data_succeeds_with_alphanumeric_fields() {
        let form = RegisterFormData {
            username: "user123".to_string(),
            password: "pass456".to_string(),
        };

        let res = form.validate();

        assert_ok!(res);
    }

    #[test]
    fn validate_register_form_data_succeeds_with_underscores_and_dashes() {
        let form = RegisterFormData {
            username: "user_name".to_string(),
            password: "pass-word".to_string(),
        };

        let res = form.validate();

        assert_ok!(res);
    }

    #[test]
    fn validate_register_form_data_succeeds_with_unicode_characters() {
        let form = RegisterFormData {
            username: "用户123".to_string(),
            password: "пароль456".to_string(),
        };

        let res = form.validate();

        assert_err!(res);
    }

    #[test]
    fn validate_register_form_data_succeeds_with_whitespace() {
        let form = RegisterFormData {
            username: "  alice  ".to_string(),
            password: "  secret  ".to_string(),
        };

        let res = form.validate();

        assert_ok!(res);
    }
}

fn validate_fields(value: &str) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(ValidationError::new("empty_field")
            .with_message(std::borrow::Cow::Borrowed("empty field")));
    }

    if value.len() > 12 {
        return Err(ValidationError::new("too_many_chars")
            .with_message(std::borrow::Cow::Borrowed("long characters")));
    }

    let a = vec!['!', '@', '$', '#'];

    for c in a.iter() {
        if value.contains(c.clone()) {
            return Err(ValidationError::new("too_many_chars")
                .with_message(std::borrow::Cow::Borrowed("long characters")));
        }
    }

    Ok(())
}
