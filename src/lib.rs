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
        self.suffix_table.positions(pattern).into_iter()
            .map(|&i| text[..i as usize].rfind('\0').unwrap() + 1)
            .collect::<HashSet<_>>().into_iter()
            .map(|i| &text[i..text[i..].find('\0').unwrap() + i])
    }
}
