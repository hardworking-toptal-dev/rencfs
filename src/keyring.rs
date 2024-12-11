use std::str::FromStr;

use keyring::Entry;
use shush_rs::{ExposeSecret, SecretString};

#[allow(dead_code)]
const KEYRING_SERVICE: &str = "rencfs";
#[allow(dead_code)]
const KEYRING_USER: &str = "rencfs";

#[allow(dead_code)]
pub(crate) fn save(password: &SecretString, suffix: &str) -> Result<(), keyring::Error> {
    let entry = Entry::new(KEYRING_SERVICE, &format!("{KEYRING_USER}.{suffix}"))?;
    entry.set_password(&password.expose_secret())
}

#[allow(dead_code)]
pub(crate) fn remove(suffix: &str) -> Result<(), keyring::Error> {
    let entry = Entry::new(KEYRING_SERVICE, &format!("{KEYRING_USER}.{suffix}"))?;
    entry.delete_password()
}

#[allow(dead_code)]
pub(crate) fn get(suffix: &str) -> Result<SecretString, keyring::Error> {
    let entry = Entry::new(KEYRING_SERVICE, &format!("{KEYRING_USER}.{suffix}"))?;
    Ok(SecretString::from_str(&entry.get_password()?).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save() {
        let password = SecretString::from_str("password").unwrap();
        assert!(save(&password, "test").is_ok());
    }

    #[test]
    fn test_get() {
        let password = SecretString::from_str("password").unwrap();
        save(&password, "test").unwrap();
        assert_eq!(
            get("test").unwrap().expose_secret(),
            password.expose_secret()
        );
    }

    #[test]
    fn test_remove() {
        let password = SecretString::from_str("password").unwrap();
        save(&password, "test").unwrap();
        assert!(remove("test").is_ok());
    }
}
