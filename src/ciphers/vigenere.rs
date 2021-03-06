/// Vigenere Cipher
///
/// The struct is generated through the new() function.
///
pub struct Vigenere {
    key: &'static str,
}

impl Vigenere {
    /// Initializes a vigenere cipher with a supplied key.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::vigenere::Vigenere;
    ///
    /// let v = Vigenere::new("secret").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Will return Err() if the key is not alphabetic.
    ///
    pub fn new(key: &'static str) -> Result<Self, String> {
        for c in key.chars() {
            if c.is_alphabetic() {
                continue;
            }
            return Err(String::from("Key must be alphabetic"));
        }
        Ok(Vigenere { key })
    }

    /// Enciphers a message with a vignere cipher.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::vigenere::Vigenere;
    ///
    /// let v = Vigenere::new("blaise").unwrap();
    /// assert_eq!(
    ///     "tsh ggy ilvm qsv hhqktfc",
    ///     v.encipher("shh you have you whisper").unwrap()
    /// );
    /// ```
    ///
    pub fn encipher(&self, plaintext: &str) -> Result<String, &'static str> {
        Vigenere::transpose(&self.convert_key().unwrap(), plaintext)
    }

    /// Deciphers a message with a vignere cipher.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::vigenere::Vigenere;
    ///
    /// let v = Vigenere::new("blaise").unwrap();
    /// assert_eq!(
    ///     "whispering can still be heard by others",
    ///     v.decipher("xsiahistno ueo dtqdp cp hmsve my wllfcs").unwrap()
    /// );
    /// ```
    ///
    pub fn decipher(&self, ciphertext: &str) -> Result<String, &'static str> {
        let mut filter = Vec::new();

        for n in self.convert_key().unwrap() {
            filter.push((26 - n) % 26);
        }
        Vigenere::transpose(&filter, ciphertext)
    }

    // Uses the converted key to perform the encipher or decipher of a message.
    //
    fn transpose(filter: &[u8], text: &str) -> Result<String, &'static str> {
        let mut filter_index = 0;
        let mut result = String::new();

        for c in text.chars() {
            match c as u8 {
                65..=90 => {
                    result.push(
                        (((c as u8 - 65 + filter[filter_index % filter.len()]) % 26) + 65) as char,
                    );
                    filter_index += 1;
                }
                97..=122 => {
                    result.push(
                        (((c as u8 - 97 + filter[filter_index % filter.len()]) % 26) + 97) as char,
                    );
                    filter_index += 1;
                }
                _ => result.push(c),
            }
        }
        Ok(result)
    }

    // Converts a key into a vector of u8.
    //
    fn convert_key(&self) -> Result<Vec<u8>, String> {
        self.key
            .chars()
            .map(|c| match c as u8 {
                65..=90 => Ok((c as u8 - 65) % 26),
                97..=122 => Ok((c as u8 - 97) % 26),
                _ => Err(String::from("Invalid character in key")),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Vigenere;

    #[test]
    fn valid_key() {
        assert!(Vigenere::new("secret").is_ok());
    }

    #[test]
    fn invalid_key() {
        assert!(Vigenere::new("s3cr3t").is_err());
    }

    #[test]
    fn key_conversion() {
        let v = vec![3, 8, 5, 5, 8, 4];
        let x = Vigenere::new("diffie").unwrap();
        assert_eq!(v, x.convert_key().unwrap());
    }

    #[test]
    fn invalid_key_conversion() {
        assert!(Vigenere::convert_key(&Vigenere { key: "dif.fie" }).is_err());
    }

    #[test]
    fn encipher() {
        let v = Vigenere::new("blaise").unwrap();
        assert_eq!(
            "tsh ggy ilvm qsv hhqktfc",
            v.encipher("shh you have you whisper").unwrap()
        );
    }

    #[test]
    fn with_punctuation() {
        let v = Vigenere::new("blaise").unwrap();
        assert_eq!(
            "tsh! ggy ilvm qsv hhqktfc",
            v.encipher("shh! you have you whisper").unwrap()
        );
    }

    #[test]
    fn with_unicode() {
        let v = Vigenere::new("babbage").unwrap();
        assert_eq!(
            "Eo zpu 🖤 yidrfu mkwtahfs?",
            v.encipher("Do you 🖤 secret messages?").unwrap()
        );
    }

    #[test]
    fn decipher() {
        let v = Vigenere::new("blaise").unwrap();
        assert_eq!(
            "whispering can still be heard by others",
            v.decipher("xsiahistno ueo dtqdp cp hmsve my wllfcs")
                .unwrap()
        );
    }
}
