/* Use paths in environment args rather than hard-code path
#[macro_export]
macro_rules! ASSET_PATH {
    () => {"assets/"};
    ($x:literal) => {concat!(ASSET_PATH!(), $x)};
    ($x:expr) => {ASSET_PATH!().to_string() + $x};
}

#[macro_export]
macro_rules! BLOG_PATH {
    () => {"blogs/"};
    ($x:literal) => {concat!(BLOG_PATH!(), $x)};
    ($x:expr) => {BLOG_PATH!().to_string() + $x};
}

#[macro_export]
macro_rules! BLOG_FOLDER {
    () => {""};
    ($x:literal) => {concat!(BLOG_FOLDER!(), $x)};
    ($x:expr) => {BLOG_FOLDER!().to_string() + $x};
}

#[macro_export]
macro_rules! HOMEPAGE_FOLDER {
    () => {""};
    ($x:literal) => {concat!(HOMEPAGE_FOLDER!(), $x)};
    ($x:expr) => {HOMEPAGE_FOLDER!().to_string() + $x};
}

#[macro_export]
macro_rules! OUTPUT_PATH {
    () => {"output/"};
    ($x:literal) => {concat!(HOMEPAGE_OUTPUT_PATH!(), $x)};
    ($x:expr) => {OUTPUT_PATH!().to_string() + $x};
}
*/

// Fit average blog titles in webpage path. Used for path/filename generation
// from blog.title and consistency check between content title and file title.
pub fn path_title(title: &str) -> String {
    // to lowercase and replace empty space to dash
    let mut path_title = String::new();
    for i in title.trim().to_ascii_lowercase().replace(" ", "-").chars() {
        match i {
            '~' | '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' |
            '{' | '}' | '|' | ':' | '"' | '<' | '>' | '?' |
            '[' | ']' | '\\' | ';' | '\'' | ',' | '.' | '/' |
            '=' => (),
            _ => path_title.push(i),
        }
    }
    path_title
}

#[cfg(test)]
mod shared_tests {
    use super::*;
    #[test]
    fn test_path_title() {
        assert_eq!("this-is-the-title", path_title("This iS The tiTle"));
        assert_eq!("this-is-the-title", path_title("\n    This iS The tiTle  \n"));
        assert_eq!("this-is-the-title666", path_title("This iS The tiTle666"));
        assert_eq!("this-is-the-title", path_title("\n   *!@#$%^&*()This iS The <>?,./;'[]\\tiTle  \n"));
        assert_eq!("中文测试", path_title("中文测试"));
        assert_eq!("烫烫烫", path_title("烫烫烫"));
        assert_eq!("-😄-", path_title("-😄-"));
    }
}