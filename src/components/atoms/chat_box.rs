use crate::{components::tools::nostr_msgs::{self, RELAY_URL}, UserKeypair};
use futures::{SinkExt, StreamExt};
use gloo::console::log;
use gloo_net::websocket::{futures::WebSocket, Message};
use serde_json::json;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(ChatBox)]
pub fn chat_input() -> Html {
    // Load up user context with keypair
    let user_context = use_context::<UserKeypair>().unwrap();

    // Create state for chat input
    let chat_state = use_state(|| "".to_owned());
    let cloned_chat_state = chat_state.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        cloned_chat_state.set(value);
    });

    // Handle the submit event
    let submit_chat = Callback::from(move |event: FocusEvent| {
        // Stop reload
        event.prevent_default();

        // Create Nostr message and sign it with keypair
        let new_chat_prompt =
            nostr_msgs::create_chat_event(chat_state.clone().to_string(), user_context.clone());
        let prompt_request = json!(["EVENT", new_chat_prompt]).to_string();
        let ws = WebSocket::open(RELAY_URL).unwrap();
        let (mut write, _read) = ws.split();

        spawn_local(async move {
            write
                .send(Message::Text(String::from(prompt_request.clone())))
                .await
                .unwrap();
            log!("sent message: {:?}", prompt_request);
        });
    });

    html! {
            <div>
            <form onsubmit={submit_chat}>
            <h2>{"Ask GePpeTto"}</h2>
            <input type="text" onchange={onchange}/>
            <button>{"Ask"}</button>
            </form>
            </div>
    }
}
