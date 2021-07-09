use std::path::PathBuf;
use syn::parse::{Parse, ParseStream};
use syn::LitStr;
use syn::Token;

struct StrArg<T> {
    value: LitStr,
    _p: std::marker::PhantomData<T>,
}

impl<T: Parse> Parse for StrArg<T> {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let _ = input.parse::<T>()?;
        let _ = input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Self {
            value,
            _p: std::marker::PhantomData,
        })
    }
}

mod kw {
    syn::custom_keyword!(operation_id);
}

#[derive(Default, Debug)]
pub struct SwaggerArgs {
    pub operation_id: Option<LitStr>,
}

impl Parse for SwaggerArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::operation_id) {
                if args.operation_id.is_some() {
                    return Err(input.error("Expected only one `name` argument!"));
                }
                let name = input.parse::<StrArg<kw::operation_id>>()?.value;
                args.operation_id = Some(name);
            }
        }

        Ok(args)
    }
}
