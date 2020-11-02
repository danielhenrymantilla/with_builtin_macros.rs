use super::*;

pub use self::concat_idents as call_on;

pub
fn concat_idents (input: TokenStream2)
  -> Result<Ident>
{
    let ref mut acc = String::new();
    let input =
        Punctuated::<::proc_macro2::TokenTree, Token![,]>::parse_terminated
            .parse2(input)?
    ;
    if input.is_empty() {
        return Err(Error::new(Span::call_site(), "Missing args"));
    }
    input
        .iter()
        .for_each(|tt| {
            use ::core::fmt::Write;
            let _ = write!(acc, "{}", tt);
        })
    ;
    parse_str::<Ident>(acc)
        .map_err(|err| Error::new_spanned(input, &format!(
            "`{}` is not a valid identifier: {}", acc, err,
        )))
}
