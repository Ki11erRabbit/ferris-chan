use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use leptos::prelude::*;


#[component]
pub fn Base64Img(image: String) -> impl IntoView {
    view! {
        {if image.len() > 0 {
            let bytes = BASE64_STANDARD.decode(image.as_bytes()).unwrap();
            let size_kb = bytes.len() / 1024;

            if image.starts_with("iVBORw0KGgo") {
                Some(view! {<img src=format!("data:image/png;base64,{}", image) />})
            } else if image.starts_with("/9") {
                Some(view! {<img src=format!("data:image/jpg;base64,{}", image) />})
            } else if image.starts_with("UklGRg") && image.contains("pXRUJQVlA4"){
                Some(view! {<img src=format!("data:image/webp;base64,{}", image) />})
            } else {
                None
            }
        } else {
            None
        } }
    }
}

#[component]
pub fn Base64ImgSize(image: String) -> impl IntoView {
    view! {
        {if image.len() > 0 {
            let Ok(bytes) = BASE64_STANDARD.decode(image.as_bytes()) else {
                return None
            };
            let size_kb = bytes.len() / 1024;

            if image.starts_with("iVBORw0KGgo") {
                Some(view! {<img src=format!("data:image/png;base64,{}", image) /> <span>{format!("{size_kb} KB PNG")}</span>})
            } else if image.starts_with("/9") {
                Some(view! {<img src=format!("data:image/jpg;base64,{}", image) /> <span>{format!("{size_kb} KB JPG")}</span>})
            } else if image.starts_with("UklGRg") && image.contains("pXRUJQVlA4"){
                Some(view! {<img src=format!("data:image/webp;base64,{}", image) /> <span>{format!("{size_kb} KB WEBP")}</span>})
            } else {
                None
            }
        } else {
            None
        } }
    }
}
