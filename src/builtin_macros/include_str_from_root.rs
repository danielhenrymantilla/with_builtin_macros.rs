use super::*;

pub use include_str_from_root as call_on;

pub
fn include_str_from_root (input: TokenStream2)
  -> Result<String>
{
    super::include_bytes_from_root::call_on(input).and_then(|bytes| {
        String::from_utf8(bytes)
            .map_err(|err| Error::new(Span::call_site(),
                &format!("Non UTF-8 contents in file: {}", err),
            ))
    })
}
