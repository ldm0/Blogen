#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Tag {
    pub name: String,
    pub description: String,
}

impl Tag {
    pub fn new(name: &str, description: &str) -> Tag {
        Tag {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}
