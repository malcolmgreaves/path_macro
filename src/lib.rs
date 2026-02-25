// Copyright David Tolnay <dtolnay@gmail.com>
// Licensed under either of Apache License, Version 2.0 or MIT license.
// Source: https://github.com/dtolnay/trybuild/blob/9b92eb13813a/src/path.rs

//! This library provides `path!`, a macro to join path components using `/`.
//!
//! Python's [`pathlib.Path`] provides an egonomic API for composing paths out
//! of path components by overloading the division operator:
//!
//! ```text
//! $ python3
//! >>> from pathlib import Path
//! >>> p = Path('a')
//! >>> q = p / 'b' / 'c'
//! >>> q
//! PosixPath('a/b/c')
//! ```
//!
//! The `path!` macro provides a similar API for Rust paths without having
//! to overload [`Path`] or [`PathBuf`]. `path!` works for any combination
//! of `&str`, `String`, `&Path`, and `PathBuf`.
//!
//! ```
//! use std::path::{Path, PathBuf};
//!
//! use path_macro::path;
//!
//! let p = path!(Path::new("a") / "x" / "y" / "z");
//!
//! #[cfg(unix)]
//! assert_eq!(p, Path::new("a/x/y/z"));
//!
//! #[cfg(windows)]
//! assert_eq!(p, Path::new("a\\x\\y\\z"));
//!
//! let p2 = path!("a" / "x" / "y" / "z");
//! assert_eq!(p, p2);
//!
//! let p3 = path!("a" / Path::new("x") / Path::new("y") / "z");
//! assert_eq!(p, p3);
//!
//! assert_eq!(path!(PathBuf::new() / "b"), PathBuf::new().join("b"));
//! ```
//! 
//! [`pathlib.Path`]: https://docs.python.org/3/library/pathlib.html#basic-use
//! [`Path`]: https://doc.rust-lang.org/std/path/struct.Path.html
//! [`PathBuf`]: https://doc.rust-lang.org/std/path/struct.PathBuf.html

#[macro_export]
macro_rules! path {
    ($($tt:tt)+) => {
        $crate::__tokenize_path!([] [] $($tt)+)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __tokenize_path {
    ([$(($($component:tt)+))*] [$($cur:tt)+] / $($rest:tt)+) => {
        $crate::__tokenize_path!([$(($($component)+))* ($($cur)+)] [] $($rest)+)
    };

    ([$(($($component:tt)+))*] [$($cur:tt)*] $first:tt $($rest:tt)*) => {
        $crate::__tokenize_path!([$(($($component)+))*] [$($cur)* $first] $($rest)*)
    };

    ([$(($($component:tt)+))*] [$($cur:tt)+]) => {
        $crate::__tokenize_path!([$(($($component)+))* ($($cur)+)])
    };

    ([($($first:tt)+) ($($second:tt)+) $(($($rest:tt)+))*]) => {{
        let mut path = <_ as ::std::convert::AsRef<::std::path::Path>>::as_ref(
            &($($first)+)
        ).to_path_buf();
        path.push(
            <_ as ::std::convert::AsRef<::std::path::Path>>::as_ref(
                &($($second)+)
            )
        );
        $(
            path.push(
                <_ as ::std::convert::AsRef<::std::path::Path>>::as_ref(
                    &($($rest)+)
                )
            );
        )*
        path
    }};

    ([($($first:tt)+)]) => {{
        <_ as ::std::convert::AsRef<::std::path::Path>>::as_ref(
            &($($first)+)
        ).to_path_buf()
    }};

    ([]) => {{
        ::std::path::PathBuf::new()
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_path_macro() {
        use std::path::{Path, PathBuf};

        let p = path!("a" / "b" / "c");
        #[cfg(unix)]
        assert_eq!(p, Path::new(r"a/b/c"));
        #[cfg(windows)]
        assert_eq!(p, Path::new(r"a\b\c"));

        let p = path!("../a/b");
        assert_eq!(p, Path::new("../a/b"));

        let p = path!("../a/b" / "c" / "d");
        #[cfg(unix)]
        assert_eq!(p, Path::new(r"../a/b/c/d"));
        #[cfg(windows)]
        assert_eq!(p, Path::new(r"../a/b\c\d"));

        let p = path!(PathBuf::from("../a/b") / "c" / "d");
        #[cfg(unix)]
        assert_eq!(p, Path::new(r"../a/b/c/d"));
        #[cfg(windows)]
        assert_eq!(p, Path::new(r"../a/b\c\d"));

        let p = path!(Path::new("../a/b") / "c" / "d");
        #[cfg(unix)]
        assert_eq!(p, Path::new(r"../a/b/c/d"));
        #[cfg(windows)]
        assert_eq!(p, Path::new(r"../a/b\c\d"));

        let p = path!("x" / PathBuf::from("../a/b") / "c" / "d");
        #[cfg(unix)]
        assert_eq!(p, Path::new(r"x/../a/b/c/d"));
        #[cfg(windows)]
        assert_eq!(p, Path::new(r"x\../a/b\c\d"));

        let p = path!("x" / Path::new("../a/b") / "c" / "d");
        #[cfg(unix)]
        assert_eq!(p, Path::new(r"x/../a/b/c/d"));
        #[cfg(windows)]
        assert_eq!(p, Path::new(r"x\../a/b\c/d"));

        let p = path!("../a/b" / "c/d");
        #[cfg(unix)]
        assert_eq!(p, Path::new(r"../a/b/c/d"));
        #[cfg(windows)]
        assert_eq!(p, Path::new(r"../a/b\c/d"));
    }

    #[test]
    fn test_path_macro_variable_first_arg() {
        use std::path::{Path, PathBuf};

        let expected = PathBuf::from("a/b/c");

        let base: &str = "a";
        let p = path!(base / "b" / "c");
        #[cfg(unix)]
        assert_eq!(p, expected);

        let base: String = String::from("a");
        let p = path!(base / "b" / "c");
        #[cfg(unix)]
        assert_eq!(p, expected);

        let base: &Path = Path::new("a");
        let p = path!(base / "b" / "c");
        #[cfg(unix)]
        assert_eq!(p, expected);

        let base: PathBuf = PathBuf::from("a");
        let p = path!(base / "b" / "c");
        #[cfg(unix)]
        assert_eq!(p, expected);
    }

    #[test]
    fn test_path_macro_variable_successive_args() {
        use std::path::{Path, PathBuf};

        let expected = PathBuf::from("a/b/c");

        let component: String = String::from("b");
        let p = path!("a" / component / "c");
        #[cfg(unix)]
        assert_eq!(p, expected);

        let component: &str = "b";
        let p = path!("a" / component / "c");
        #[cfg(unix)]
        assert_eq!(p, expected);

        let component: &Path = Path::new("b");
        let p = path!("a" / component / "c");
        #[cfg(unix)]
        assert_eq!(p, expected);
    }

    #[test]
    fn test_path_macro_mixed_variables() {
        use std::path::{Path, PathBuf};

        let expected = PathBuf::from("a/b/c");

        let base: String = String::from("a");
        let mid: &Path = Path::new("b");
        let end: &str = "c";
        let p = path!(base / mid / end);
        #[cfg(unix)]
        assert_eq!(p, expected);
    }
}
