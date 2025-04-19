use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetPostsRequest {
    pub board: String,
    pub category: String,
    pub offset: usize,
    pub count: usize,
}

#[derive(Deserialize, Serialize)]
pub struct GetPostsResponse {
    posts: Vec<Post>,
}

impl GetPostsResponse {
    pub fn new(posts: Vec<Post>) -> Self {
        Self { posts }
    }
}

impl Default for GetPostsResponse {
    fn default() -> Self {
        Self {
            posts: vec![
                Post {
                    username: String::from("Anonymous"),
                    image: String::new(),
                    text: String::from("test"),
                    unix_timestamp: 0,
                    post_number: 0,
                }
            ]
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GetPostReplyRequest {
    pub parent: usize,
    pub count: usize,
    pub offset: usize,
}

#[derive(Deserialize, Serialize)]
pub struct GetPostReplyResponse {
    posts: Vec<Post>,
}

impl GetPostReplyResponse {
    pub fn new(posts: Vec<Post>) -> Self {
        Self { posts }
    }
}

impl Default for GetPostReplyResponse {
    fn default() -> Self {
        Self {
            posts: vec![
                Post {
                    username: String::from("Anonymous"),
                    image: String::new(),
                    text: String::from("idiot"),
                    unix_timestamp: 0,
                    post_number: 1,
                }
            ]
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreatePostRequest {
    pub board: String,
    pub category: String,
    pub image: String,
    pub text: String,
    #[serde(default)]
    pub auth_token: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CreatePostResponse {
    post: Post,
}

impl CreatePostResponse {
    pub fn new(post: Post) -> Self {
        Self { post }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreatePostReplyRequest {
    pub board: String,
    pub category: String,
    pub image: String,
    pub text: String,
    pub parent: i64,
    #[serde(default)]
    pub auth_token: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CreatePostReplyResponse {
    post: Post,
}

#[derive(Deserialize, Serialize)]
pub struct Post {
    pub username: String,
    pub image: String,
    pub text: String,
    pub unix_timestamp: usize,
    pub post_number: usize,
}
