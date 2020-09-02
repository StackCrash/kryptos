/// Rail Fence Cipher
///
/// The struct is generated through the new() function.
///
pub struct RailFence {
    key: usize,
}

impl RailFence {
    /// Initializes a rail fence cipher with a supplied height.
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
    pub fn new(key: usize) -> Result<Self, String> {
        if key == 0 {
            Err(String::from("The key must be 1 or greater"))
        } else {
            Ok(RailFence { key })
        }
    }

    /// Enciphers a message with a rail fence cipher.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::railfence::RailFence;
    ///
    /// let r = RailFence::new(7).unwrap();
    /// assert_eq!(
    ///     "I  pace aesnket\' cetr",
    ///     r.encipher("I can't keep a secret").unwrap()
    /// );
    /// ```
    ///
    pub fn encipher(&self, plaintext: &str) -> Result<String, String> {
        if self.key == 1 {
            return Ok(String::from(plaintext));
        }
        let order = self
            .calculate_order(plaintext)
            .expect("Unable to calculate the order");

        let mut ciphertext = String::new();
        for p in order {
            ciphertext.push(plaintext.chars().nth(p).unwrap());
        }

        Ok(ciphertext)
    }

    /// Deciphers a message with a rail fence cipher.
    ///
    /// # Examples
    ///
    /// ```
    /// use kryptos::ciphers::railfence::RailFence;
    ///
    /// let r = RailFence::new(7).unwrap();
    /// assert_eq!(
    ///     "I can't keep a secret",
    ///     r.decipher("I  pace aesnket\' cetr").unwrap()
    /// );
    /// ```
    ///
    pub fn decipher(&self, ciphertext: &str) -> Result<String, String> {
        if self.key == 1 {
            return Ok(String::from(ciphertext));
        }
        let order = self
            .calculate_order(ciphertext)
            .expect("Unable to calculate the order");

        let mut plaintext = vec![' '; ciphertext.chars().count()];
        for (p, c) in ciphertext.chars().enumerate() {
            plaintext[order[p]] = c;
        }

        Ok(plaintext.iter().collect::<String>())
    }

    // Calculate the row a given position is in matrix.
    fn calculate_row(&self, position: usize) -> Result<usize, String> {
        let iteration = 2 * self.key - 2;

        if position % iteration <= iteration / 2 {
            Ok(position % iteration)
        } else {
            Ok(iteration - position % iteration)
        }
    }

    // Calculate the order in which the text will be arranged.
    fn calculate_order(&self, text: &str) -> Result<Vec<usize>, String> {
        let length = text.chars().count();
        let mut matrix = vec![vec![(' ', false); length]; self.key];

        for (p, c) in text.chars().enumerate() {
            let row = self.calculate_row(p).unwrap();
            matrix[row][p] = (c, true);
        }
        let matrix = matrix;

        let mut order = Vec::new();
        for row in &matrix {
            for (p, c) in row.iter().enumerate() {
                if c.1 {
                    order.push(p);
                }
            }
        }

        Ok(order)
    }
}

#[cfg(test)]
mod tests {
    use super::RailFence;

    #[test]
    fn valid_height() {
        assert!(RailFence::new(4).is_ok());
    }

    #[test]
    fn invalid_height() {
        assert!(RailFence::new(0).is_err());
    }

    #[test]
    fn valid_position() {
        let r = RailFence::new(3).unwrap();
        assert!(r.calculate_row(5).is_ok());
    }

    #[test]
    fn accurate_position() {
        let r = RailFence::new(3).unwrap();
        assert_eq!(2, r.calculate_row(10).unwrap());
    }

    #[test]
    fn encipher() {
        let r = RailFence::new(7).unwrap();
        assert_eq!(
            "I  pace aesnket\' cetr",
            r.encipher("I can't keep a secret").unwrap()
        );
    }

    #[test]
    fn encipher_with_key_of_one() {
        let r = RailFence::new(1).unwrap();
        assert_eq!(
            "I can't keep a secret",
            r.encipher("I can't keep a secret").unwrap()
        );
    }

    #[test]
    fn decipher() {
        let r = RailFence::new(7).unwrap();
        assert_eq!(
            "I can't keep a secret",
            r.decipher("I  pace aesnket\' cetr").unwrap()
        );
    }

    #[test]
    fn decipher_with_key_of_one() {
        let r = RailFence::new(1).unwrap();
        assert_eq!(
            "I  pace aesnket\' cetr",
            r.decipher("I  pace aesnket\' cetr").unwrap()
        );
    }
}
