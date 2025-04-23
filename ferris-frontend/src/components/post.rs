use chrono::{DateTime, Local, Utc};
use leptos::{component, view, IntoView};
use leptos::callback::Callback;
use leptos::context::use_context;
use leptos::control_flow::For;
use leptos::either::{Either, EitherOf3};
use leptos::logging::log;
use leptos::prelude::{signal, ClassAttribute, Get, GetUntracked, GlobalAttributes, GlobalOnAttributes, OnAttribute, Read, ReadSignal, Resource, Set, StorageAccess, Suspend, Suspense, Write, WriteSignal};
use leptos::prelude::ElementChild;
use leptos::task::spawn_local;
use leptos_router::components::A;
use wasm_bindgen::JsValue;
use web_sys::wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::Event;
use web_sys::js_sys::Function;
use ferris_shared::transfer::post::{GetPostReplyResponse, GetPostsResponse, Post};
use crate::api;
use crate::components::base64_img::Base64ImgSize;
use crate::components::send_post::SendPostReply;

#[component]
pub fn PostCore(
    get_board: ReadSignal<String>,
    get_category: ReadSignal<String>,
    username: String,
    timestamp: i64,
    post_number: usize,
    post_text: String,
    post_image: String,
    post_alt_text: String,
    #[prop(into)]
    set_post: Callback<(Post,)>,
    parent: i64,
) -> impl IntoView {

    let (get_reply, set_reply) = signal(false);
    view! {
        <div class="post-header">
            <p>{username}</p><span> {DateTime::<Local>::from(DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap()).format("%x(%a)%H:%M:%S").to_string()}{format!(" No.{}", post_number)}</span>
            <button on:click=move |_| {
                set_reply.set(!get_reply.get());
            }>
            {move ||if get_reply.get() {
                "Close"
            } else {
                "Reply"
            } }
            </button>
        </div>
        <div class="post-content">
        <div class="post-image">
        <Base64ImgSize image=post_image />

        </div>
        <For
            each={move|| post_text.split("\n").map(String::from).collect::<Vec<String>>()}
            key=|val| val.clone()
            let(body)
        > <p>{
            if body.starts_with(">>") {
                let number = body.strip_prefix(">>").unwrap().to_string();
                EitherOf3::A(view! { <div class="post-link"> <A href=format!("#{number}")>{body}</A></div>})
            } else if body.starts_with(">") {
                EitherOf3::B(view! { <div class="post-quote">{body}</div>})
            } else {
                EitherOf3::C(view! { {body} })
            }
        }</p> </For>
        </div>
        { move || {
            let parent = if parent == 0 { post_number as i64 } else { parent };
            if get_reply.get() {
                Some(view! {
                    <SendPostReply parent=parent set_post=set_post get_board=get_board get_category=get_category />
                })
            } else {
                None
            }
        }}
    }
}
#[component]
pub fn PostReply(
    get_board: ReadSignal<String>,
    get_category: ReadSignal<String>,
    username: String,
    timestamp: i64,
    post_number: usize,
    post_text: String,
    post_image: String,
    post_alt_text: String,
    parent: i64,
    #[prop(into)]
    set_post: Callback<(Post,)>,
) -> impl IntoView {
    view! {
        <div class="post" id=format!("{post_number}")>
            <PostCore
                username=username
                timestamp=timestamp
                post_number=post_number
                post_text=post_text
                post_image=post_image
                get_category=get_category
                get_board=get_board
                set_post=set_post
                parent=parent
                post_alt_text=post_alt_text
            />
        </div>
    }
}

#[component]
pub fn PostToplevel(
    get_board: ReadSignal<String>,
    get_category: ReadSignal<String>,
    username: String,
    timestamp: i64,
    post_number: usize,
    post_text: String,
    post_image: String,
    post_alt_text: String,
) -> impl IntoView {
    let (get_posts, set_posts) = signal(Vec::new());
    let set_post_callback: Callback<(Post,)> = Callback::from(move |post: Post| { set_posts.write().insert(0, post); });
    let server_url: String = use_context().unwrap();

    let reply_response: Resource<Option<()>> = Resource::new(
        move || (post_number, server_url.clone()),
        move |(post_number, server_url)| async move {
            let result = api::get_request(format!("{server_url}/post-reply/{post_number}/{}/{}", 10, 0).as_str()).await
                .map(|GetPostReplyResponse { posts }| posts);

            if let Some(posts) = result {
                set_posts.set(posts);
            }
            Some(())
        }
    );
    view! {

        <div class="post-and-replies" id=format!("{post_number}")>
        <div class=(["post", "post-reply"], move || true)>
        <PostCore username=username timestamp=timestamp post_number=post_number post_text=post_text post_image=post_image get_category=get_category get_board=get_board set_post=set_post_callback parent=post_number as i64 post_alt_text=post_alt_text />
        </div>
        <Suspense fallback = || view! {}>
            {move || Suspend::new(async move { match reply_response.await {
                None => Either::Left(()),
                Some(_) => Either::Right(view! {
                    <PostListReplies get_posts=get_posts get_category=get_category get_board=get_board set_post=set_post_callback parent=(post_number as i64)/>
                })
            }})}
        </Suspense>
        </div>
    }
}


#[component]
pub fn PostList(
    get_page: ReadSignal<u64>,
    set_page: WriteSignal<u64>,
    get_board: ReadSignal<String>,
    get_category: ReadSignal<String>,
    get_posts: ReadSignal<Vec<Post>>,
    set_posts: WriteSignal<Vec<Post>>,
) -> impl IntoView {



    let onscroll = move |_: Event| {

        let window = web_sys::window().unwrap();
        let inner_height = window.inner_height().unwrap().as_f64().unwrap();
        let scroll_y = window.scroll_y().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        let offset_height = body.offset_height();
        let server_url: String = use_context().unwrap();

        if (inner_height + scroll_y) >= offset_height as f64 {
            spawn_local(async move {
                let board = get_board.get_untracked();
                let board = urlencoding::encode(board.as_str());


                let result = api::get_request(format!("{server_url}/post/{}/{}/{}/{}", get_category.get_untracked(), board, 10, get_page.get_untracked()).as_str()).await
                    .map(|GetPostsResponse { posts }| posts);


                if let Some(posts) = result {
                    *set_page.write() += posts.len() as u64;
                    set_posts.write().extend(posts);
                }
            });
        }
    };

    let onscroll = Box::new(onscroll) as Box<dyn FnMut(Event)>;

    let closure = Closure::wrap(onscroll);
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.set_onscroll(Some(closure.as_ref().unchecked_ref::<Function>()));
    closure.forget();

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
                get_category=get_category
                get_board=get_board
                post_alt_text=post.alt_text
            />
        </For>
    }
}

#[component]
fn PostListReplies(
    get_board: ReadSignal<String>,
    get_category: ReadSignal<String>,
    get_posts: ReadSignal<Vec<Post>>,
    #[prop(into)]
    set_post: Callback<(Post,)>,
    parent: i64,
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
                get_category=get_category
                get_board=get_board
                set_post=set_post
                parent=parent
                post_alt_text=post.alt_text
            />
        </For>
    }
}