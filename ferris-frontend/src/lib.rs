use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

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
