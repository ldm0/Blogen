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

// for directory iteration, template read, result write
use std::fs;
use std::env;
use dotenv;

fn get_blog_mds(blog_path: &str) -> Vec<(String, String)> {
    let blog_subdirs = fs::read_dir(blog_path)
        .expect(&format!("read blog directory: {} failed.", blog_path));

    let blog_markdown_names: Vec<String>
        = blog_subdirs.map(|x| x.unwrap().file_name().into_string().unwrap()).collect();

    let blog_markdown_paths: Vec<String>
        = blog_markdown_names.iter().map(|x| blog_path.to_string() + x).collect();

    // Return filenames zipped with contents
    let blogs: Vec<(String, String)>
        = blog_markdown_names.iter().cloned()
                            .map(|mut x| {
                                // length -3 to remove ".md"
                                x.truncate(x.len() - 3);
                                x
                            })
                            .zip(blog_markdown_paths.iter().map(|x| fs::read_to_string(x).unwrap()))
                            .collect();
    blogs
}

fn main() {
    dotenv::dotenv().ok();
    let tags_path               = env::var("TAGS_PATH")
                                    .expect("Please specify tags path in environment variable.");
    let homepage_template_path  = env::var("TEMPLATE_HOMEPAGE_PATH")
                                    .expect("Please specify homepage template path in environment variable.");
    let blog_template_path      = env::var("TEMPLATE_BLOG_PATH")
                                    .expect("Please specify blog template path in environment variable.");
    let cluster_template_path   = env::var("TEMPLATE_CLUSTER_PATH")
                                    .expect("Please specify cluster template path in environment variable.");
    let output_path             = env::var("OUTPUT_PATH")
                                    .expect("Please specify output path in environment variable.");
    let blog_path               = env::var("BLOG_PATH")
                                    .expect("Please specify blog path in environment variable.");

    let homepage_template_raw   = fs::read_to_string(&homepage_template_path)
                                    .expect("homepage template not found!");
    let blog_template_raw       = fs::read_to_string(&blog_template_path)
                                    .expect("blog template not found!");
    let cluster_template_raw    = fs::read_to_string(&cluster_template_path)
                                    .expect("cluster template not found!");

    let homepage_template   : HomepageTemplate    = HTMLTemplate::load(&homepage_template_raw).unwrap();
    let blog_template       : BlogTemplate        = HTMLTemplate::load(&blog_template_raw).unwrap();
    let cluster_template    : ClusterTemplate     = HTMLTemplate::load(&cluster_template_raw).unwrap();

    let tags: String = fs::read_to_string(&tags_path)
                            .expect("failed to read tags.");
    let blog_mds: Vec<(String, String)> = get_blog_mds(&blog_path);

    let mut blog_clusters = BlogClusters::new();
    blog_clusters.add_tags(&tags);
    blog_clusters.add_blogs(&blog_mds);

    let blog_html_result    : Vec<(String, String)> = blog_template.fill(&blog_clusters);
    let cluster_html_result : Vec<(String, String)> = cluster_template.fill(&blog_clusters);
    let homepage_html_result: Vec<(String, String)> = homepage_template.fill(&blog_clusters);
    assert_eq!(cluster_html_result.len(), 1);
    assert_eq!(homepage_html_result.len(), 1);

    match fs::create_dir_all(&output_path) {
        Ok(_) => println!("Create direcotry \"{}\" if not exist.", &output_path),
        Err(err) => println!("Create directory failed: {}.", err),
    }

    for (file_name, file_content) in blog_html_result {
        let path = output_path.clone() + &file_name;
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
        let path = output_path.clone() + &file_name;
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

