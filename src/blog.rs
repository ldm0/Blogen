use crate::blog_clusters::TagHandle;

fn valid_date(year: i64, month: i64, day: i64) -> bool {
    if year > 2200 || year < 2000 {
        return false;
    }
    let leap = (((year % 4) == 0) && (year % 100 != 0)) || ((year % 400) == 0);
    return match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => (day <= 31 && day >= 1),
        4 | 6 | 9 | 11 => (day <= 30 && day >= 1),
        2 => ((day <= if leap {29} else {28}) && (day >= 1)),
        _ => false,
    }
}

#[derive(Debug, Clone)]
pub struct Blog {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub title: String,
    pub tags: Vec<TagHandle>,
    pub preview: String,
    pub content: String, // reference to the blog content
}

impl Blog {
    pub fn new(year: i64, month: i64, day: i64, title: &str, tags: &Vec<TagHandle>, preview: &str, content: &str) -> Self {
        // This isn't a program for others, I would use it myself so I will panic whenever possible
        if !valid_date(year, month, day) {
            panic!("Blog's date is invalid!");
        }
        Blog {
            year: year as u16,
            month: month as u16,
            day: day as u16,
            title: title.to_string(),
            tags: tags.clone(),
            preview: preview.to_string(),
            content: content.to_string(),
        }
    }
}

#[cfg(test)]
mod blog_tests {
    use super::*;
    #[test]
    fn test_valid_date() {
        assert_eq!(true, valid_date(2000, 2, 29));
        assert_eq!(true, valid_date(2004, 2, 29));
        assert_eq!(true, valid_date(2019, 12, 15));

        assert_eq!(true, valid_date(2001, 2, 28));
        assert_eq!(true, valid_date(2002, 2, 28));
        assert_eq!(true, valid_date(2003, 2, 28));

        assert_eq!(true, valid_date(2003, 3, 31));

        assert_eq!(false, valid_date(2001, 2, 29));
        assert_eq!(false, valid_date(2002, 2, 29));
        assert_eq!(false, valid_date(2003, 2, 29));

        assert_eq!(false, valid_date(2003, 4, 31));

        assert_eq!(false, valid_date(1999, 1, 1));
        assert_eq!(false, valid_date(2099, 0, 1));
        assert_eq!(false, valid_date(2099, 1, 0));
    }
}