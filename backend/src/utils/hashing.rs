pub fn hash_password(password: &str) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}

pub fn verify_password(hashed_password: &str, password: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hashed_password)
}
