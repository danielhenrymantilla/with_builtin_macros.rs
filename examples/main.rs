use ::with_builtin_macros::with_builtin;

macro_rules! count_tts {(
    $($tt:tt)*
) => (
    {
        0 $(+ { let _ = stringify!($tt); 1 })*
    }
)}

with_builtin!(let $fname = concat_idents!(ma, in) in {
    fn $fname ()
    {
        with_builtin! {
            let $code = include_from_root!(concat!(
                env!("CARGO_MANIFEST_DIR"), "/",
                stringify!(examples),
                "/main.rs",
            )) in {
                assert_eq!(
                    count_tts!($code),
                    14,
                );
            }
        }
    }
});
