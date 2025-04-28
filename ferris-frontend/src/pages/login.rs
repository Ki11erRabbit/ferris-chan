
use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use ferris_shared::transfer::user::{LoginRequest, LoginResponse};
use crate::{api, AppState};

#[component]
pub fn Login() -> impl IntoView {
    let app_state: AppState = use_context().unwrap();

    let (get_email, set_email) = signal(String::new());
    let (get_password, set_password) = signal(String::new());

    view! {
        <div class="login-page">
        <input type="text" on:input:target=move |ev| set_email.set(ev.target().value()) />
        <input type="password" on:input:target=move |ev| set_password.set(ev.target().value()) />
        <button on:click=move |_| {
            let server_url = app_state.server_url.clone();
            spawn_local(async move {
                let result: Option<LoginResponse> = api::post_request_body(&format!("{server_url}/auth"), LoginRequest::new(get_email.get_untracked(), get_password.get_untracked())).await;

                match result {
                    Some(LoginResponse::Success { token, is_admin }) => {
                        let window = web_sys::window().unwrap();
                        let document = window.document().unwrap();
                        let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
                        html_document.set_cookie(&format!("isAdmin={is_admin}")).unwrap();
                        html_document.set_cookie(&format!("token={token}")).unwrap();
                        let history = window.history().unwrap();
                        history.back().unwrap();
                    }
                    Some(LoginResponse::Error { message }) => {
                        let window = web_sys::window().unwrap();
                        window.alert_with_message(message.as_str()).unwrap();
                    }
                    None => {}
                }
            });
        }>"Login"</button>
        </div>
    }
}