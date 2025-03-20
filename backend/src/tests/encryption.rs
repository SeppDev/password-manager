#[cfg(test)]
mod tests {
    // use bcrypt::DEFAULT_COST;

    // const PASSWORDS: [&str; 3] = ["password123", "Ilovepancakes!", "great-password"];

    // fn test_password(password: &str) {
    //     let hash = bcrypt::hash(password, DEFAULT_COST).expect("Failed to hash password");
    //     let result = bcrypt::verify(password, &hash).expect("Failed to verify hash");

    //     assert!(result, "Verification failed")
    // }

    // fn test_mismatch_password(password: &str, wrong_password: &str) {
    //     assert!(password != wrong_password, "Passwords cannot be the same");

    //     let hash = bcrypt::hash(password, DEFAULT_COST).expect("Failed to hash password");
    //     let result = bcrypt::verify(wrong_password, &hash).expect("Failed to verify hash");

    //     assert!(!result, "Verification succeeded")
    // }

    // #[test]
    // fn password() {
    //     PASSWORDS
    //         .iter()
    //         .for_each(|password| test_password(password));
    // }

    // #[test]
    // fn password_mismatch() {
    //     const OTHER_PASSWORDS: [&str; 3] = ["Robloxfan", "123456", "aura-fixer"];

    //     PASSWORDS
    //         .iter()
    //         .zip(OTHER_PASSWORDS)
    //         .for_each(|(a, b)| test_mismatch_password(a, b));
    // }
}
