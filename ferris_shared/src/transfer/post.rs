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
                    alt_text: String::new(),
                    text: String::from("test"),
                    timestamp: 0,
                    post_number: 0,
                    parent: 0
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
    pub posts: Vec<Post>,
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
                    alt_text: String::new(),
                    text: String::from("idiot"),
                    timestamp: 0,
                    post_number: 1,
                    parent: 0
                }
            ]
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePostRequest {
    pub board: String,
    pub category: String,
    pub image: String,
    pub alt_text: String,
    pub text: String,
    #[serde(default)]
    pub auth_token: Option<String>,
}

impl CreatePostRequest {
    pub fn new(board: String, category: String, image: String, alt_text: String, text: String, auth_token: Option<String>) -> Self {
        Self {
            board,
            category,
            image,
            alt_text,
            text,
            auth_token,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatePostResponse {
    Success {
        post: Post,
    },
    Error {
        message: String,
    }
}

impl CreatePostResponse {
    pub fn new(post: Post) -> Self {
        Self::Success { post }
    }

    pub fn error<M: AsRef<str>>(message: M) -> Self {
        Self::Error { message: message.as_ref().to_string() }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreatePostReplyRequest {
    pub board: String,
    pub category: String,
    pub image: String,
    pub alt_text: String,
    pub text: String,
    pub parent: i64,
    #[serde(default)]
    pub auth_token: Option<String>,
}

impl CreatePostReplyRequest {
    pub fn new(board: String, category: String, image: String, alt_text: String, text: String, parent: i64, auth_token: Option<String>) -> Self {
        Self {
            board,
            category,
            image,
            alt_text,
            text,
            parent,
            auth_token,
        }
    }
}


#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatePostReplyResponse {
    Success {
        post: Post,
    },
    Error {
        message: String,
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Post {
    pub username: String,
    pub image: String,
    pub alt_text: String,
    pub text: String,
    pub timestamp: i64,
    pub post_number: usize,
    pub parent: i64,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            username: String::new(),
            image: String::new(),
            alt_text: String::new(),
            text: String::new(),
            timestamp: 0,
            post_number: 0,
            parent: 0,
        }
    }
}