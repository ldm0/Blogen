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
    () => {"articles/"};
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

// Change the blog title to be campatible with webpage path
// Used for path/filename generation from blog.title
// Used for content title and file title consistency check
pub fn path_title(title: &str) -> String {
    // to lowercase and replace empty space to dash
    let mut path_title = String::new();
    for i in title.trim().to_ascii_lowercase().replace(" ", "-").chars() {
        match i {
            '-' | '0'..='9' | 'a'..='z' => path_title.push(i),
            _ => (),
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
    }
}