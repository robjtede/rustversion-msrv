use proc_macro::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};

use crate::{
    error::{Error, Result},
    expr, iter, token,
};

pub fn cfg(introducer: &str, args: TokenStream, input: TokenStream) -> TokenStream {
    try_cfg(introducer, args, input).unwrap_or_else(Error::into_compile_error)
}

fn try_cfg(introducer: &str, args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let introducer = Ident::new(introducer, Span::call_site());

    let mut full_args = TokenStream::from(TokenTree::Ident(introducer));
    if !args.is_empty() {
        full_args.extend(std::iter::once(TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            args,
        ))));
    }

    let full_args = &mut iter::new(full_args);
    let expr = expr::parse(full_args)?;
    token::parse_end(full_args)?;

    if expr.eval(crate::RUST_VERSION) {
        Ok(input)
    } else {
        Ok(TokenStream::new())
    }
}
