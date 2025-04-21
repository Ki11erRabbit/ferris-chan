use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use chrono::{DateTime, Local, Utc};
use leptos::{component, view, IntoView};
use leptos::callback::Callback;
use leptos::control_flow::For;
use leptos::either::Either;
use leptos::prelude::{signal, ClassAttribute, Get, ReadSignal, Resource, Set, Suspend, Suspense, Write};
use leptos::prelude::ElementChild;
use ferris_shared::transfer::post::{GetPostsResponse, Post};
use crate::api;


#[component]
pub fn PostCore(
    username: String,
    timestamp: i64,
    post_number: usize,
    post_text: String,
    post_image: String,
) -> impl IntoView {
    view! {
        <div class="post-header"><p>{username}</p><span> {DateTime::<Local>::from(DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap()).format("%x(%a)%H:%M:%S").to_string()}{format!(" No.{}", post_number)}</span></div>
        <div class="post-content">
        <div class="post-image">
        {if post_image.len() > 0 {
            let bytes = BASE64_STANDARD.decode(post_image.as_bytes()).unwrap();
            let size_kb = bytes.len() / 1024;

            if post_image.starts_with("iVBORw0KGgo") {
                Some(view! {<img src=format!("data:image/png;base64,{}", post_image) /> <span>{format!("{size_kb} KB PNG")}</span>})
            } else if post_image.starts_with("/9") {
                Some(view! {<img src=format!("data:image/jpg;base64,{}", post_image) /> <span>{format!("{size_kb} KB JPG")}</span>})
            } else if post_image.starts_with("UklGRg") && post_image.contains("pXRUJQVlA4"){
                Some(view! {<img src=format!("data:image/webp;base64,{}", post_image) /> <span>{format!("{size_kb} KB WEBP")}</span>})
            } else {
                None
            }
        } else {
            None
        } }

        </div>
        <For
            each={move|| post_text.split("\n").map(String::from).collect::<Vec<String>>()}
            key=|val| val.clone()
            let(body)
        > <p>{body}</p> </For>
        </div>
    }
}
#[component]
pub fn PostReply(
    username: String,
    timestamp: i64,
    post_number: usize,
    post_text: String,
    post_image: String,
) -> impl IntoView {
    view! {
        <div class="post">
            <PostCore username=username timestamp=timestamp post_number=post_number post_text=post_text post_image=post_image/>
        </div>
    }
}

#[component]
pub fn PostToplevel(
    username: String,
    timestamp: i64,
    post_number: usize,
    post_text: String,
    post_image: String,
) -> impl IntoView {
    let (get_posts, set_posts) = signal(Vec::new());
    let set_post_callback: Callback<(Post,)> = Callback::from(move |post: Post| { set_posts.write().insert(0, post); });

    let reply_response: Resource<Option<()>> = Resource::new(
        move || post_number,
        move |post_number| async move {
            let result = api::get_request(format!("http://127.0.0.1:3000/post/reply/{post_number}/{}/{}", 10, 0).as_str()).await
                .map(|GetPostsResponse { posts }| posts);

            if let Some(posts) = result {
                set_posts.set(posts);
            }
            Some(())
        }
    );
    view! {
        <div class="post-and-replies">
        <div class=(["post", "post-reply"], move || true)>
        <PostCore username=username timestamp=timestamp post_number=post_number post_text=post_text post_image=post_image/>
        </div>
        <Suspense fallback = || view! {}>
            {move || Suspend::new(async move { match reply_response.await {
                None => Either::Left(()),
                Some(_) => Either::Right(view! {
                    <PostListReplies get_posts=get_posts/>
                })
            }})}
        </Suspense>
        </div>
    }
}


#[component]
pub fn PostList(
    get_posts: ReadSignal<Vec<Post>>,
) -> impl IntoView {
    view! {
        <For
            each=move|| {
                get_posts.get()
            }
            key=|post| post.post_number
            let(post)
        >
            <PostToplevel
                username=post.username
                timestamp=post.timestamp
                post_number=post.post_number
                post_text=post.text
                post_image=post.image
            />
        </For>
    }
}

#[component]
fn PostListReplies(
    get_posts: ReadSignal<Vec<Post>>,
) -> impl IntoView {
    view! {
        <For
            each=move|| {
                get_posts.get()
            }
            key=|post| post.post_number
            let(post)
        >
            <PostReply
                username=post.username
                timestamp=post.timestamp
                post_number=post.post_number
                post_text=post.text
                post_image=post.image
            />
        </For>
    }
}