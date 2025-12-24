use dioxus::prelude::*;
use web_sys::console;

use crate::{AuthState, Route};

#[component]
pub fn Callback(code: String, state: String) -> Element {
    let mut auth_signal = use_context::<Signal<AuthState>>();

    use_effect(move || {
        console::debug_1(&format!("URL Params - code: {:?}, state: {:?}", code, state).into());
        let code = code.clone();
        let state = state.clone();
        spawn(async move {
            let auth = AuthState::new().callback(&code, &state).await;
            auth_signal.set(auth);
            let route = Route::from_string(&state);
            navigator().push(route);
        });
    });
    rsx!(
        div {
            h1 { "Processing authentication callback..." }
        }
    )
}
