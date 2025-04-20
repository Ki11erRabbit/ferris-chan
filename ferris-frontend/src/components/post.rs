use chrono::{DateTime, Local, Utc};
use leptos::{component, view, IntoView};
use leptos::control_flow::For;
use leptos::prelude::{ClassAttribute, Get, ReadSignal};
use leptos::prelude::ElementChild;
use ferris_shared::transfer::post::Post;

#[component]
pub fn Post(
    username: String,
    timestamp: i64,
    post_number: usize,
    post_text: String,
    post_image: String,
) -> impl IntoView {


    view! {
        <div class="post">
        <div class="post-header"><p>{username}</p><span> {DateTime::<Local>::from(DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap()).format("%x(%a)%H:%M:%S").to_string()}{format!(" No.{}", post_number)}</span></div>
        {if post_image.len() > 0 {
            Some(view! {<img src=format!("data:image/png;base64,{}", post_image) />})
        } else {
            None
        } }
        <p>{post_text}</p>
        </div>
    }
}

#[component]
pub fn PostList(
    get_posts: ReadSignal<Vec<Post>>
) -> impl IntoView {
    view! {
        <For
            each=move|| {
                get_posts.get()
            }
            key=|post| post.post_number
            let(post)
        >
            <Post
                username=post.username
                timestamp=post.timestamp
                post_number=post.post_number
                post_text=post.text
                post_image=post.image
            />
        </For>
    }
}