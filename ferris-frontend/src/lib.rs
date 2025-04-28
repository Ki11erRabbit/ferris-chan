use std::collections::HashMap;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use wasm_bindgen::JsCast;

// Modules
mod components;
mod pages;
pub mod api;


// Top-Level pages
use crate::pages::home::Home;

use crate::pages::board::Board;
use crate::pages::login::Login;
use crate::pages::register::Register;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_context(include_str!("server-url.txt").to_string());

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="Ferris-chan" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| view! { NotFound }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/:category/:board") view=Board />
                <Route path=path!("/login") view=Login />
                <Route path=path!("/register") view=Register />
            </Routes>
        </Router>
    }
}

pub fn get_cookie_data() -> Option<HashMap<String, String>> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let cookie_data = html_document.cookie().unwrap();
    let split = cookie_data.split("; ");

    let mut output = HashMap::new();

    for item in split {
        let pair = item.split("=").collect::<Vec<&str>>();
        if pair.len() < 2 {
            continue;
        }
        output.insert(pair[0].to_string(), pair[1].to_string());
    }

    if output.is_empty() {
        None
    } else {
        Some(output)
    }
}

pub fn clear_cookie_data() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let cookie = html_document.cookie().unwrap();
    html_document.set_cookie(&format!("{};expires=Thu, 01 Jan 1970 00:00:00 GMT", cookie)).unwrap();
    let cookie = html_document.cookie().unwrap();
    html_document.set_cookie(&format!("{};expires=Thu, 01 Jan 1970 00:00:00 GMT", cookie)).unwrap();
}
