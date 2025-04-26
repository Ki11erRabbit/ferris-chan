
use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use ferris_shared::transfer::user::{LoginRequest, LoginResponse};
use crate::api;

#[component]
pub fn Login() -> impl IntoView {

    let (get_email, set_email) = signal(String::new());
    let (get_password, set_password) = signal(String::new());

    view! {
        <div class="login-page">
        <input type="text" on:input:target=move |ev| set_email.set(ev.target().value()) />
        <input type="password" on:input:target=move |ev| set_password.set(ev.target().value()) />
        <button on:click=move |_| {
            spawn_local(async move {
                let server_url: String = use_context().unwrap();
                let result: Option<LoginResponse> = api::put_request_body(&format!("{server_url}/auth"), LoginRequest::new(get_email.get_untracked(), get_password.get_untracked())).await;

                if let Some(LoginResponse { token, is_admin }) = result {
                    let window = web_sys::window().unwrap();
                    let document = window.document().unwrap();
                    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
                    html_document.set_cookie(&format!("token={token}; is_admin={is_admin}")).unwrap();
                }
            });
        }>"Login"</button>
        </div>
    }
}