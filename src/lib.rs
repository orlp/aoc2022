use regex::{Regex, CaptureMatches, Captures};

fn extract_from_capture<'t, const N: usize>(caps: Captures<'t>) -> (&'t str, [&'t str; N]) {
    let mut participating = caps.iter().flatten();
    let whole_match = participating.next().unwrap().as_str();
    let captured = [0; N].map(|_| participating.next().unwrap().as_str());
    assert!(participating.next().is_none(), "too many participating capture groups");
    (whole_match, captured)
}

pub struct RegexExtractIter<'r, 't, const N: usize> {
    captures: CaptureMatches<'r, 't>,
}

impl<'r, 't, const N: usize> Iterator for RegexExtractIter<'r, 't, N> {
    type Item = (&'t str, [&'t str; N]);

    fn next(&mut self) -> Option<Self::Item> {
        self.captures.next().map(extract_from_capture)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.captures.size_hint()
    }
}

pub trait RegexExtract {
    /// Finds the leftmost-first match in `text` and returns a tuple containing the whole match
    /// and its N participating capture groups as strings. If no match is found, `None` is returned.
    ///
    /// # Panics
    ///
    /// Panics if the number of participating captures is not equal to N.
    fn extract<'t, const N: usize>(&self, text: &'t str) -> Option<(&'t str, [&'t str; N])>;

    fn extract_iter<'r, 't, const N: usize>(&'r self, text: &'t str) -> RegexExtractIter<'r, 't, N>;
}

impl RegexExtract for Regex {
    fn extract<'t, const N: usize>(&self, text: &'t str) -> Option<(&'t str, [&'t str; N])> {
        self.captures(text).map(extract_from_capture)
    }
    
    fn extract_iter<'r, 't, const N: usize>(&'r self, text: &'t str) -> RegexExtractIter<'r, 't, N> {
        RegexExtractIter { captures: self.captures_iter(text) }
    }
}

pub trait GetDisjointMut {
    type Item;

    fn get_disjoint_mut<const N: usize>(&mut self, idxs: [usize; N]) -> Option<[&mut Self::Item; N]>;
}

impl<T> GetDisjointMut for [T] {
    type Item = T;

    fn get_disjoint_mut<const N: usize>(&mut self, idxs: [usize; N]) -> Option<[&mut Self::Item; N]> {
        if idxs.iter().any(|i| *i >= self.len()) {
            return None;
        }

        // Check all disjoint.
        let disjoint = match N {
            0..=1 => true,
            2 => idxs[0] != idxs[1],
            3 => idxs[0] != idxs[1] && idxs[1] != idxs[2] && idxs[0] != idxs[2],
            4..=8 => {
                for i in 1..N {
                    for j in 0..i {
                        if idxs[i] == idxs[j] {
                            return None;
                        }
                    }
                }
                true
            }
            _ => {
                let mut sorted = idxs;
                sorted.sort_unstable();
                for i in 1..N {
                    if idxs[i-1] == idxs[i] {
                        return None;
                    }
                }
                true
            }
        };

        let p = self.as_mut_ptr();
        disjoint.then(|| idxs.map(|i| unsafe { &mut *p.add(i) }))
    }
}
