use std::collections::HashMap;
use crate::tag::Tag;
use crate::blog::Blog;
use crate::shared::path_title;

use std::string::String;
use std::str;

pub type BlogHandle = usize;
pub type TagHandle = usize;

// Construction procedure:
// BlogCluster construction
// Add tags
// Insert blogs(after tags were added because tags in metadata of articles needs validation)


pub struct BlogClusters {
    //blog_map: HashMap<String, BlogHandle>, currently now used
    tag_map: HashMap<String, TagHandle>,
    tags: Vec<Tag>,
    blogs: Vec<Blog>,
    tag_blog_map: HashMap<TagHandle, Vec<BlogHandle>>,
}

impl BlogClusters {
    pub fn new() -> BlogClusters {
        BlogClusters {
            tag_map: HashMap::new(),
            tags: Vec::new(),
            blogs: Vec::new(),
            tag_blog_map: HashMap::new(),
        }
    }

    // used when parse tag file
    // return if add successfully 
    // when tag_name is present, description is not updated
    fn add_tag(&mut self, tag_name: &str, tag_desc: &str) -> bool {
        if self.tag_map.contains_key(tag_name) {
            false
        } else {
            self.tags.push(Tag::new(tag_name, tag_desc));
            self.tag_map.insert(tag_name.to_string(), self.tags.len() - 1);
            true
        }
    }

    // Assume there is a overall tags file
    // contains things like this:
    // ```
    // tagname
    // description
    // (serveral no letter lines)
    // tagname
    // description
    // (serveral no letter lines)
    // tagname
    // description
    // ...
    // (serveral no letter lines)
    // tagname
    // description
    // ```
    pub fn add_tags(&mut self, tags_raw: &str) {
        let mut name_found = false;
        let mut tag_name = String::new();
        let lines: Vec<&str> = tags_raw.lines()
                                    .map(|x| x.trim())
                                    .collect();
        for line in lines {
            if !line.is_empty() {
                if name_found {
                    if !self.add_tag(&tag_name, &line) {
                        panic!("Duplicate tag name found");
                    }
                    name_found = false;
                } else {
                    tag_name = line.to_string();
                    name_found = true;
                }
            }
        }
    }

    // Should call add_tags before calling this.
    // blog_mds: blog filename and blog content in markdown with metadata
    pub fn add_blogs(&mut self, blog_mds: &[(String, String)]) {
        // blog_name is used for checking if the title in the file is corresponding
        for (blog_path_title, blog) in blog_mds {
            let mut line_it = blog.lines();
            // First line is title
            let title = line_it.next().unwrap().trim();
            // We need to ensure title in content is roughly the same as file name
            // the path_title is only used for validation, the title stored is unprocessed.
            assert_eq!(&path_title(title), &path_title(blog_path_title), "filname need to correspond to the article title");

            // Second line is time: `2000/9/27`
            let time: Vec<i64>          = line_it.next().unwrap()
                                                .split('/')
                                                .map(|x| x.trim().parse().expect("Time is not valid"))
                                                .collect();

            // Third line is tags: `aaa |  bbb |ccc`
            let tag_names: Vec<&str>    = line_it.next().unwrap()
                                                .split('|')
                                                .map(|x| x.trim())
                                                .collect();
            let tags: Vec<TagHandle>    = tag_names.into_iter()
                                                .map(|x| *self.get_tag_handle(x).expect("tag name not present in tag file"))
                                                .collect();

            // Assme there is no "---" in article content.
            // Emmm.... This is not a legit assumption, we can change it later
            let mut parts = blog.split("---");

            // Assume there always three parts: 
            // meta data
            // ---
            // preview
            // ---
            // content

            // Just ignore the metadata, because we have parsed it.
            parts.next().expect("where is the meta data?");
            // Allow wrapping white spaces in preview and content.
            // Get the preview part
            let preview = parts.next().expect("where is the meta data?").trim();
            // Get the content part
            let content = parts.next().expect("where is the content?").trim();

            self.blogs.push(Blog::new(time[0], time[1], time[2], &title, &tags, preview, content))
        }
    }

    fn get_tag_handle(&self, tag_name: &str) -> Option<&TagHandle> {
        self.tag_map.get(tag_name)
    }

    pub fn get_tag(&self, tag_handle: TagHandle) -> Option<&Tag> {
        self.tags.get(tag_handle)
    }

    pub fn get_tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn get_blogs(&self) -> &Vec<Blog>{
        &self.blogs
    }

    pub fn num_blog(&self) -> usize {
        self.blogs.len()
    }

    pub fn num_tag(&self) -> usize {
        self.tags.len()
    }
}

#[cfg(test)]
mod blog_cluster_tests {
    use super::*;

    #[test]
    fn test_tag_parsing() {
        let mut clusters = BlogClusters::new();
        clusters.add_tags(
            "life
            things about current life

            work
            about my works


            fun
            maybe some gameplay
            tech
            be geek about hardware and software

            proramming
            programming techniques");
        assert_eq!(5, clusters.num_tag());
    }

    #[test]
    #[should_panic]
    fn test_tag_duplication() {
        let mut clusters = BlogClusters::new();
        clusters.add_tags("
            life
            things about current life
            life
            things about current life
        ");
    }

    #[test]
    fn test_blog_adding() {
        let mut clusters = BlogClusters::new();
        clusters.add_tags(
            "life
            things about current life

            work
            about my works

            fun
            maybe some gameplay");
            
        clusters.add_blogs(&vec![
            (
                "test-blog".to_string(),
                "Test Blog
                2000/9/27
                life | work | fun
                ---
                lolololololol
                ---
                ololololololo
                ".to_string())
        ]);
        let blogs = clusters.get_blogs();
        assert_eq!(blogs.len(), 1);
        let blog = &blogs[0];
        assert_eq!(blog.year, 2000);
        assert_eq!(blog.month, 9);
        assert_eq!(blog.day, 27);
        assert_eq!(blog.tags.len(), 3);
        assert_eq!(blog.title, "Test Blog");
        assert_eq!(blog.preview, "lolololololol");
        assert_eq!(blog.content, "ololololololo");
    }
}