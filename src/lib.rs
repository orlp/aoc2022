use core::panic::Location;
use std::cmp::Ordering;
use std::fmt;

use regex::{CaptureMatches, Captures, Regex};

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

    fn extract_iter<'r, 't, const N: usize>(&'r self, text: &'t str)
        -> RegexExtractIter<'r, 't, N>;
}

impl RegexExtract for Regex {
    fn extract<'t, const N: usize>(&self, text: &'t str) -> Option<(&'t str, [&'t str; N])> {
        self.captures(text).map(extract_from_capture)
    }

    fn extract_iter<'r, 't, const N: usize>(
        &'r self,
        text: &'t str,
    ) -> RegexExtractIter<'r, 't, N> {
        RegexExtractIter {
            captures: self.captures_iter(text),
        }
    }
}

pub trait GetDisjointMut {
    type Item;

    fn get_disjoint_mut<const N: usize>(
        &mut self,
        idxs: [usize; N],
    ) -> Option<[&mut Self::Item; N]>;
}

impl<T> GetDisjointMut for [T] {
    type Item = T;

    fn get_disjoint_mut<const N: usize>(
        &mut self,
        idxs: [usize; N],
    ) -> Option<[&mut Self::Item; N]> {
        // Check all in range and disjoint.
        let mut valid = true;
        if N > 10 {
            let mut sorted = idxs;
            sorted.sort_unstable();
            valid &= idxs[N - 1] < self.len();
            for i in 1..N {
                valid &= idxs[i - 1] != idxs[i];
            }
        } else {
            for i in 0..N {
                valid &= idxs[i] < self.len();
                for j in 0..i {
                    valid &= idxs[i] != idxs[j];
                }
            }
        }

        let p = self.as_mut_ptr();
        valid.then(|| idxs.map(|i| unsafe { &mut *p.add(i) }))
    }
}

#[derive(Debug)]
pub struct NoneError {
    location: &'static Location<'static>,
}

impl std::error::Error for NoneError {}

impl fmt::Display for NoneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "some() was called on None value at {}", self.location)
    }
}

pub trait OptionSomeExt {
    type Item;

    fn some(self) -> Result<Self::Item, NoneError>;
}

impl<T> OptionSomeExt for Option<T> {
    type Item = T;

    #[track_caller]
    fn some(self) -> Result<Self::Item, NoneError> {
        match self {
            Some(val) => Ok(val),
            None => Err(NoneError {
                location: Location::caller(),
            }),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Priority<P, T>(pub P, pub T);

impl<P: Ord + Eq, T> Ord for Priority<P, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<P: Ord + Eq, T> PartialOrd for Priority<P, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<P: Eq, T> PartialEq for Priority<P, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<P: Eq, T> Eq for Priority<P, T> { }

