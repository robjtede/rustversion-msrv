use std::{fmt::Display, iter::FromIterator};

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Error {
    begin: Span,
    end: Span,
    msg: String,
}

impl Error {
    pub fn new(span: Span, msg: impl Display) -> Self {
        Self::new2(span, span, msg)
    }

    pub fn new2(begin: Span, end: Span, msg: impl Display) -> Self {
        Error {
            begin,
            end,
            msg: msg.to_string(),
        }
    }

    pub fn into_compile_error(self) -> TokenStream {
        // compile_error! { $msg }
        TokenStream::from_iter(vec![
            TokenTree::Ident(Ident::new("compile_error", self.begin)),
            TokenTree::Punct({
                let mut punct = Punct::new('!', Spacing::Alone);
                punct.set_span(self.begin);
                punct
            }),
            TokenTree::Group({
                let mut group = Group::new(Delimiter::Brace, {
                    TokenStream::from_iter(vec![TokenTree::Literal({
                        let mut string = Literal::string(&self.msg);
                        string.set_span(self.end);
                        string
                    })])
                });
                group.set_span(self.end);
                group
            }),
        ])
    }
}
