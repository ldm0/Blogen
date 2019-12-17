use comrak::{markdown_to_html, ComrakOptions};

use std::collections::HashMap;          // to store HLF 

use crate::BLOG_FOLDER;

use crate::blog_clusters::BlogClusters; // for template filling
use crate::blog::Blog;                  // for template filling
use crate::shared::path_title;          // for homepage template filling hyperlink
use crate::tag::Tag;                    // for template filling
use crate::hlf_parser::{HLF, HlfLhs, HlfRhs, Symbol, parse};
// use std::io;


// 1. Retrives the blogs into cluster
// 2. Parse the template file into HLF
// 3. Use the information in cluster to expand the HLF to get the webpage result
// For each kind of webpages the expand rules are different and hard-coded.
// The hard-coded rules could be wrote in files, which make this program data driven
// (But it's difficult and not practicle because demand always ugly and hard to be describled in a general way).

// Use a bnf like thing is a fancier expression of html snippet provider
// while symbol in content means this symbol can be repeated



pub trait HTMLTemplate {
    fn load(template_raw: &str) -> Self;
    // file name, and file content
    fn fill(&self, blog_clusters: &BlogClusters) -> Vec<(String, String)>;
}

pub struct ClusterTemplate {
    hlfs: HashMap<HlfLhs, HlfRhs>,
}

pub struct BlogTemplate {
    hlfs: HashMap<HlfLhs, HlfRhs>,
}

pub struct HomepageTemplate {
    hlfs: HashMap<HlfLhs, HlfRhs>,
}

impl HTMLTemplate for ClusterTemplate {
    fn load(template_raw: &str) -> Self {
        // if input invalid template, spits out 
        let hlfs_vec = parse(&template_raw).expect("template parse failed");
        let mut hlfs = HashMap::new();
        for i in hlfs_vec.iter() {
            hlfs.insert(i.lhs.clone(), i.rhs.clone());
        }
        Self {
            hlfs: hlfs,
        }
    }
    fn fill(&self, clusters: &BlogClusters) -> Vec<(String, String)> {
        let mut content = String::new();
        //let
        //for i in 
        // maybe shouldn't use replace because it's variable length
        // tmp = self.raw.replace("_slot_of_tags", );
        vec![("clusters.html".to_string(), content)]
    }
}

impl HTMLTemplate for BlogTemplate {
    fn load(template_raw: &str) -> Self {
        let hlfs_vec = parse(&template_raw).expect("template parse failed");
        let mut hlfs = HashMap::new();
        for i in hlfs_vec.iter() {
            hlfs.insert(i.lhs.clone(), i.rhs.clone());
        }
        Self {
            hlfs: hlfs,
        }
    }
    fn fill(&self, cluster: &BlogClusters) -> Vec<(String, String)> {
        let mut results = Vec::new();

        // We have the knowledge of blog template's structure
        let main_rhs = self.hlfs.get("main").expect("there should be a main symbol in blog template.");
        let tags_rhs = match main_rhs.get(1).unwrap() {
            Symbol::N(x) => self.hlfs.get(x).expect(&format!("\"{}\" symbol not found.", x)),
            _ => panic!(),
        };
        let tag_rhs = match tags_rhs.get(1).unwrap() {
            Symbol::N(x) => self.hlfs.get(x).expect(&format!("\"{}\" symbol not found.", x)),
            _ => panic!(),
        };
        assert_eq!(main_rhs.len(), 3);
        assert_eq!(tags_rhs.len(), 3);
        assert_eq!(tag_rhs.len(), 1);

        let blogs = cluster.get_blogs();
        for blog in blogs {
            let mut result = String::new();
            match main_rhs.get(0).unwrap() {
                Symbol::T(x) => {
                    let options = ComrakOptions::default();
                    let content = markdown_to_html(&blog.content, &options);
                    result.push_str(&x.replace("_slot_of_blog_title", &blog.title)
                                      .replace("_slot_of_blog_day", &blog.day.to_string())
                                      .replace("_slot_of_blog_month", &blog.month.to_string())
                                      .replace("_slot_of_blog_year", &blog.year.to_string())
                                      .replace("_slot_of_blog_preview", &blog.preview)
                                      .replace("_slot_of_blog_content", &content));
                }
                _ => panic!(),
            }; 
            match tags_rhs.get(0).unwrap() {
                Symbol::T(x) => result.push_str(x),
                _ => panic!(),
            }; 
            // add multiple tag names
            for tag_handle in blog.tags.iter() {
                match tag_rhs.get(0).unwrap() {
                    Symbol::T(x) => {
                        let tag = cluster.get_tag(*tag_handle).unwrap();
                        result.push_str(&x.replace("_slot_of_tag_name", &tag.name));
                    },
                    _ => panic!(),
                }; 
            }
            match tags_rhs.get(2).unwrap() {
                Symbol::T(x) => result.push_str(x),
                _ => panic!(),
            }; 
            match main_rhs.get(2).unwrap() {
                Symbol::T(x) => result.push_str(x),
                _ => panic!(),
            }
            results.push((format!("{}{}", &path_title(&blog.title), ".html"), result));
        }
        results
    }
}

impl HTMLTemplate for HomepageTemplate {
    fn load(template_raw: &str) -> Self {
        let hlfs_vec = parse(&template_raw).expect("template parse failed");
        let mut hlfs = HashMap::new();
        for i in hlfs_vec.iter() {
            hlfs.insert(i.lhs.clone(), i.rhs.clone());
        }
        Self {
            hlfs: hlfs,
        }
    }
    fn fill(&self, cluster: &BlogClusters) -> Vec<(String, String)> {
        let mut result = String::new();

        let main = self.hlfs.get("main").expect("main symbol not found");
        let blog_chunk_rhs = match main.get(1).unwrap() {
            Symbol::N(x) => self.hlfs.get(x).expect(&format!("{} symbol not found.", x)),
            _ => panic!(),
        };
        let tags_rhs = match blog_chunk_rhs.get(1).unwrap() {
            Symbol::N(x) => self.hlfs.get(x).expect(&format!("{} symbol not found.", x)),
            _ => panic!(),
        };
        let tag_rhs = match tags_rhs.get(1).unwrap() {
            Symbol::N(x) => self.hlfs.get(x).expect(&format!("{} symbol not found.", x)),
            _ => panic!(),
        };
        match main.get(0).unwrap() {
            Symbol::T(x) => result.push_str(x),
            _ => panic!(),
        }; 

        let blogs = cluster.get_blogs();
        for blog in blogs {
            match blog_chunk_rhs.get(0).unwrap() {
                Symbol::T(x) => result.push_str(&x.replace("_slot_of_blog_path", &(BLOG_FOLDER!(&path_title(&blog.title)) + ".html"))
                                                  .replace("_slot_of_blog_title", &blog.title)
                                                  .replace("_slot_of_blog_preview", &blog.preview)),
                _ => panic!(),
            } 
            match tags_rhs.get(0).unwrap() {
                Symbol::T(x) => result.push_str(x),
                _ => panic!(),
            } 
            let tags: Vec<&Tag> = blog.tags.iter().map(|x| cluster.get_tag(*x).unwrap()).collect();
            for tag in tags {
                match tag_rhs.get(0).unwrap() {
                    Symbol::T(x) => result.push_str(&x.replace("_slot_of_tag_name", &tag.name)),
                    _ => panic!(),
                }
            }
            match tags_rhs.get(2).unwrap() {
                Symbol::T(x) => result.push_str(x),
                _ => panic!(),
            } 
            match blog_chunk_rhs.get(2).unwrap() {
                Symbol::T(x) => result.push_str(x),
                _ => panic!(),
            } 
        }
        match main.get(2).unwrap() {
            Symbol::T(x) => result.push_str(x),
            _ => panic!(),
        } 

        vec![("index.html".to_string(), result)]
    }
}
