#[cfg(test)]
mod tests;
mod python;

use std::collections::HashSet;
use suffix::SuffixTable;

pub struct SubstringMatcher {
    suffix_table: SuffixTable<'static, 'static>,
}

impl SubstringMatcher {
    pub fn new<'a>(texts: impl Iterator<Item = &'a str>) -> Self {
        let text = texts.fold(String::new(), |a, b| a + "\0" + b) + "\0";
        Self { suffix_table: SuffixTable::new(text) }
    }

    pub fn find<'a, 'b>(&'a self, pattern: &'b str) -> impl Iterator<Item = &'a str> {
        let text = self.suffix_table.text();
        let positions = self.suffix_table.positions(pattern);
        // is there a better way to remove duplicates?
        let mut matches = HashSet::new();
        positions.iter()
            .filter_map(move |&pos| {
                let pos = pos as usize;
                let start = text[..pos].rfind('\0').unwrap() + 1;
                if matches.insert(start) {
                    let end = text[pos..].find('\0').unwrap() + pos;
                    Some(&text[start..end])
                } else {
                    None
                }
            })
    }
}
