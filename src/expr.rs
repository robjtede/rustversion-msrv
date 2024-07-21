use std::env;

use proc_macro::{Span, TokenTree};

use crate::{
    error::{Error, Result},
    iter::Iter,
    version::{Channel, Version},
};

pub enum Expr {
    Msrv,
}

impl Expr {
    pub fn eval(&self, rustc: Version) -> bool {
        use self::Expr::*;

        match self {
            Msrv => {
                let cargo_rust_version = env::var("CARGO_PKG_RUST_VERSION")
                    .expect("manifest should specify a rust-version to use the msrv macro");
                let mut cargo_rust_version = cargo_rust_version.splitn(3, '.');

                let _maj = cargo_rust_version.next().unwrap();
                let min = cargo_rust_version.next().unwrap().parse::<u16>().unwrap();
                let patch = cargo_rust_version
                    .next()
                    .map_or(0, |val| val.parse().unwrap());

                rustc.channel == Channel::Stable && rustc.minor == min && rustc.patch >= patch
            }
        }
    }
}

pub fn parse(iter: Iter) -> Result<Expr> {
    match &iter.next() {
        Some(TokenTree::Ident(i)) if i.to_string() == "msrv" => Ok(Expr::Msrv),
        unexpected => {
            let span = unexpected
                .as_ref()
                .map_or_else(Span::call_site, TokenTree::span);
            Err(Error::new(span, "expected `msrv`"))
        }
    }
}
