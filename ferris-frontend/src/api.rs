use leptos::logging;
use leptos::prelude::on_cleanup;
use leptos::server_fn::serde::de::DeserializeOwned;
use leptos::server_fn::serde::Serialize;
use send_wrapper::SendWrapper;
use leptos::wasm_bindgen::JsValue;
use leptos::web_sys::AbortController;

pub fn get_request<T>(path: &str) -> impl std::future::Future<Output = Option<T>>
where
    T: Serialize + DeserializeOwned {
    SendWrapper::new(async move {
        let abort_controller = SendWrapper::new(AbortController::new().ok());
        let abort_signal = abort_controller.as_ref().map(|a| a.signal());

        on_cleanup(move || {
            if let Some(abort_controller) = abort_controller.take() {
                abort_controller.abort();
            }
        });

        gloo_net::http::Request::get(path)
            .abort_signal(abort_signal.as_ref())
            .send()
            .await
            .map_err(|e| logging::error!("{e}"))
            .ok()?
            .json()
            .await
            .ok()
    })
}

pub fn post_request<T>(path: &str) -> impl std::future::Future<Output = Option<T>>
where
    T: Serialize + DeserializeOwned {
    SendWrapper::new(async move {
        let abort_controller = SendWrapper::new(AbortController::new().ok());
        let abort_signal = abort_controller.as_ref().map(|a| a.signal());

        on_cleanup(move || {
            if let Some(abort_controller) = abort_controller.take() {
                abort_controller.abort();
            }
        });

        gloo_net::http::Request::post(path)
            .abort_signal(abort_signal.as_ref())
            .send()
            .await
            .map_err(|e| logging::error!("{e}"))
            .ok()?
            .json()
            .await
            .ok()
    })
}
pub fn post_request_body<R, T>(path: &str, request: R) -> impl std::future::Future<Output = Option<T>>
where
    R: Serialize,
    T: Serialize + DeserializeOwned {
    SendWrapper::new(async move {
        let abort_controller = SendWrapper::new(AbortController::new().ok());
        let abort_signal = abort_controller.as_ref().map(|a| a.signal());

        on_cleanup(move || {
            if let Some(abort_controller) = abort_controller.take() {
                abort_controller.abort();
            }
        });

        gloo_net::http::Request::post(path)
            .abort_signal(abort_signal.as_ref())
            .json(&request)
            .map_err(|e| logging::error!("{e}"))
            .ok()?
            .send()
            .await
            .map_err(|e| logging::error!("{e}"))
            .ok()?
            .json()
            .await
            .ok()
    })
}

pub fn put_request<T>(path: &str) -> impl std::future::Future<Output = Option<T>>
where
    T: Serialize + DeserializeOwned {
    SendWrapper::new(async move {
        let abort_controller = SendWrapper::new(AbortController::new().ok());
        let abort_signal = abort_controller.as_ref().map(|a| a.signal());

        on_cleanup(move || {
            if let Some(abort_controller) = abort_controller.take() {
                abort_controller.abort();
            }
        });

        gloo_net::http::Request::put(path)
            .abort_signal(abort_signal.as_ref())
            .send()
            .await
            .map_err(|e| logging::error!("{e}"))
            .ok()?
            .json()
            .await
            .ok()
    })
}
pub fn put_request_body<R, T>(path: &str, request: R) -> impl std::future::Future<Output = Option<T>>
where
    R: Serialize,
    T: Serialize + DeserializeOwned {
    SendWrapper::new(async move {
        let abort_controller = SendWrapper::new(AbortController::new().ok());
        let abort_signal = abort_controller.as_ref().map(|a| a.signal());

        on_cleanup(move || {
            if let Some(abort_controller) = abort_controller.take() {
                abort_controller.abort();
            }
        });

        gloo_net::http::Request::put(path)
            .abort_signal(abort_signal.as_ref())
            .json(&request)
            .map_err(|e| logging::error!("{e}"))
            .ok()?
            .send()
            .await
            .map_err(|e| logging::error!("{e}"))
            .ok()?
            .json()
            .await
            .ok()
    })
}

pub fn delete_request<R, T>(path: &str, request: Option<R>) -> impl std::future::Future<Output = Option<T>>
where
    R: DeserializeOwned + Into<JsValue> + Send,
    T: Serialize + DeserializeOwned {
    SendWrapper::new(async move {
        let abort_controller = SendWrapper::new(AbortController::new().ok());
        let abort_signal = abort_controller.as_ref().map(|a| a.signal());

        on_cleanup(move || {
            if let Some(abort_controller) = abort_controller.take() {
                abort_controller.abort();
            }
        });

        let request_builder = gloo_net::http::Request::delete(path)
            .abort_signal(abort_signal.as_ref());

        let request = if let Some(request) = request {
            request_builder.body(request)
                .map_err(|e| logging::error!("{e}"))
                .ok()?
                .send()
                .await
        } else {
            request_builder
                .send()
                .await
        };
        request
            .map_err(|e| logging::error!("{e}"))
            .ok()?
            .json()
            .await
            .ok()
    })
}