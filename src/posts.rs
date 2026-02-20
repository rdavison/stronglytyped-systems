#[derive(Clone, Debug)]
pub struct Post {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
    pub html: &'static str,
}

static POSTS: &[Post] = include!(concat!(env!("OUT_DIR"), "/posts_data.rs"));

pub fn get_posts() -> Vec<Post> {
    POSTS.to_vec()
}

pub fn get_post_by_slug(slug: &str) -> Option<Post> {
    POSTS.iter().find(|p| p.slug == slug).cloned()
}
