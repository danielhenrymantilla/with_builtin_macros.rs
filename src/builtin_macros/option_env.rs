use super::*;

pub use option_env as call_on;

pub
fn option_env (input: TokenStream2)
  -> Result<Option<String>>
{
    let CompileTimeString(ref s) = parse2(input)?;
    Ok(::std::env::var(s).ok())
}
