use leptos::either::Either;
use leptos::prelude::*;
use ferris_shared::transfer::{RootGetResponse, BoardInfo};
use crate::api;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {

    let home_page: Resource<Option<RootGetResponse>> = Resource::new(
        move || None,
        move |_: Option<()>| async move {
            api::get_request("http://localhost:3000/").await
        }
    );

    //let (data, set_data) = signal(HashMap::new());

    view! {
        <Suspense fallback=|| view! { "Loading..." }>
            {move || Suspend::new(async move { match home_page.await {
                None => Either::Left(view! { <h1>"Source site not found"</h1> }),
                Some(home_page) => Either::Right(view! {
                    <h1>{home_page.title.clone()}</h1>
                    <div class="categories"><For
                        each=move|| {
                            let categories =home_page.categories.clone();
                            categories.into_iter()
                        }
                        key=|x| x.clone()
                        let(category)
                    ><div class="board-category"> <h3> {category.clone()}</h3> <ul>{
                        home_page.boards.iter()
                            .filter(|BoardInfo { category: cat, ..}| *cat == category)
                            .map(|BoardInfo { name, ..}| view! { <li><a href=format!("http://localhost:3001/{}/{}",category, name)>{name.clone()}</a></li>})
                            .collect::<Vec<_>>()
                    }</ul></div></For>
                    </div>

                })
            }})}
        </Suspense>
    }
}
