#![forbid(unsafe_code)]
#![allow(nonstandard_style, unused_imports)]

/// For compatibility with older versions of Rust.
extern crate proc_macro;

use ::proc_macro::{
    TokenStream,
};
use ::proc_macro2::{
    Group,
    Span,
    TokenStream as TokenStream2,
};
use ::quote::{
    quote,
    quote_spanned,
    ToTokens,
};
use ::syn::{*,
    parse::{
        Parse,
        Parser,
        ParseStream,
    },
    punctuated::Punctuated,
    spanned::Spanned,
    Result,
};

use self::builtin_macros as builtin;

use compile_time_string::CompileTimeString;
mod compile_time_string;

supported_builtin_macros![
    concat!(...),
    concat_idents!(...),
    env!(...),
    option_env!(...),
    include_from_root!(...),
    include_bytes_from_root!(...),
    include_str_from_root!(...),
    stringify!(...),
];

/// Pre-processor (pattern) with hard-coded implementations for some of the
/// built-in macros, so as to be able to chain their result with custom macros.
///
///  - [Click here to see the list of supported macros](
///     builtin_macros/index.html)
///
/// By using a pre-processor, this also enables "using" macros where it is
/// otherwise forbidden.
///
/// For instance, the following code fails to compile:
///
/// ```rust,compile_fail
/// fn concat_idents!(f, o, o) ()
/// {}
///
/// fn main ()
/// {
///     foo();
/// }
/// ```
///
/// But the following call does not:
///
/// ```rust
/// # extern crate with_builtin_macros_proc_macros as with_builtin_macros;
/// use ::with_builtin_macros::with_builtin;
///
/// with_builtin!(let $fname = concat_idents!(f, o, o) in {
///     fn $fname ()
///     {}
/// });
///
/// fn main ()
/// {
///     foo();
/// }
/// ```
///
///   - Note: for this behavior, with some added neat features (such as case
///     conversion), see the [`::paste::paste!`](https://docs.rs/paste) macro.
///
/// Even if we forget about allowed macro call-sites, there is also the issue
/// of macro evaluation order leading to things like the following not
/// working:
///
/// ```rust,compile_fail
/// /// Some (proc-)macro that expects a string literal containing a hexadecimal
/// /// sequence of digits, that it decodes into the represented byte sequence.
/// use ::hex_literal::hex;
///
/// const BYTES: &[u8] = hex!(include_str!(concat!(
///     env!("CARGO_MANIFEST_DIR"), "/", "data.hex",
/// )));
/// ```
///
/// With [`with_builtin!`], however, the following works:
///
/// ```rust
/// # macro_rules! ignore {($($t:tt)*) => ()} ignore! {
/// /// Some (proc-)macro that expects a string literal containing a hexadecimal
/// /// sequence of digits, that it decodes into the represented byte sequence.
/// use ::hex_literal::hex;
/// use ::with_builtin_macros::with_builtin;
///
/// const BYTES: &[u8] = {
///     with_builtin!(let $hex_str = include_str_from_root!("data.hex") in {
///         hex!($hex_str)
///     })
/// };
/// # } fn main () {}
/// ```
#[proc_macro] pub
fn with_builtin (input: TokenStream)
  -> TokenStream
{
    let Input {
        metavar,
        macro_invocation: (builtin_macro, macro_input),
        template,
    } = parse_macro_input!(input);
    let input_span = macro_input.span();
    let expansion = match builtin_macro.call_on(macro_input) {
        | Ok(it) => it,
        | Err(mut err) => {
            if format!("{:?}", err.span()) == format!("{:?}", Span::call_site()) {
                err = Error::new(input_span, &err.to_string());
            }
            return err.to_compile_error().into();
        },
    };
    map_replace(&metavar, &expansion, template)
        .into()
}

struct Input {
    metavar: Ident,
    macro_invocation: (BuiltinMacro, TokenStream2),
    template: TokenStream2,
}

impl Parse for Input {
    fn parse (input: ParseStream<'_>)
      -> Result<Self>
    {
        let _:             Token![ let ]        = input.parse()?;
        let _:             Token![ $   ]        = input.parse()?;
        let metavar:               Ident        = input.parse()?;
        let _:             Token![ =   ]        = input.parse()?;
        let builtin_macro:         BuiltinMacro = input.parse()?;
        let _:             Token![ !   ]        = input.parse()?;
        let macro_input = { let g: Group        = input.parse()?; g.stream() };
        let _:             Token![ in  ]        = input.parse()?;
        let template;              braced!(template in input);
        Ok(Input {
            metavar,
            macro_invocation: (builtin_macro, macro_input),
            template: template.parse()?,
        })
    }
}

macro_rules! supported_builtin_macros {(
    $(
        $( #[cfg $cfg:tt] )?
        $macro_name:ident !(...)
    ),+ $(,)?
) => (
    mod kw {
        $(
            $( #[cfg $cfg] )?
            ::syn::custom_keyword!($macro_name);
        )+
    }

    mod builtin_macros {
        use super::*;

        $(
            $( #[cfg $cfg] )?
            pub
            mod $macro_name;
        )+
    }

    enum BuiltinMacro {
        $(
            $( #[cfg $cfg] )?
            $macro_name(kw::$macro_name),
        )*
    }

    impl Parse for BuiltinMacro {
        fn parse (input: ParseStream<'_>)
          -> Result<Self>
        {
            let lookahead = input.lookahead1();
            match () {
                $(
                    $( #[cfg $cfg] )?
                    _case if lookahead.peek(kw::$macro_name) => {
                        input
                            .parse::<kw::$macro_name>()
                            .map(Self::$macro_name)
                    },
                )*
                _default => Err(lookahead.error()),
            }
        }
    }

    impl Spanned for BuiltinMacro {
        fn span (self: &'_ Self)
          -> Span
        {
            match *self {
                $(
                    $( #[cfg $cfg] )?
                    | Self::$macro_name(ref it) => it.span(),
                )+
            }
        }
    }

    impl BuiltinMacro {
        fn call_on (self: &'_ Self, args: TokenStream2)
          -> Result<TokenStream2>
        {
            match *self {
                $(
                    $( #[cfg $cfg] )?
                    | Self::$macro_name(_) => {
                        builtin::$macro_name::call_on(args)
                            .map(|it| it.into_token_stream())
                    },
                )+
            }
        }
    }
)}
use supported_builtin_macros;

trait MockToTokens {
    fn into_token_stream (self: Self)
      -> TokenStream2
    ;

    const _impl: () = {
        impl MockToTokens for Vec<u8> {
            fn into_token_stream (self: Vec<u8>)
              -> TokenStream2
            {
                let each_byte = &self;
                match self.len() {
                    | 0 => quote!(
                        [0_u8; 0]
                    ),
                    | _ => quote!(
                        [
                            #(#each_byte),*
                        ]
                    ),
                }
            }
        }
    };
}

fn map_replace (
    metavar: &'_ Ident,
    tokenized: &'_ TokenStream2,
    template: TokenStream2,
) -> TokenStream2
{
    use ::proc_macro2::{*, TokenTree as TT};
    let mut tokens = template.into_iter().peekable();
    let mut ret = TokenStream2::new();
    loop {
        match (tokens.next(), tokens.peek()) {
            | (
                Some(TT::Punct(dollar)),
                Some(&TT::Ident(ref ident)),
            )
                if  dollar.as_char() == '$'
                &&  ident == metavar
            => {
                drop(tokens.next());
                ret.extend(tokenized.clone());
            },

            | (Some(TT::Group(group)), _) => {
                ret.extend(Some(TT::Group(Group::new(
                    group.delimiter(),
                    map_replace(metavar, tokenized, group.stream()),
                ))));
            },

            | (None, _) => break,

            | (tt, _) => ret.extend(tt),
        }
    }
    ret
}
