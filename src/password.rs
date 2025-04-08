/*
 * Copyright (c) Portalnesia - All Rights Reserved
 * Unauthorized copying of this file, via any medium is strictly prohibited
 * Proprietary and confidential
 * Written by Putu Aditya <aditya@portalnesia.com>
 */

use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};

/// Hashes the password using bcrypt
///
/// ## Example
///
/// ```
/// let hashed = pn_utils::password::hash_password("this is secret password".to_string());
/// println!("{}",hashed.unwrap_or("failed".to_string()));
/// ```
pub fn hash_password(password: String) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}

/// Compares a password with a hashed password
///
/// ## Example
///
/// ```
/// let password = "this is secret password";
/// let hashed_password = "$2b$12$jfIoU3eWvkujdHnwpDf01ek0zKHNxOxVn7ifbEMV4eIPE.j7ZWBR."; // This is only examples
/// let is_password_valid = pn_utils::password::compare_password(password.to_string(),hashed_password.to_string());
/// println!("is password valid? {}",is_password_valid);
/// ```
pub fn compare_password(password: String, hashed_password: String) -> bool {
    verify(password, hashed_password.as_str()).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_compare_password() {
        let password = "my_secret_password";
        let hashed_password =
            hash_password(password.to_string()).expect("failed to hashed password");

        // Ensure the password matches the hash
        assert!(compare_password(
            password.to_string(),
            hashed_password.clone()
        ));

        // Ensure a different password does not match
        assert!(!compare_password(
            "wrong_password".to_string(),
            hashed_password.clone()
        ));
    }
}
