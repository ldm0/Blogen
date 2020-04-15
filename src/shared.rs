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
pub fn path_title(title: &str) -> String {
    // to lowercase and replace empty space to dash
    let mut path_title = String::new();
    for i in title.trim().to_ascii_lowercase().replace(" ", "-").chars() {
        match i {
            '~' | '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '{' | '}' | '|'
            | ':' | '"' | '<' | '>' | '?' | '[' | ']' | '\\' | ';' | '\'' | ',' | '.' | '/'
            | '=' => (),
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
