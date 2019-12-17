/**
 * Auto matically convert raw markdown blogs to my serveral blog web pages
 */

mod tag;
mod blog;
mod blog_clusters;
mod template;
mod hlf_parser;
mod shared;

use blog_clusters::BlogClusters;
use template::{BlogTemplate, HomepageTemplate, ClusterTemplate, HTMLTemplate};

use std::fs;                    // for directory iteration, template read, result write

fn get_blog_mds() -> Vec<(String, String)> {
    let blog_subdirs = fs::read_dir(BLOG_PATH!())
        .expect(&format!("read blog directory: {} failed.", BLOG_PATH!()));

    let blog_markdown_names: Vec<String> 
        = blog_subdirs.map(|x| x.unwrap().file_name().into_string().unwrap()).collect();

    let blog_markdown_paths: Vec<String>
        = blog_markdown_names.iter().map(|x| BLOG_PATH!(x)).collect();

    // return names zip with contents
    let blogs: Vec<(String, String)>
        = blog_markdown_names.iter().cloned().map(|mut x| {x.truncate(x.len() - 3); x})
                             .zip(blog_markdown_paths.iter().map(|x| fs::read_to_string(x).unwrap()))
                             .collect();
    blogs
}

fn main() {
    let homepage_template_raw   = fs::read_to_string(ASSET_PATH!("template_homepage.html")).unwrap();
    let blog_template_raw       = fs::read_to_string(ASSET_PATH!("template_blog.html")).unwrap();
    let cluster_template_raw    = fs::read_to_string(ASSET_PATH!("template_cluster.html")).unwrap();

    let homepage_template   : HomepageTemplate    = HTMLTemplate::load(&homepage_template_raw);
    let blog_template       : BlogTemplate        = HTMLTemplate::load(&blog_template_raw);
    let cluster_template    : ClusterTemplate     = HTMLTemplate::load(&cluster_template_raw);

    let tags: String = fs::read_to_string(&ASSET_PATH!("tags.txt")).unwrap();
    let blog_mds: Vec<(String, String)> = get_blog_mds();

    let mut blog_clusters = BlogClusters::new();
    blog_clusters.add_tags(&tags);
    blog_clusters.add_blogs(&blog_mds);

    let blog_html_result: Vec<(String, String)> = blog_template.fill(&blog_clusters);
    let cluster_html_result: Vec<(String, String)> = cluster_template.fill(&blog_clusters);
    assert_eq!(cluster_html_result.len(), 1);
    let homepage_html_result: Vec<(String, String)> = homepage_template.fill(&blog_clusters);
    assert_eq!(homepage_html_result.len(), 1);

    match fs::create_dir_all(OUTPUT_PATH!(BLOG_FOLDER!())) {
        Ok(_) => println!("Create direcotry \"{}\" if not exist.", OUTPUT_PATH!(BLOG_FOLDER!())),
        Err(err) => println!("Create directory failed: {}.", err),
    }
    match fs::create_dir_all(OUTPUT_PATH!(HOMEPAGE_FOLDER!())) {
        Ok(_) => println!("Create direcotry \"{}\" if not exist.", OUTPUT_PATH!(HOMEPAGE_FOLDER!())),
        Err(err) => println!("Create directory failed: {}.", err),
    }

    for (file_name, file_content) in blog_html_result {
        let path = OUTPUT_PATH!(&BLOG_FOLDER!(&file_name));
        match fs::write(&path, file_content) {
            Ok(_) => {
                println!("Output to \"{}\" ok.", &path);
            }
            Err(err) => {
                panic!(format!("Write to \"{}\" failed: {}.", &path, err));
            }
        }
    }
    for (file_name, file_content) in homepage_html_result {
        let path = OUTPUT_PATH!(&HOMEPAGE_FOLDER!(&file_name));
        match fs::write(&path, file_content) {
            Ok(_) => {
                println!("Output to \"{}\" ok.", &path);
            }
            Err(err) => {
                panic!(format!("Write to \"{}\" failed: {}.", &path, err));
            }
        }
    }
}

