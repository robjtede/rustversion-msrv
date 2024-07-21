use crate::{
    error::{Error, Result},
    iter::Iter,
};

pub fn parse_end(iter: Iter) -> Result<()> {
    match iter.next() {
        None => Ok(()),
        Some(unexpected) => Err(Error::new(unexpected.span(), "unexpected token")),
    }
}
