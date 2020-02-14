#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Tag {
    pub name: String,
    pub description: String,
}

impl Tag {
    pub fn new(name: String, description: String) -> Tag {
        Tag {
            name: name,
            description: description,
        }
    }
}
