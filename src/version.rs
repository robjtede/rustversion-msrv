#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Version {
    pub minor: u16,
    pub patch: u16,
    pub channel: Channel,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Channel {
    Stable,
    Beta,
    Nightly,
    Dev,
}
