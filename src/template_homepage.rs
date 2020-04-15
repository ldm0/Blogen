use std::collections::HashMap;

use crate::blog_clusters::BlogClusters;
use crate::hlf_parser::{parse, HlfLhs, HlfRhs, Symbol};
use crate::shared::{path_title, HTMLTemplate};
use crate::tag::Tag; // for template filling

pub struct HomepageTemplate {
    hlfs: HashMap<HlfLhs, HlfRhs>,
}

impl HTMLTemplate for HomepageTemplate {
    fn load(template_raw: &str) -> Result<Self, String> {
        let hlfs_vec = match parse(&template_raw) {
            Some(x) => x,
            None => return Err("template parse failed".to_string()),
        };
        let mut hlfs = HashMap::new();
        for i in hlfs_vec.iter() {
            hlfs.insert(i.lhs.clone(), i.rhs.clone());
        }
        Ok(Self { hlfs: hlfs })
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
                Symbol::T(x) => result.push_str(
                    &x.replace("_slot_of_blog_path", &(path_title(&blog.title) + ".html"))
                        .replace("_slot_of_blog_title", &blog.title)
                        .replace("_slot_of_blog_preview", &blog.preview),
                ),
                _ => panic!(),
            }
            match tags_rhs.get(0).unwrap() {
                Symbol::T(x) => result.push_str(x),
                _ => panic!(),
            }
            let tags: Vec<&Tag> = blog
                .tags
                .iter()
                .map(|x| cluster.get_tag(*x).unwrap())
                .collect();
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
