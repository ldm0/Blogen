use crate::blog_clusters::BlogClusters;

// Implemented by templates
pub trait HTMLTemplate {
    // Try to load html template from string
    fn load(template_raw: &str) -> Result<Self, String>
    where
        Self: std::marker::Sized;

    // Return file name and file content
    fn fill(&self, blog_clusters: &BlogClusters) -> Vec<(String, String)>;
}

// Fit average blog titles in webpage path. Used for path/filename generation
// from blog.title and consistency check between content title and file title.
pub fn path_title<T: AsRef<str>>(title: T) -> String {
    // to lowercase and replace empty space to dash
    title
        .as_ref()
        .trim()
        .chars()
        .fold(String::new(), |mut path, ch| {
            let ch = if ch.is_ascii() {
                match ch {
                    '~' | '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '{' | '}'
                    | '|' | ':' | '"' | '<' | '>' | '?' | '[' | ']' | '\\' | ';' | '\'' | ','
                    | '.' | '/' | '=' => None,
                    'A'..='Z' => Some((ch as u8).to_ascii_lowercase() as char),
                    ' ' => Some('-'),
                    _ => Some(ch),
                }
            } else {
                Some(ch)
            };
            if let Some(ch) = ch {
                path.push(ch);
            }
            path
        })
}

#[cfg(test)]
mod shared_tests {
    use super::*;
    #[test]
    fn test_path_title() {
        assert_eq!("this-is-the-title", path_title("This iS The tiTle"));
        assert_eq!(
            "this-is-the-title",
            path_title("\n    This iS The tiTle  \n")
        );
        assert_eq!("this-is-the-title666", path_title("This iS The tiTle666"));
        assert_eq!(
            "this-is-the-title",
            path_title("\n   *!@#$%^&*()This iS The <>?,./;'[]\\tiTle  \n")
        );
        assert_eq!("中文测试", path_title("中文测试"));
        assert_eq!("烫烫烫", path_title("烫烫烫"));
        assert_eq!("-😄-", path_title("-😄-"));
    }
}
