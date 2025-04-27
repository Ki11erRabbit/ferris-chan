use leptos::attr::name;
use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use ferris_shared::transfer::post::{GetPostsResponse, Post};
use crate::api;
use crate::components::post::PostList;
use crate::components::send_post::SendPost;
use crate::components::topbar::TopBar;

#[component]
pub fn Board() -> impl IntoView {

    let (get_posts, set_posts) = signal(Vec::new());
    let (get_page, set_page) = signal(10);
    let set_post_callback: Callback<(Post,)> = Callback::from(move |post: Post| { set_posts.write().insert(0, post); });

    let params = use_params_map();
    let server_url: String = use_context().unwrap();
    let server_url2 = server_url.clone();

    let board_response: Resource<Option<()>> = Resource::new(
        move || (params.read().get("category").unwrap().clone(), params.read().get("board").unwrap().clone(), server_url2.clone()),
        move |(category, board, server_url)| async move {
            let category = urlencoding::encode(category.as_str());
            let board = urlencoding::encode(board.as_str());

            let result = api::get_request(format!("{}/post/{category}/{board}/{}/{}", server_url.clone(), 10, 0).as_str()).await
                .map(|GetPostsResponse { posts }| posts);

            if let Some(posts) = result {
                *set_page.write() += posts.len() as u64;
                set_posts.set(posts);
            }
            Some(())
        }
    );

    let (get_board, set_board) = signal(String::new());
    set_board.set(params.read_untracked().get("board").unwrap().clone());
    let (get_category, set_category) = signal(String::new());
    set_category.set(params.read_untracked().get("category").unwrap().clone());


    let (get_file, set_file) = signal(String::new());
    let set_file_callback: Callback<(String,)> = Callback::from(move |file| {set_file.set(file)});

    let refresh_callback = Callback::from(move || {
        let navigate = leptos_router::hooks::use_navigate();
        navigate(&format!("{}/{}/{}", server_url.clone(), get_category.get_untracked(), get_board.get_untracked()), Default::default());
    });

    view! {
        <Suspense fallback=|| view! { "Loading..." }>
            {move || Suspend::new(async move { match board_response.await {
                None => Either::Left(view! { <h1>"Source site not found"</h1> }),
                Some(_) => Either::Right(view! {
                    <TopBar refresh=refresh_callback board_name=get_board />
                    <SendPost get_board=get_board get_category=get_category set_post=set_post_callback />
                    <PostList
                        get_posts=get_posts
                        get_board=get_board
                        get_category=get_category
                        get_page=get_page
                        set_page=set_page
                        set_posts=set_posts
                    />
                })
            }})}
        </Suspense>
    }
}
