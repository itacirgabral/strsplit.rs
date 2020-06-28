//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delimiter) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delimiter];
            *remainder = &remainder[(next_delimiter + self.delimiter.len())..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
    let gauge = vec!["a", "b", "c", "d", "e"];
    assert_eq!(letters, gauge);
}

#[test]
fn it_works_tail() {
    let haystack = "a b c d e ";
    let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
    let gauge = vec!["a", "b", "c", "d", "e", ""];
    assert_eq!(letters, gauge);
}