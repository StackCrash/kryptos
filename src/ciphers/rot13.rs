/// ROT13 Cipher
///
/// The struct is generated through the new() function.
///
pub struct Rot13 {}

impl Rot13 {
    /// Initializes a rot13 cipher with a supplied rotation.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::rot13::Rot13;
    ///
    /// let c = Rot13::new().unwrap();
    /// ```
    ///
    pub fn new() -> Result<Self, String> {
        Ok(Rot13 {})
    }

    /// Enciphers a message with a rot13 cipher.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::rot13::Rot13;
    ///
    /// let c = Rot13::new().unwrap();
    /// assert_eq!("guvf vf n frperg", c.encipher("this is a secret").unwrap());
    /// ```
    ///
    pub fn encipher(&self, plaintext: &str) -> Result<String, &'static str> {
        Rot13::shift(plaintext, 13)
    }

    /// Deciphers a message with a rot13 cipher.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::rot13::Rot13;
    ///
    /// let c = Rot13::new().unwrap();
    /// assert_eq!("this is a secret", c.decipher("guvf vf n frperg").unwrap());
    /// ```
    ///
    pub fn decipher(&self, ciphertext: &str) -> Result<String, &'static str> {
        Rot13::shift(ciphertext, 13)
    }

    // Shifts letters in a message by a given rotation.
    //
    fn shift(text: &str, rot: u8) -> Result<String, &'static str> {
        Ok(text
            .chars()
            .map(|c| match c as u8 {
                65..=90 => (((c as u8 - 65 + rot) % 26) + 65) as char,
                97..=122 => (((c as u8 - 97 + rot) % 26) + 97) as char,
                _ => c,
            })
            .collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::Rot13;

    #[test]
    fn encipher() {
        let c = Rot13::new().unwrap();
        assert_eq!("guvf vf n frperg", c.encipher("this is a secret").unwrap());
    }

    #[test]
    fn decipher() {
        let c = Rot13::new().unwrap();
        assert_eq!("this is a secret", c.decipher("guvf vf n frperg").unwrap());
    }

    #[test]
    fn with_punctuation() {
        let c = Rot13::new().unwrap();
        assert_eq!(
            "Uryyb, V unir n frperg",
            c.encipher("Hello, I have a secret").unwrap()
        );
    }

    #[test]
    fn with_unicode() {
        let c = Rot13::new().unwrap();
        assert_eq!(
            "V 🖤 pelcgbtencul",
            c.encipher("I 🖤 cryptography").unwrap()
        );
    }
}
