extern crate regex;

use regex::{Captures, Regex};

#[test]
fn test_regex() {
    let binary = "01010";

    let re = Regex::new(r"[01]{5}").unwrap();
    let result = re.replace_all(&binary, |caps: &Captures| {
        format!("{:?}", &caps[0].as_bytes().iter().map(|b| b - 48).collect::<Vec<u8>>())
    });

    assert_eq!(result.to_string(),"[0, 1, 0, 1, 0]")
}