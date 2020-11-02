use super::*;

pub use stringify as call_on;

pub
fn stringify (input: TokenStream2)
  -> Result<String>
{
    Ok(input.to_string())
}
