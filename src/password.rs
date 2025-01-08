use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};

// HashPassword hashes the password using bcrypt
pub fn hash_password(password: String) -> Result<String,BcryptError> {
    hash(password, DEFAULT_COST)
}

// ComparePassword compares a password with a hashed password
pub fn compare_password(password: String, hashed_password: String) -> bool {
    verify(password, hashed_password.as_str()).unwrap_or_else(|_| false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_compare_password() {
        let password = "my_secret_password";
        let hashed_password = hash_password(password.to_string()).expect("failed to hashed password");

        // Ensure the password matches the hash
        assert!(compare_password(password.to_string(), hashed_password.clone()));

        // Ensure a different password does not match
        assert!(!compare_password("wrong_password".to_string(), hashed_password.clone()));
    }
}