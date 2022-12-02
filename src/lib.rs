use regex::Regex;

pub trait RegexExtract {
    /// Finds the leftmost-first match in `text` and returns a tuple containing the whole match
    /// and its N participating capture groups as strings. If no match is found, `None` is returned.
    ///
    /// # Panics
    ///
    /// Panics if the number of participating captures is not equal to N.
    fn extract<'t, const N: usize>(&self, text: &'t str) -> Option<(&'t str, [&'t str; N])>;
}

impl RegexExtract for Regex {
    fn extract<'t, const N: usize>(&self, text: &'t str) -> Option<(&'t str, [&'t str; N])> {
        let caps = self.captures(text)?;
        let mut participating = caps.iter().flatten();
        let whole_match = participating.next().unwrap().as_str();
        let captured = [0; N].map(|_| participating.next().unwrap().as_str());
        assert!(participating.next().is_none(), "too many participating capture groups");
        Some((whole_match, captured))
    }
}
