use leptos::either::Either;
use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::{api, clear_cookie_data, get_cookie_data, AppState};

#[component]
pub fn TopBar(
    refresh: Callback<()>,
    board_name: ReadSignal<String>,
) -> impl IntoView {
    let app_state: AppState = use_context().unwrap();
    view! {
        <div class="top-bar">
        <a class="top-bar-button" href="/">"Home"</a>

        <h1 class="top-bar-header">{board_name.get_untracked()}</h1>

        {
            match get_cookie_data() {
                Some(cookie_data) => {
                    Either::Left(view! {
                        <button class="top-bar-button" on:click=move |_| {
                            let server_url = app_state.server_url.clone();
                            let cookie_data = cookie_data.clone();
                            spawn_local(async move {
                                _ = api::delete_request::<()>(&format!("{server_url}/auth/{}", cookie_data.get("token").unwrap()), None).await;
                                clear_cookie_data();
                                let window = web_sys::window().unwrap();
                                let location = window.location();
                                _ = location.reload();
                            })
                        }>"Logout"</button>
                    })
                }
                None => {
                    Either::Right(view! {
                        <a class="top-bar-button" href="/login">"Login"</a>
                    })
                }
            }
        }

        </div>
    }
}
