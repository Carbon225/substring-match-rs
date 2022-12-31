#[cfg(test)]
mod tests;

use std::collections::HashSet;
use suffix::SuffixTable;

pub struct SubstringMatcher<'s, 't> {
    suffix_table: SuffixTable<'s, 't>,
}

impl<'s, 't> SubstringMatcher<'s, 't> {
    pub fn new<'a>(texts: impl Iterator<Item = &'a str>) -> SubstringMatcher<'s, 't> {
        let text = texts.fold(String::new(), |a, b| a + "\0" + b) + "\0";
        SubstringMatcher {
            suffix_table: SuffixTable::new(text),
        }
    }

    pub fn find(&self, pattern: &str) -> Vec<&str> {
        let text = self.suffix_table.text();
        let positions = self.suffix_table.positions(pattern);
        // is there a better way to remove duplicates?
        let mut matches = HashSet::new();
        Vec::from_iter(positions.iter()
            .filter_map(|&pos| {
                let pos = pos as usize;
                let start = text[..pos].rfind('\0').unwrap() + 1;
                if matches.insert(start) {
                    let end = text[pos..].find('\0').unwrap() + pos;
                    Some(&text[start..end])
                } else {
                    None
                }
            })
        )
    }
}
