/// Caesar Cipher
///
/// The struct is generated through the new() function.
///
pub struct Caesar {
    rot: u8,
}

impl Caesar {
    /// Initializes a caesar cipher with a supplied rotation.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::caesar::Caesar;
    ///
    /// let c = Caesar::new(13).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Will return Err() if the rotation is not between 1 and 26.
    ///
    pub fn new(rot: u8) -> Result<Self, String> {
        if rot < 1 || rot > 26 {
            Err(String::from("Rotation must be between 1 through 26"))
        } else {
            Ok(Caesar { rot })
        }
    }

    /// Enciphers a message with a caesar cipher.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::caesar::Caesar;
    ///
    /// let c = Caesar::new(13).unwrap();
    /// assert_eq!("guvf vf n frperg", c.encipher("this is a secret"));
    /// ```
    ///
    pub fn encipher(&self, plaintext: &str) -> String {
        Caesar::shift(plaintext, self.rot)
    }

    /// Deciphers a message with a caesar cipher.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::caesar::Caesar;
    ///
    /// let c = Caesar::new(10).unwrap();
    /// assert_eq!("this is a secret", c.decipher("drsc sc k combod"));
    /// ```
    ///
    pub fn decipher(&self, ciphertext: &str) -> String {
        let rot = 26 - self.rot;
        Caesar::shift(ciphertext, rot)
    }

    /// Shifts letters in a message by a given rotation.
    ///
    fn shift(text: &str, rot: u8) -> String {
        text.chars()
            .map(|c| match c as u8 {
                65...90 => (((c as u8 - 65 + rot) % 26) + 65) as char,
                97...122 => (((c as u8 - 97 + rot) % 26) + 97) as char,
                _ => c,
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::Caesar;

    #[test]
    fn encipher() {
        let c = Caesar::new(13).unwrap();
        assert_eq!("guvf vf n frperg", c.encipher("this is a secret"));
    }

    #[test]
    fn decipher() {
        let c = Caesar::new(10).unwrap();
        assert_eq!("this is a secret", c.decipher("drsc sc k combod"));
    }

    #[test]
    fn all_rotations() {
        let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        for n in 1..27 {
            let c = Caesar::new(n).unwrap();
            assert_eq!(alpha, c.decipher(&c.encipher(alpha)));
        }
    }

    #[test]
    fn with_punctuation() {
        let c = Caesar::new(7).unwrap();
        assert_eq!(
            "Olssv, P ohcl h zljyla",
            c.encipher("Hello, I have a secret")
        );
    }

    #[test]
    fn with_unicode() {
        let c = Caesar::new(9).unwrap();
        assert_eq!("R 🖤 lahycxpajyqh", c.encipher("I 🖤 cryptography"));
    }

    #[test]
    fn too_low_rotation() {
        assert!(Caesar::new(0).is_err());
    }

    #[test]
    fn too_high_rotation() {
        assert!(Caesar::new(27).is_err());
    }
}
