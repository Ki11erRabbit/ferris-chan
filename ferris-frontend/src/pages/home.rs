use leptos::either::Either;
use leptos::prelude::*;
use ferris_shared::transfer::{RootGetResponse, BoardInfo};
use crate::api;
use crate::components::base64_img::Base64Img;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let server_url: String = use_context().unwrap();

    let home_page: Resource<Option<RootGetResponse>> = Resource::new(
        move || server_url.clone(),
        move |server_url| async move {
            api::get_request(&format!("{server_url}/")).await
        }
    );

    //let (data, set_data) = signal(HashMap::new());

    view! {
        <Suspense fallback=|| view! { "Loading..." }>
            {move || Suspend::new(async move { match home_page.await {
                None => Either::Left(view! { <h1>"Source site not found"</h1> }),
                Some(home_page) => Either::Right(view! {


                    <div class="logo">
                    <Base64Img image=home_page.logo />
                    <h1>{home_page.title.clone()}</h1>
                    </div>
                    <div class="categories">
                    <div class="categories-header"><h2>{"Boards"}</h2></div>
                    <div class="categories-list">
                    <For
                        each=move|| {
                            let categories =home_page.categories.clone();
                            categories.into_iter()
                        }
                        key=|x| x.clone()
                        let(category)
                    ><div class="board-category"> <h3> {category.clone()}</h3> <ul>{
                        home_page.boards.iter()
                            .filter(|BoardInfo { category: cat, ..}| *cat == category)
                            .map(|BoardInfo { name, ..}| {
                                let category_url = urlencoding::encode(&category);
                                let board_url = urlencoding::encode(&name);

                                view! { <li><a href=format!("/{}/{}",category_url, board_url)>{name.clone()}</a></li>}
                        })
                            .collect::<Vec<_>>()
                    }</ul></div></For>
                    </div>
                    </div>

                })
            }})}
        </Suspense>
    }
}
