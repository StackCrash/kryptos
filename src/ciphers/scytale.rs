/// Scytale Cipher
///
/// The struct is generated through the new() function.
///
pub struct Scytale {
    height: usize,
}

impl Scytale {
    /// Initializes a scytale cipher with a supplied height.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::scytale::Scytale;
    ///
    /// let s = Scytale::new(4).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Will return Err() if the key is zero.
    ///
    pub fn new(height: usize) -> Result<Self, String> {
        if height == 0 {
            Err(String::from("The height must be 1 or greater"))
        } else {
            Ok(Scytale { height })
        }
    }

    /// Enciphers a message with a scytale cipher.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::scytale::Scytale;
    ///
    /// let s = Scytale::new(6).unwrap();
    /// assert_eq!("I r aeh tas ve ec ", s.encipher("I have a secret").unwrap());
    /// ```
    ///
    /// # Errors
    /// Will return an error if the height is greater than the length of the text
    ///
    pub fn encipher(&self, plaintext: &str) -> Result<String, String> {
        let matrix = self.transpose(plaintext, false);
        if matrix.is_err() {
            return Err(matrix.unwrap_err().clone());
        }

        Ok(matrix
            .unwrap()
            .iter()
            .map(|col| col.iter().cloned().collect::<String>())
            .collect::<String>())
    }

    /// Deciphers a message with a scytale cipher.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::scytale::Scytale;
    ///
    /// let s = Scytale::new(6).unwrap();
    /// assert_eq!("I have a secret", s.decipher("I r aeh tas ve ec ").unwrap());
    /// ```
    ///
    /// # Errors
    /// Will return an error if the height is greater than the length of the text
    ///
    pub fn decipher(&self, ciphertext: &str) -> Result<String, String> {
        let mut plaintext = String::new();
        let width = f64::ceil(ciphertext.chars().count() as f64 / self.height as f64) as usize;

        // Pass any errors from self.transpose()
        let matrix = self.transpose(ciphertext, true);
        if matrix.is_err() {
            return Err(matrix.unwrap_err().clone());
        }

        let matrix = matrix.unwrap();
        for row in 0..width {
            for col in matrix.iter().take(self.height) {
                plaintext.push(col[row])
            }
        }

        Ok(String::from(plaintext.trim()))
    }

    // Takes a &str and converts it to a two dimensional vector.
    fn transpose(&self, text: &str, decipher: bool) -> Result<Vec<Vec<char>>, String> {
        if self.height >= text.chars().count() {
            return Err(String::from("The height must be less than the text"));
        }

        let width = f64::ceil(text.chars().count() as f64 / self.height as f64) as usize;
        let mut matrix = vec![vec![' '; width]; self.height];
        for (p, c) in text.chars().enumerate() {
            if decipher {
                let col = p / width;
                let row = p % width;

                matrix[col][row] = c;
            } else {
                let col = p % self.height;
                let row = p / self.height;

                matrix[col][row] = c;
            }
        }

        Ok(matrix)
    }
}

#[cfg(test)]
mod tests {
    use super::Scytale;

    #[test]
    fn valid_height() {
        assert!(Scytale::new(4).is_ok());
    }

    #[test]
    fn invalid_height() {
        assert!(Scytale::new(0).is_err());
    }

    #[test]
    fn height_greater_than_text() {
        let s = Scytale::new(10).unwrap();
        assert!(s.encipher("abc").is_err());
    }

    #[test]
    fn encipher() {
        let s = Scytale::new(6).unwrap();
        assert_eq!("I r aeh tas ve ec ", s.encipher("I have a secret").unwrap());
    }

    #[test]
    fn decipher() {
        let s = Scytale::new(6).unwrap();
        assert_eq!("I have a secret", s.decipher("I r aeh tas ve ec ").unwrap());
    }
}
