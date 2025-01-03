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

impl<'haystack> Iterator for StrSplit<'haystack, '_> {
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_deli) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_deli];
                *remainder = &remainder[(next_deli + self.delimiter.len())..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, &format!("{}", c)).next().unwrap()
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("Hello world", 'o'), "Hell")
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
}
