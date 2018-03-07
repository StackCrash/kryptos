pub const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn binary_to_char(bin: &str) -> Result<char, String> {
    let binary = bin.as_bytes().iter().map(|b| b - 48).collect::<Vec<u8>>();
    for bit in &binary {
        if bit > &1 {
            return Err(String::from("Must be a valid binary number"));
        }
    }

    Ok((binary.iter().fold(0, |x, &b| x * 2 + b as u8) + 65) as char)
}

#[cfg(test)]
mod tests {
    use super::binary_to_char;

    #[test]
    fn valid_binary() {
        assert!(binary_to_char("010").is_ok());
    }

    #[test]
    fn invalid_binary() {
        assert!(binary_to_char("2010").is_err());
    }

    #[test]
    fn number_to_char() {
        assert_eq!('C', binary_to_char("010").unwrap());
    }
}
