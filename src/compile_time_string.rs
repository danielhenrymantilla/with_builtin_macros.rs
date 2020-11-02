use super::*;

pub(in crate)
struct CompileTimeString /* = */ (
    pub(in crate) String,
);

impl Parse for CompileTimeString {
    fn parse (input: ParseStream<'_>)
      -> Result<Self>
    {
        let lookahead = input.lookahead1();
        macro_rules! compile_time_string_macros {(
            $(
                $macro_name:ident !(...)
            ),+ $(,)?
        ) => ({
            mod kw {
                $(
                    ::syn::custom_keyword!($macro_name);
                )+
            }
            match () {
                _case if lookahead.peek(LitStr) => {
                    Ok(Self(
                        input
                            .parse::<LitStr>()
                            .unwrap()
                            .value()
                    ))
                },
                $(
                    _case if lookahead.peek(kw::$macro_name) => {
                        let _: kw::$macro_name = input.parse().unwrap();
                        let _: Token![!] = input.parse()?;
                        let contents = input.parse::<Group>()?.stream();
                        Ok(Self(
                            crate::builtin::$macro_name::call_on(contents)?
                        ))
                    },
                )+
                _default => Err(lookahead.error()),
            }
        })}
        return compile_time_string_macros![
            concat!(...),
            env!(...),
            include_str_from_root!(...),
            stringify!(...),
        ];
    }
}
