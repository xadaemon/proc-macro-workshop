extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, ExprBlock, Ident, LitInt, Token,
};

struct Seq {
    it_var: Ident,
    start: LitInt,
    end: LitInt,
    body: Option<ExprBlock>,
}

impl Parse for Seq {
    fn parse(input: ParseStream) -> Result<Self> {
        let it_var: Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let start: LitInt = input.parse()?;
        input.parse::<Token![..]>()?;
        let end: LitInt = input.parse()?;
        let body: ExprBlock = input.parse()?;
        Ok(Seq {
            it_var,
            start,
            end,
            body: Some(body),
        })
    }
}

#[proc_macro]
pub fn seq_macro(input: TokenStream) -> TokenStream {
    let Seq {
        it_var,
        start,
        end,
        body,
    } = parse_macro_input!(input as Seq);

    let expanded = quote! {
        enum a;
    };
    TokenStream::from(expanded)
}
