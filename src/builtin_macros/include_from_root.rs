use super::*;

pub use include_from_root as call_on;

pub
fn include_from_root (input: TokenStream2)
  -> Result<TokenStream2>
{
    super::include_str_from_root::call_on(input)?
        .parse::<TokenStream2>()
        .map_err(|err| Error::new(
            Span::call_site(),
            &format!("Failed to lex the file: {:?}", err),
        ))
}
