use std::collections::HashMap;

use crate::blog_clusters::BlogClusters;
use crate::hlf_parser::{parse, HlfLhs, HlfRhs, Symbol};
use crate::shared::HTMLTemplate;

pub struct ClusterTemplate {
    hlfs: HashMap<HlfLhs, HlfRhs>,
}

impl HTMLTemplate for ClusterTemplate {
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
    fn fill(&self, clusters: &BlogClusters) -> Vec<(String, String)> {
        let mut content = String::new();
        //let
        //for i in
        // maybe shouldn't use replace because it's variable length
        // tmp = self.raw.replace("_slot_of_tags", );
        vec![("clusters.html".to_string(), content)]
    }
}
