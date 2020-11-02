use super::*;

pub use include_bytes_from_root as call_on;

pub
fn include_bytes_from_root (input: TokenStream2)
  -> Result<Vec<u8>>
{
    let CompileTimeString(ref s) = parse2(input)?;
    let mut path: ::std::path::PathBuf =
        ::std::env::var_os(
            "CARGO_MANIFEST_DIR"
        )
        .expect("Missing `CARGO_MANIFEST_DIR` env var")
        .into()
    ;
    path.push(s);
    ::std::fs::read(&path).map_err(|err| Error::new(Span::call_site(), &format!(
        "Failed to read the contents of `{filename}`: {err_msg}",
        filename = path.display(),
        err_msg = err,
    )))
}
