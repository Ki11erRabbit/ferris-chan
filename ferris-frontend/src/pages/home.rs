use leptos::either::Either;
use crate::components::counter_btn::Button;
use leptos::prelude::*;
use ferris_shared::transfer::RootGetResponse;
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

    view! {
        <Suspense fallback=|| view! { "Loading..." }>
            {move || Suspend::new(async move { match home_page.await {
                None => Either::Left(view! { <h1>"Source site not found"</h1> }),
                Some(home_page) => Either::Right(view! {
                    <div>
                        <h1>{home_page.title}</h1>
                        <ul>
                            {
                                home_page.boards.iter().map(|board| view! { <li>{board.name.clone()}</li>})
                                .collect_view()
                            }
                        </ul>
                    </div>
                })
            }})}
        </Suspense>
    }
}
