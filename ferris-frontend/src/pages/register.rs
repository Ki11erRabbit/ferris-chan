
use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use ferris_shared::transfer::user::{LoginRequest, LoginResponse, RegisterRequest};
use crate::api;

#[component]
pub fn Register() -> impl IntoView {
    let server_url: String = use_context().unwrap();

    let (get_username, set_username) = signal(String::new());
    let (get_email, set_email) = signal(String::new());
    let (get_password, set_password) = signal(String::new());

    view! {
        <div class="login-page">
        <input type="text" on:input:target=move |ev| set_username.set(ev.target().value()) />
        <input type="text" on:input:target=move |ev| set_email.set(ev.target().value()) />
        <input type="password" on:input:target=move |ev| set_password.set(ev.target().value()) />
        <button on:click=move |_| {
            let server_url = server_url.clone();
            spawn_local(async move {

                let result: Option<LoginResponse> = api::post_request_body(&format!("{server_url}/auth/register"), RegisterRequest::new(get_username.get_untracked(), get_email.get_untracked(), get_password.get_untracked())).await;

                match result {
                    Some(LoginResponse::Success { token, is_admin }) => {
                        let window = web_sys::window().unwrap();
                        let document = window.document().unwrap();
                        let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
                        html_document.set_cookie(&format!("token={token}; is_admin={is_admin}")).unwrap();
                    }
                    Some(LoginResponse::Error { message }) => {
                        let window = web_sys::window().unwrap();
                        window.alert_with_message(message.as_str()).unwrap();
                    }
                    None => {}
                }
            });
        }>"Register"</button>
        </div>
    }
}