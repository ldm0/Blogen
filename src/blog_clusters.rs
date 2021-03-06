use crate::blog::Blog;
use crate::shared::path_title;
use crate::tag::Tag;
use std::collections::HashMap;

use std::str;
use std::string::String;

// Currently just index of a blog or tag in vector
pub type BlogHandle = usize;
pub type TagHandle = usize;

// Construction procedure:
// BlogCluster construction
// Add tags
// Insert blogs(after tags were added because tags in metadata of articles needs validation)

// Squash time to comparable format
fn time_squash<T: Into<u64>>(year: T, month: T, day: T) -> u64 {
    year.into() * 1024 + month.into() * 64 + day.into()
}

pub struct BlogClusters {
    //blog_map: HashMap<String, BlogHandle>,              // currently not used
    tag_map: HashMap<String, TagHandle>, // map tag name to index of tag in tag vector
    tags: Vec<Tag>,                      // Follow the order of tags in tag file
    blogs: Vec<Blog>,                    // Follow the order of blog creation date
    tag_blog_map: HashMap<TagHandle, Vec<BlogHandle>>, // map tag handle to handles of blog referencing it
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
    fn add_tag(&mut self, tag_name: String, tag_desc: String) -> Option<TagHandle> {
        if self.tag_map.contains_key(&tag_name) {
            None
        } else {
            let tag_handle = self.tags.len();
            self.tags.push(Tag::new(tag_name.clone(), tag_desc));
            self.tag_map.insert(tag_name, tag_handle);
            Some(tag_handle)
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
        let lines: Vec<&str> = tags_raw.lines().map(|x| x.trim()).collect();
        for line in lines {
            if !line.is_empty() {
                let line = line.to_string();
                if name_found {
                    // actually this `.clone()` is just useless, rust lifetime
                    // analyzer is broken on complicated situation like this.
                    // :-/
                    match self.add_tag(tag_name.clone(), line) {
                        Some(tag_handle) => {
                            self.tag_blog_map.insert(tag_handle, Vec::new());
                        }
                        None => panic!("Duplicate tag name found."),
                    }
                } else {
                    tag_name = line;
                }
                name_found = !name_found;
            }
        }
    }

    // Should call add_tags before calling this.
    // blog_mds: blog filename and blog content in markdown with metadata
    // PS: blog_name is used for checking if the title in the file is corresponding
    pub fn add_blogs(&mut self, blog_mds: &[(String, String)]) {
        // Insert blogs to blog vector
        for (blog_path_title, blog) in blog_mds {
            let mut line_it = blog.lines();

            // First line is title
            let title = line_it.next().unwrap().trim();

            // We need to ensure title in content is roughly the same as file
            // name. The path_title is only used for validation, the title
            // stored is unprocessed.
            assert_eq!(
                &path_title(title),
                &path_title(blog_path_title),
                "Incorrespondence between filename and title"
            );

            // Second line is time: `2000 / 9 / 27`
            let time = line_it
                .next()
                .unwrap()
                .split('/')
                .map(|x| x.trim().parse().expect("Invalid time."))
                .collect::<Vec<i64>>();

            assert_eq!(3, time.len(), "Invalid time format.");

            // Third line is tags: `aaa | bbb | ccc | ...`
            let tag_names = line_it
                .next()
                .unwrap()
                .split('|')
                .map(|x| x.trim()) // permits arbitrary spaces between time and seperator
                .collect::<Vec<&str>>();
            let tag_handles = tag_names
                .into_iter()
                .map(|x| *self.get_tag_handle(x).expect("Invalid tag name."))
                .collect::<Vec<TagHandle>>();

            // Assme there is no "---" in article content.
            // Emmm.... This is not a legit assumption, we can change it later
            let mut parts = blog.split("---");

            // Assume there always three parts:
            // Meta Data
            // ---
            // Preview
            // ---
            // Content

            // Just ignore the metadata, because we have parsed it.
            parts.next().expect("Where is the meta data?");
            // Wrapping white spaces in preview and content is legal.
            // Get the preview part
            let preview = parts.next().expect("Where is the preview?").trim();
            // Get the content part
            let content = parts.next().expect("Where is the content?").trim();

            self.blogs.push(Blog::new(
                time[0],
                time[1],
                time[2],
                title.to_string(),
                tag_handles.clone(),
                preview.to_string(),
                content.to_string(),
            ));
        }

        // Sort blog vector by time, from new to old
        self.blogs.sort_by(|a, b| {
            time_squash(b.year, b.month, b.day).cmp(&time_squash(a.year, a.month, a.day))
        });

        // Map tag_handle-blog_handle pair
        for (i, blog) in self.blogs.iter().enumerate() {
            // Gen handle of current blog(just the index)
            let blog_handle = i;

            for tag_handle in &blog.tags {
                // We know this tag_handle is always valid
                self.tag_blog_map
                    .get_mut(&tag_handle)
                    .unwrap()
                    .push(blog_handle);
            }
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

    pub fn get_blogs(&self) -> &Vec<Blog> {
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
    fn test_time_squashing() {
        assert!(time_squash(2000u64, 9, 27) < time_squash(2001u64, 8, 13));
        assert!(time_squash(2000u64, 9, 27) > time_squash(1997u64, 5, 27));
    }

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
            programming techniques",
        );
        assert_eq!(5, clusters.num_tag());
    }

    #[test]
    #[should_panic]
    fn test_tag_duplication() {
        let mut clusters = BlogClusters::new();
        clusters.add_tags(
            "
            life
            things about current life
            life
            things about current life
        ",
        );
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
            maybe some gameplay",
        );

        clusters.add_blogs(&vec![(
            "test-blog".to_string(),
            "Test Blog
                2000/9/27
                life | work | fun
                ---
                lolololololol
                ---
                ololololololo
                "
            .to_string(),
        )]);
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
