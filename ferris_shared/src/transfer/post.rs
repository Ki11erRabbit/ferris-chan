use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct GetPostsRequest {
    pub board: String,
    pub category: String,
    pub offset: usize,
    pub count: usize,
}

impl GetPostsRequest {
    pub fn new(board: String, category: String, count: usize, offset: usize) -> Self {
        Self {
            board,
            category,
            count,
            offset,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetPostsResponse {
    pub posts: Vec<Post>,
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
                    timestamp: 0,
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
                    timestamp: 0,
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

#[derive(Deserialize, Serialize, Clone)]
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

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Post {
    pub username: String,
    pub image: String,
    pub text: String,
    pub timestamp: i64,
    pub post_number: usize,
}
