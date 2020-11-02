use super::*;

pub use concat as call_on;

pub
fn concat (input: TokenStream2)
  -> Result<String>
{
    Punctuated::<CompileTimeString, Token![,]>::parse_terminated
        .parse2(input)
        .map(|it| {
            it  .into_iter()
                .map(|CompileTimeString(s)| s)
                .collect()
        })
}
