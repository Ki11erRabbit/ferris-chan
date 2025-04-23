use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use leptos::{component, view, IntoView};
use leptos::logging::log;
use leptos::prelude::{signal, Callable, Callback, Get, GetUntracked, OnAttribute, PropAttribute, ReadSignal, Set, ElementChild, OnTargetAttribute, use_context};
use leptos::task::spawn_local;
use leptos::web_sys::{HtmlInputElement};
use leptos::web_sys::Blob;
use leptos::wasm_bindgen::JsCast;
use web_sys::js_sys::{ArrayBuffer, Uint8Array};
use ferris_shared::transfer::post::{CreatePostReplyRequest, CreatePostReplyResponse, CreatePostRequest, CreatePostResponse, Post};
use crate::api;

async fn to_base64(data: Blob) -> String {
    let file_raw_data = wasm_bindgen_futures::JsFuture::from(data.array_buffer())
        .await
        .expect("File reading should not fail");

    let file_raw_data = file_raw_data
        .dyn_into::<ArrayBuffer>()
        .expect("Expected an ArrayBuffer");

    let file_raw_data = Uint8Array::new(&file_raw_data);
    let len = file_raw_data.length() as usize;

    let mut file_bytes = vec![0; len];
    file_raw_data.copy_to(&mut file_bytes.as_mut_slice());

    let output = BASE64_STANDARD.encode(file_bytes.as_slice());
    log!("{}", output);
    output
}


#[component]
pub fn UploadFile(
    #[prop(into)]
    set_file: Callback<(String,)>
) -> impl IntoView {


    let handle_file_conversion = move |event: leptos::ev::Event| {
        let input: HtmlInputElement = event.target().unwrap().unchecked_into();

        if let Some(files) = input.files() {
            spawn_local(async move {
                let file = files.get(0).unwrap();
                let blob = file.slice().expect("File reading should not fail");

                let file = to_base64(blob).await;
                set_file.run((file,))
            });
        }
    };

    view! {
        <input
            type="file"
            on:input=handle_file_conversion

        />
    }
}

#[component]
pub fn SendPost(
    get_board: ReadSignal<String>,
    get_category: ReadSignal<String>,
    #[prop(into)]
    set_post: Callback<(Post,)>,
) -> impl IntoView {

    let (get_file, set_file) = signal(String::new());
    let set_file_callback: Callback<(String,)> = Callback::from(move |file| {set_file.set(file)});

    let (get_text, set_text) = signal(String::new());
    let (get_alt_text, set_alt_text) = signal(String::new());

    let server_url: String = use_context().unwrap();

    view! {
        <UploadFile set_file=set_file_callback />
        <input type="text" on:input:target=move |ev| set_alt_text.set(ev.target().value()) />
        <textarea
            prop:value=move || get_text.get()
            on:input:target=move |ev| set_text.set(ev.target().value())
        >{move || get_text.get_untracked()}</textarea>
        <button on:click=move |_| {
            let server_url = server_url.clone();
            spawn_local(async move {
                let result: Option<CreatePostResponse> = api::post_request_body(&format!("{server_url}/post"), CreatePostRequest::new(
                    get_board.get_untracked(),
                    get_category.get_untracked(),
                    get_file.get_untracked(),
                    get_alt_text.get_untracked(),
                    get_text.get_untracked(),
                    None
                ))
                .await;

                if let Some(result) = result {
                    set_post.run((result.post,));
                }
            })
        }>
            "Post"
        </button>
    }
}

#[component]
pub fn SendPostReply(
    get_board: ReadSignal<String>,
    get_category: ReadSignal<String>,
    #[prop(into)]
    set_post: Callback<(Post,)>,
    parent: i64,
) -> impl IntoView {

    let (get_file, set_file) = signal(String::new());
    let set_file_callback: Callback<(String,)> = Callback::from(move |file| {set_file.set(file)});

    let (get_text, set_text) = signal(String::new());
    let (get_alt_text, set_alt_text) = signal(String::new());
    let server_url: String = use_context().unwrap();

    view! {
        <UploadFile set_file=set_file_callback />
        <input type="text" on:input:target=move |ev| set_alt_text.set(ev.target().value()) />
        <textarea
            prop:value=move || get_text.get()
            on:input:target=move |ev| set_text.set(ev.target().value())
        >{move || get_text.get_untracked()}</textarea>
        <button on:click=move |_| {
            let server_url = server_url.clone();
            spawn_local(async move {
                let category = get_category.get_untracked();
                let category = urlencoding::decode(category.as_str()).unwrap();
                let board = get_board.get_untracked();
                let board = urlencoding::decode(board.as_str()).unwrap();
                let result: Option<CreatePostReplyResponse> = api::post_request_body(&format!("{server_url}/post/reply"), CreatePostReplyRequest::new(
                    board.to_string(),
                    category.to_string(),
                    get_file.get_untracked(),
                    get_alt_text.get_untracked(),
                    get_text.get_untracked(),
                    parent,
                    None
                ))
                .await;

                if let Some(result) = result {
                    set_post.run((result.post,));
                }
            })
        }>
            "Post"
        </button>
    }
}