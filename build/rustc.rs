use std::fmt::{self, Debug};

use self::Channel::*;

pub enum ParseResult {
    Success(Version),
    OopsClippy,
    Unrecognized,
}

#[cfg_attr(test, derive(PartialEq))]
pub struct Version {
    pub minor: u16,
    pub patch: u16,
    pub channel: Channel,
}

#[cfg_attr(test, derive(PartialEq))]
pub enum Channel {
    Stable,
    Beta,
    Nightly,
    Dev,
}

#[cfg_attr(test, derive(PartialEq))]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

pub fn parse(string: &str) -> ParseResult {
    let last_line = string.lines().last().unwrap_or(string);
    let mut words = last_line.trim().split(' ');

    match words.next() {
        Some("rustc") => {}
        Some(word) if word.starts_with("clippy") => return ParseResult::OopsClippy,
        Some(_) | None => return ParseResult::Unrecognized,
    }

    parse_words(&mut words).map_or(ParseResult::Unrecognized, ParseResult::Success)
}

fn parse_words(words: &mut dyn Iterator<Item = &str>) -> Option<Version> {
    let mut version_channel = words.next()?.split('-');
    let version = version_channel.next()?;
    let channel = version_channel.next();

    let mut digits = version.split('.');
    let major = digits.next()?;
    if major != "1" {
        return None;
    }
    let minor = digits.next()?.parse().ok()?;
    let patch = digits.next().unwrap_or("0").parse().ok()?;

    let channel = match channel {
        None => Stable,
        Some("dev") => Dev,
        Some(channel) if channel.starts_with("beta") => Beta,
        Some("nightly") => match words.next() {
            Some(hash) if hash.starts_with('(') => match words.next() {
                None if hash.ends_with(')') => Dev,
                Some(date) if date.ends_with(')') => {
                    let mut date = date[..date.len() - 1].split('-');
                    let _year: u16 = date.next()?.parse().ok()?;
                    let _month: u8 = date.next()?.parse().ok()?;
                    let _day: u8 = date.next()?.parse().ok()?;
                    match date.next() {
                        None => Nightly,
                        Some(_) => return None,
                    }
                }
                None | Some(_) => return None,
            },
            Some(_) => return None,
            None => Dev,
        },
        Some(_) => return None,
    };

    Some(Version {
        minor,
        patch,
        channel,
    })
}

impl Debug for Version {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_struct("crate::version::Version")
            .field("minor", &self.minor)
            .field("patch", &self.patch)
            .field("channel", &self.channel)
            .finish()
    }
}

impl Debug for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Channel::Stable => f.write_str("crate::version::Channel::Stable"),
            Channel::Beta => f.write_str("crate::version::Channel::Beta"),
            Channel::Nightly => f.write_str("crate::version::Channel::Nightly"),
            Channel::Dev => f.write_str("crate::version::Channel::Dev"),
        }
    }
}

impl Debug for Date {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_struct("crate::date::Date")
            .field("year", &self.year)
            .field("month", &self.month)
            .field("day", &self.day)
            .finish()
    }
}
