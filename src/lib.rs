//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {
    type Item = &'haystack str;
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

fn until_char<'s>(s: &'s str, c: char) -> &'s str {
    let delimiter = format!("{}", c);
    StrSplit::new(s, &delimiter).next().expect("StrPlit sempre retorna pelo menos um resultado")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("Olá mundo", 'n'), "Olá mu");
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