use common::ALPHABET;

/// Substitution Cipher
///
/// The struct is generated through the new() function.
///
pub struct Substitution {
    key: &'static str,
}

impl Substitution {
    /// Initializes a substitution cipher with a supplied substitute alphabet.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::substitution::Substitution;
    ///
    /// let s = Substitution::new("RBQIZDJCFMELOPAVNHYGWKTUXS").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Will return Err() if the alphabet is not 26 unique alphabetic characters.
    ///
    pub fn new(key: &'static str) -> Result<Self, String> {
        if key.chars().count() != 26 {
            return Err(String::from("Key is not the correct length"));
        }

        let mut alphabet_check = String::new();
        for c in key.chars() {
            if c.is_alphabetic() {
                if alphabet_check.contains(c) {
                    return Err(String::from("Key alphabet must be unique"));
                }

                alphabet_check.push(c);
                continue;
            } else {
                return Err(String::from("Key must be alphabetic"));
            }
        }

        Ok(Substitution { key })
    }

    /// Enciphers a message with a substitution cipher.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::substitution::*;
    ///
    /// let s = Substitution::new("NAKYQRTXBZPFIVEJSDCHGOUMWL").unwrap();
    /// assert_eq!(
    ///     "Ye weg fbpq cqkdqh iqccntqc",
    ///     s.encipher("Do you like secret messages").unwrap()
    /// );
    /// ```
    ///
    /// # Panics
    ///
    /// Will panic if there is a missing character or invalid character.
    ///
    pub fn encipher(&self, plaintext: &str) -> Result<String, &'static str> {
        Ok(plaintext
            .chars()
            .map(|c| match c as u8 {
                65..=90 => {
                    let index = ALPHABET.chars().position(|i| i == c).unwrap();
                    self.key
                        .chars()
                        .nth(index)
                        .expect("Something is wrong with the key alphabet")
                }
                97..=122 => {
                    let index = ALPHABET
                        .chars()
                        .position(|i| i == char::from(c as u8 - 97 + 65))
                        .unwrap();
                    (self
                        .key
                        .chars()
                        .nth(index)
                        .expect("Something is wrong with the key alphabet")
                        as u8
                        - 65
                        + 97) as char
                }
                _ => c,
            })
            .collect::<String>())
    }

    /// Deciphers a message with a substitution cipher.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::substitution::*;
    ///
    /// let s = Substitution::new("NAKYQRTXBZPFIVEJSDCHGOUMWL").unwrap();
    /// assert_eq!(
    ///     "Do you like secret messages",
    ///     s.decipher("Ye weg fbpq cqkdqh iqccntqc").unwrap()
    /// );
    /// ```
    ///
    /// # Panics
    ///
    /// Will panic if there is a missing character or invalid character.
    ///
    pub fn decipher(&self, plaintext: &str) -> Result<String, &'static str> {
        Ok(plaintext
            .chars()
            .map(|c| match c as u8 {
                65..=90 => {
                    let index = self.key.chars().position(|i| i == c).unwrap();
                    ALPHABET
                        .chars()
                        .nth(index)
                        .expect("Something is wrong with the key alphabet")
                }
                97..=122 => {
                    let index = self
                        .key
                        .chars()
                        .position(|i| i == char::from(c as u8 - 97 + 65))
                        .unwrap();
                    (ALPHABET
                        .chars()
                        .nth(index)
                        .expect("Something is wrong with the key alphabet")
                        as u8
                        - 65
                        + 97) as char
                }
                _ => c,
            })
            .collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::Substitution;

    #[test]
    fn unique_alphabet() {
        assert!(Substitution::new("RBQIZDJCFMELOPAVNHYGWKTUXS").is_ok());
    }

    #[test]
    fn non_unique_alphabet() {
        assert!(Substitution::new("ABCDEFGHIJKLMNOPQRSTUVWXAZ").is_err());
    }

    #[test]
    fn non_alphabetic() {
        assert!(Substitution::new("RBQI1DJCFMELOPAVNHYGWKTUXS").is_err());
    }

    #[test]
    fn too_small_alphabet() {
        assert!(Substitution::new("ABC").is_err());
    }

    #[test]
    fn too_large_alphabet() {
        assert!(Substitution::new("ABCDEFGHIJKLMNOPQRSTUVWXYZA").is_err());
    }

    #[test]
    fn encipher() {
        let s = Substitution::new("NAKYQRTXBZPFIVEJSDCHGOUMWL").unwrap();
        assert_eq!(
            "Ye weg fbpq cqkdqh iqccntqc",
            s.encipher("Do you like secret messages").unwrap()
        );
    }

    #[test]
    fn with_punctuation() {
        let s = Substitution::new("NAKYQRTXBZPFIVEJSDCHGOUMWL").unwrap();
        assert_eq!(
            "Ye weg fbpq cqkdqh iqccntqc?",
            s.encipher("Do you like secret messages?").unwrap()
        );
    }

    #[test]
    fn with_unicode() {
        let s = Substitution::new("NAKYQRTXBZPFIVEJSDCHGOUMWL").unwrap();
        assert_eq!(
            "Ye weg fbpq cqkdqh 🖤 iqccntqc",
            s.encipher("Do you like secret 🖤 messages").unwrap()
        );
    }

    #[test]
    fn decipher() {
        let s = Substitution::new("NAKYQRTXBZPFIVEJSDCHGOUMWL").unwrap();
        assert_eq!(
            "Do you like secret messages",
            s.decipher("Ye weg fbpq cqkdqh iqccntqc").unwrap()
        );
    }
}
