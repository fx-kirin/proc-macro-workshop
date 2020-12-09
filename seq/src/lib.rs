use proc_macro::TokenStream;
use quote::{quote_spanned, quote};
use std::iter::FromIterator;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Result, Token};

#[derive(Debug)]
struct SeqMacroInput {
    from: isize,
    to: isize,
    tt: proc_macro2::TokenStream,
    ident: proc_macro2::Ident,
}

impl Parse for SeqMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = syn::Ident::parse(input)?;
        let _in = <Token![in]>::parse(input)?;
        let from = syn::LitInt::parse(input)?.base10_parse()?;
        let _dots = <Token![..]>::parse(input)?;
        let to = syn::LitInt::parse(input)?.base10_parse()?;

        let content;
        let _braces = syn::braced!(content in input);
        let tt = proc_macro2::TokenStream::parse(&content)?;

        Ok(SeqMacroInput {
            from,
            to,
            tt,
            ident,
        })
    }
}

impl Into<TokenStream> for SeqMacroInput {
    fn into(self) -> TokenStream {
        let expanded: Vec<proc_macro2::TokenStream> = (self.from..self.to)
            .map(|i| expand(self.ident.clone(), self.tt.clone(), i))
            .collect();
        let output : proc_macro2::TokenStream = proc_macro2::TokenStream::from_iter(expanded.into_iter()).into();
        (quote!{
            #output
        }).into()
    }
}

fn expand(
    ident: syn::Ident,
    stream: proc_macro2::TokenStream,
    i: isize,
) -> proc_macro2::TokenStream {
    let mut out = proc_macro2::TokenStream::new();
    let mut tts = stream.into_iter();
    while let Some(tt) = tts.next() {
        out.extend(std::iter::once(expand2(&ident, tt, &mut tts, i)));
    }
    out
}

fn expand2(
    ident: &syn::Ident,
    tt: proc_macro2::TokenTree,
    rest: &mut proc_macro2::token_stream::IntoIter,
    i: isize,
) -> proc_macro2::TokenTree {
    match tt {
        proc_macro2::TokenTree::Group(g) => {
            let mut expanded =
                proc_macro2::Group::new(g.delimiter(), expand(ident.clone(), g.stream(), i));
            expanded.set_span(g.span());
            proc_macro2::TokenTree::Group(expanded)
        }
        proc_macro2::TokenTree::Ident(ref inner_ident) if inner_ident == ident => {
            let i = proc_macro2::Literal::isize_unsuffixed(i);
            proc_macro2::TokenTree::Literal(
                syn::parse2(quote_spanned! {inner_ident.span() => #i}).unwrap(),
            )
        }
        proc_macro2::TokenTree::Ident(mut inner_ident) => {
            let mut peek = rest.clone();
            match (peek.next(), peek.next()) {
                (
                    Some(proc_macro2::TokenTree::Punct(ref punct)),
                    Some(proc_macro2::TokenTree::Ident(ref inner_ident2)),
                ) if punct.as_char() == '#' && inner_ident2 == ident => {
                    inner_ident = proc_macro2::Ident::new(
                        &format!("{}{}", inner_ident, i),
                        inner_ident.span(),
                    );
                    *rest = peek;
                }
                _ => {}
            }
            proc_macro2::TokenTree::Ident(inner_ident)
        }
        tt => tt,
    }
}

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SeqMacroInput);
    input.into()
}
