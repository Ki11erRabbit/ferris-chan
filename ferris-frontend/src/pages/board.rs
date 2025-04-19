use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use ferris_shared::transfer::post::{GetPostsRequest, GetPostsResponse};
use ferris_shared::transfer::RootGetResponse;
use crate::api;

#[component]
pub fn Board() -> impl IntoView {

    let params = use_params_map();

    let board_response: Resource<Option<GetPostsResponse>> = Resource::new(
        move || (params.read().get("category").unwrap().clone(), params.read().get("board").unwrap().clone()),
        move |(category, board)| async move {
            api::get_request_body("http://localhost:3000/post", GetPostsRequest::new(board, category, 10, 0)).await
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
                            <h4>{post.username.clone()}{post.post_number.to_string()}</h4>
                            <p>{post.text.clone()}</p>
                        }
                    }</For>

                })
            }})}
        </Suspense>
    }
}
