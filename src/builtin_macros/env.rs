use super::*;

pub use env as call_on;

pub
fn env (input: TokenStream2)
  -> Result<String>
{
    super::option_env::call_on(input)?
        .ok_or_else(|| Error::new(
            Span::call_site(),
            "Missing env var or not UTF-8",
        ))
}
