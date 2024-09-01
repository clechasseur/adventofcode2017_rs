use std::str::FromStr;

use regex::{Captures, Regex};

#[derive(Debug)]
pub struct EzCaptures<'h>(Captures<'h>);

impl<'h> EzCaptures<'h> {
    pub fn get<T>(&self, name: &str) -> T
    where
        T: FromStr,
    {
        self.0[name]
            .parse::<T>()
            .unwrap_or_else(|_| panic!("invalid {name} value: {}", &self.0[name]))
    }
}

pub trait EzCapturesHelper {
    fn ez_captures<'h>(&self, haystack: &'h str, to_parse: &str) -> EzCaptures<'h>;
}

impl EzCapturesHelper for Regex {
    fn ez_captures<'h>(&self, haystack: &'h str, to_parse: &str) -> EzCaptures<'h> {
        EzCaptures(
            self.captures(haystack)
                .unwrap_or_else(|| panic!("wrong {to_parse} format: {haystack}")),
        )
    }
}
