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

    pub fn encipher(self, plaintext: &str) -> Result<String, String> {
        if self.key == 1 {
            return Ok(String::from(plaintext));
        }
        let width = plaintext.chars().count();

        let mut matrix = vec![vec![(' ', false); width]; self.key];
        for (p, c) in plaintext.chars().enumerate() {
            let row = self.calculate_row(p).unwrap();
            matrix[row][p] = (c, true);
        }

        let mut text = String::new();
        for row in &matrix {
            for col in row.iter() {
                if col.1 {
                    text.push(col.0);
                }
            }
        }

        Ok(text)
    }

    // Calculate the row a given position is in matrix
    fn calculate_row(&self, position: usize) -> Result<usize, String> {
        let iteration = 2 * self.key - 2;

        if position % iteration <= iteration / 2 {
            Ok(position % iteration)
        } else {
            Ok(iteration - position % iteration)
        }
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
}
