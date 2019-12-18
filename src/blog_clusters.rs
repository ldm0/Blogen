use std::collections::HashMap;
use crate::tag::Tag;
use crate::blog::Blog;
use crate::shared::path_title;

use std::string::String;
use std::str;

pub type BlogHandle = usize;
pub type TagHandle = usize;

// new
// add tags
// add blogs
pub struct BlogClusters {
    //blog_map: HashMap<String, BlogHandle>, currently now used
    tag_map: HashMap<String, TagHandle>,
    tags: Vec<Tag>,
    blogs: Vec<Blog>,
    tag_blog_map: HashMap<TagHandle, Vec<BlogHandle>>,
}

impl BlogClusters {
    /**
     * Create tag pool and blog pool, and link them to a instance of BlogClusters.
     * Insert Blog to clusters, it will help you manage the two pools.
     */
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
    // contains things like this 
    // tagname
    // description
    // tagname
    // description
    // ...
    pub fn add_tags(&mut self, tags_raw: &str) {
        let lines: Vec<&str> = tags_raw.lines().map(|x| x.trim()).collect();
        let num_tags = lines.len() / 2;
        for i in 0..num_tags {
            if !self.add_tag(lines[2 * i], lines[2 * i + 1]) {
                panic!("duplicate tag name find");
            }
        }
    }


    // Should call add_tags before this is called
    pub fn add_blogs(&mut self, blog_mds: &[(String, String)]) {
        // blog filename and blog content in markdown with metadata
        // blog_name is used for validate if the title in file is corresponding to title 
        for (blog_path_title, blog) in blog_mds {
            let mut line_it = blog.lines();
            // assume first line is time: this is the title!
            let title = line_it.next().unwrap().trim();
            // We need to ensure title in content roughly the same as file name
            assert_eq!(&path_title(title), &path_title(blog_path_title), "filname need to correspond to title in file content");
            // assume second line is time: 2000/9/27
            let time: Vec<i64> = line_it.next().unwrap().split('/').map(|x| x.trim().parse().expect("Time is not valid")).collect();
            // assume third line is tags: aaa|bbb|ccc
            let tag_names: Vec<&str> = line_it.next().unwrap().split('|').map(|x| x.trim()).collect();
            let tags: Vec<TagHandle> = 
                tag_names.into_iter()
                .map(|x| *self.get_tag_handle(x).expect("tag name not present in tag file")).collect();

            // we assme there is no "---" in the content
            // emmm.... that's not a legit assumption, but currently just do quick and dirty things
            let mut parts = blog.split("---");

            // Assume there always three parts: 
            // meta data
            // ---
            // preview
            // ---
            // content

            // Just ignore the metadata, because we have parsed it
            parts.next().expect("where is the meta data?");
            // Allow prefixed and trailing space and in preview and content
            // get the preview part
            let preview = parts.next().expect("where is the meta data?").trim();
            // get the content part
            let content = parts.next().expect("where is the content?").trim();

            self.blogs.push(Blog::new(time[0], time[1], time[2], &title, &tags, preview, content))
        }
    }

    // wait what if use get_tag and pop?
    // may be tag of this struct are considered borrowed and cannot be used any longer
    // we should be cautious
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
    fn test_all() {
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
}