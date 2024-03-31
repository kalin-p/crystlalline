use sqlx::sqlite::SqlitePool;
use super::types::*;

pub const DB_PATH: &str = "sqlite:///home/kalin/.emacs.d/org-roam.db";
pub const DEV_DB_PATH: &str = "sqlite:///data/graph_viz/src/orgroam/dev-org-roam.db";

pub async fn get_all_links(pool: &SqlitePool) -> Vec<Link> {
    // let path = Path::new(path);
    let links: Vec<Link> = sqlx::query_as!(
        Link,
        r#"select * from links"#)
        .fetch_all(pool).await.unwrap();
    links
}

pub async fn get_all_nodes(pool: &SqlitePool) -> Vec<Node> {
    let nodes: Vec<Node> = sqlx::query_as!(
        Node,
        r#"select * from nodes"#)
        .fetch_all(pool).await.unwrap();
    nodes
}

pub async fn get_all_refs(pool: &SqlitePool) -> Vec<Ref> {
    let refs: Vec<Ref> = sqlx::query_as!(
        Ref,
        r#"select * from refs"#)
        .fetch_all(pool).await.unwrap();
    refs
}

pub async fn get_all_tags(pool: &SqlitePool) -> Vec<Tag> {
    let tags: Vec<Tag> = sqlx::query_as!(
        Tag,
        r#"select * from tags"#)
        .fetch_all(pool).await.unwrap();
    tags
}

pub async fn get_all_citations(pool: &SqlitePool) -> Vec<Citation> {
    let citations: Vec<Citation> = sqlx::query_as!(
        Citation,
        r#"select * from citations"#)
        .fetch_all(pool).await.unwrap();
    citations
}

pub async fn get_all_aliases(pool: &SqlitePool) -> Vec<Alias> {
    let aliases: Vec<Alias> = sqlx::query_as!(
        Alias,
        r#"select * from aliases"#)
        .fetch_all(pool).await.unwrap();
    aliases
}



