use std::collections::HashMap;

use super::sqlite_con::*;
use super::types::*;
use super::utils::*;

#[derive(Debug)]
struct NodeWrapper {
    node: Node,
    links: Vec<usize>,
}

impl NodeWrapper {
    pub fn new(node: Node) -> Self {
        let links: Vec<usize> = [0; 0].to_vec();
        NodeWrapper {
            node,
            links,
        }
    }
    
    pub fn add_link(&mut self, link: usize) {
        self.links.push(link);
    }
}


pub fn count_links_per_node(nodes: Vec<Node>, links: &Vec<Link>) {
    let quote: char = "\"".chars().next().unwrap() ;
    let mut map: HashMap<String, NodeWrapper> = HashMap::with_capacity(10);
    // for l in links {
    //     println!("{:?}", l);
    // }
    for n in nodes {
        let clean_id = clean_id(n.id);

        map.insert(clean_id, NodeWrapper::new(n));
        // println!("{:?}", n.id.bytes());
    }
    for (i, l) in links.iter().enumerate() {
        let src = clean_id(l.source.clone());
        let dest = clean_id(l.dest.clone());
        map.get(&src.clone()).unwrap().add_link(i);
        map.get(&dest).unwrap().add_link(i);
    }
    for (k, v) in map.iter() {
        println!("{:?}", v);
    }
}

