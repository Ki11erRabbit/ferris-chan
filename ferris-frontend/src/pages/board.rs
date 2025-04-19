use chrono::{DateTime, Local, Utc};
use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use ferris_shared::transfer::post::{GetPostsRequest, GetPostsResponse};
use crate::api;

#[component]
pub fn Board() -> impl IntoView {

    let params = use_params_map();

    let board_response: Resource<Option<GetPostsResponse>> = Resource::new(
        move || (params.read().get("category").unwrap().clone(), params.read().get("board").unwrap().clone()),
        move |(category, board)| async move {
            api::get_request(format!("http://127.0.0.1:3000/post/{category}/{board}/{}/{}", 10, 0).as_str()).await
        }
    );

    view! {
        <Suspense fallback=|| view! { "Loading..." }>
            {move || Suspend::new(async move { match board_response.await {
                None => Either::Left(view! { <h1>"Source site not found"</h1> }),
                Some(GetPostsResponse { posts }) => Either::Right(view! {
                    <h1>{params.read().get("board").unwrap().clone()}</h1>
                    <For
                        each=move|| {
                            let posts = posts.clone();
                            posts.into_iter()
                        }
                        key=|x| x.clone()
                        let(post)
                    >{
                        view! {
                            <div class="post">
                            <div class="post-header"><p>{post.username.clone()}</p><span> {DateTime::<Local>::from(DateTime::<Utc>::from_timestamp(post.timestamp, 0).unwrap()).format("%x(%a)%H:%M:%S").to_string()}{format!(" No.{}", post.post_number)}</span></div>
                            <p>{post.text.clone()}</p>
                            </div>
                        }
                    }</For>

                })
            }})}
        </Suspense>
    }
}
