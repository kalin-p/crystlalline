// Each struct models a row in a table, quite obvious I guess but cannot hurt to
// say
#[derive(Debug)]
pub struct Link {
    pub pos: i64,
    pub source: String,
    pub dest: String,
    pub r#type: String,
    pub properties: String,
}

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub file: String,
    pub level: i64,
    pub pos: i64,
    pub todo: Option<String>,
    pub priority: Option<String>,
    pub scheduled: Option<String>,
    pub deadline: Option<String>,
    pub title: Option<String>,
    pub properties: Option<String>,
    pub olp: Option<String>,
}

#[derive(Debug)]
pub struct Ref {
    pub node_id: String,
    pub r#ref: String,
    pub r#type: String,
}

#[derive(Debug)]
pub struct Tag {
    pub node_id: String,
    pub tag: Option<String>,
}

#[derive(Debug)]
pub struct Citation {
    pub node_id: String,
    pub cite_key: String,
    pub pos: i64,
    pub properties: Option<String>,
}

#[derive(Debug)]
pub struct Alias {
    pub node_id: String,
    pub alias: Option<String>,
}