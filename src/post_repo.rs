use crate::model::Post;
use r2d2::Pool;
use redis::Client;

pub struct PostRepository {
    pool: Pool<Client>,
}

impl PostRepository {
    pub fn new(pool:Pool<Client>) -> PostRepository {
        PostRepository{pool}
    }

    pub fn get(&self, id:i64) -> Option<Post>{
        Some(Post::new(0, "hoge", "fuga"))
    }

    pub fn delete(&self, id:i64) {
        // no action
    }
}