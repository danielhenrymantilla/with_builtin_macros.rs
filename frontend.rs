#![forbid(unsafe_code)]
#![no_std]
//! # `::with_builtin_macros`
//!
//! [![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](https://github.com/danielhenrymantilla/with_builtin_macros.rs)
//! [![Latest version](https://img.shields.io/crates/v/with_builtin_macros.svg)](https://crates.io/crates/with_builtin_macros)
//! [![Documentation](https://docs.rs/with_builtin_macros/badge.svg)](https://docs.rs/with_builtin_macros)
//! [![MSRV](https://img.shields.io/badge/MSRV-1.45.0-white)](https://gist.github.com/danielhenrymantilla/8e5b721b3929084562f8f65668920c33)
//! [![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![License](https://img.shields.io/crates/l/with_builtin_macros.svg)](https://github.com/danielhenrymantilla/with_builtin_macros.rs/blob/master/LICENSE)
//! [![CI](https://github.com/danielhenrymantilla/with_builtin_macros.rs/workflows/CI/badge.svg)](https://github.com/danielhenrymantilla/with_builtin_macros.rs/actions)
//!
//! ### Helper for macro_rules authors to chain their macros with builtin ones (such as `env!`, `include!`, or `concat_idents!`).


pub use ::proc_macros::with_builtin;

/// Documentation for the built-in macros supported by [`with_builtin`!].
pub mod builtin_macros {
    /// See [the stdlib documentation][`::core::concat!`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use ::with_builtin_macros::with_builtin;
    /// macro_rules! expect_hello_world {
    ///     ("Hello, World!") => ();
    /// }
    ///
    /// with_builtin!(let $msg = concat!("Hello, ", "World!") in {
    ///     expect_hello_world!($msg);
    /// });
    /// ```
    pub mod concat {}

    /// See [the stdlib documentation][`::core::concat_idents!`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use ::with_builtin_macros::with_builtin;
    ///
    /// with_builtin!(let $fname = concat_idents!(f, o, o) in {
    ///     fn $fname ()
    ///     {}
    /// });
    ///
    /// foo();
    /// ```
    pub mod concat_idents {}

    /// See [the stdlib documentation][`::core::env!`].
    pub mod env {}


    /// See [the stdlib documentation][`::core::option_env!`].
    ///
    /// # Example
    ///
    /// ```rust
    /// // Objective
    /// cfg_env! {
    ///     #[cfg_env("NON_EXISTENT_ENV_VAR")]
    ///     compile_error!("Unreachable!");
    /// }
    /// // Some module to be used when debugging
    /// cfg_env! {
    ///     #[cfg_env("MY_FANCY_NAME_DEBUG")]
    ///     pub mod debugging_helpers {
    ///         // ...
    ///     }
    /// }
    ///
    /// // Implementation
    /// macro_rules! cfg_env {(
    ///     #[cfg_env($var_name:expr)]
    ///     $item:item
    /// ) => (
    ///     ::with_builtin_macros::with_builtin!(let $mb_env = option_env!($var_name) in {
    ///         cfg_non_empty! {
    ///             #[cfg_non_empty($mb_env)]
    ///             $item
    ///         }
    ///     });
    /// )}
    /// use cfg_env;
    ///
    /// // Helper
    /// macro_rules! cfg_non_empty {
    ///     (
    ///         #[cfg_non_empty()]
    ///         $item:item
    ///     ) => (
    ///         /* Nothing */
    ///     );
    ///     (
    ///         #[cfg_non_empty( $($stuff:tt)+ )]
    ///         $item:item
    ///     ) => (
    ///         $item
    ///     );
    /// }
    /// use cfg_non_empty;
    /// ```
    pub mod option_env {}


    /// See [the stdlib documentation][`::core::stringify!`].
    pub mod stringify {}

    /// See [the stdlib documentation][`::core::include!`].
    ///
    /// The difference between [`::core::include!`] and this version
    /// of the macro is that, due to limitations of proc-macros in stable Rust,
    /// it is not possible to know whence a macro is called.
    ///
    /// Thus, this macro requires it be called with a path that starts
    /// from the "root" of the package, _i.e._, a path that will be interpreted
    /// as if it started from: `concat!(env!("CARGO_MANIFEST_DIR"), "/")`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # macro_rules! ignore {($($t:tt)*) => ()} ignore! {
    /// use ::with_builtin_macros::with_builtin;
    ///
    /// macro_rules! metamancy {(
    ///     $use_stmt:item
    ///
    ///     $macro_def_item:item
    ///
    ///     #[cfg(any())]
    ///     mod to_be_expanded { $foo_fn:item }
    ///
    ///     $main_fn:item
    ///
    ///     $macro_call:item
    /// ) => (
    ///     $foo_fn
    /// )}
    ///
    /// #[cfg(any())]
    /// mod to_be_expanded {
    ///     fn foo ()
    ///     {}
    /// }
    ///
    /// fn main ()
    /// {
    ///     foo();
    /// }
    ///
    /// with_builtin!(let $this_file = include_from_root!("src/main.rs") in {
    ///     metamancy!($this_file);
    /// });
    /// # } fn main () {}
    /// ```
    pub mod include_from_root {}

    /// See [the stdlib documentation][`::core::include_bytes!`].
    ///
    /// The difference between [`::core::include_bytes!`] and this version
    /// of the macro is that, due to limitations of proc-macros in stable Rust,
    /// it is not possible to know whence a macro is called.
    ///
    /// Thus, this macro requires it be called with a path that starts
    /// from the "root" of the package, _i.e._, a path that will be interpreted
    /// as if it started from: `concat!(env!("CARGO_MANIFEST_DIR"), "/")`.
    pub mod include_bytes_from_root {}

    /// See [the stdlib documentation][`::core::include_str!`].
    ///
    /// The difference between [`::core::include_str!`] and this version
    /// of the macro is that, due to limitations of proc-macros in stable Rust,
    /// it is not possible to know whence a macro is called.
    ///
    /// Thus, this macro requires it be called with a path that starts
    /// from the "root" of the package, _i.e._, a path that will be interpreted
    /// as if it started from: `concat!(env!("CARGO_MANIFEST_DIR"), "/")`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use ::with_builtin_macros::with_builtin;
    ///
    /// const HEX_ARRAY: &[u8] = {
    ///     with_builtin!(let $hex_string = include_str_from_root!("path/to/hex/file") in {
    ///         ::hex_literal::hex!($hex_string)
    ///     })
    /// };
    /// ```
    pub mod include_str_from_root {}
}
