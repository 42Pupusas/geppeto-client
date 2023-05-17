use futures::{SinkExt, StreamExt};
use gloo::console::log;
use gloo_net::websocket::{futures::WebSocket, Message};
use serde_json::json;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::tools::nostr_msgs::{self, RELAY_URL};
use crate::UserKeypair;

#[function_component(CreditBox)]
pub fn credit_box() -> Html {
    let user_context = use_context::<UserKeypair>().unwrap();
    let credit_state = use_state(|| 0);
    let credit_state_onchange = credit_state.clone();
    let cloned_credit_state = credit_state.clone();

    let onchange = Callback::from(move |event: Event| {
        let cloned_credit_state = credit_state_onchange.clone();
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        cloned_credit_state.set(value.parse::<i32>().unwrap());
    });

    let credit_request = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        // Create Nostr message and sign it with keypair
        let new_credit_prompt = nostr_msgs::create_token_event(
            cloned_credit_state.clone().to_string(),
            user_context.clone(),
        );
        let prompt_request = json!(["EVENT", new_credit_prompt]).to_string();
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
        <form onsubmit={credit_request}>
        <h2>{"CreditBox"}</h2>
        <select onchange={onchange}>
            <option value="10">{"10"}</option>
            <option value="20">{"20"}</option>
            <option value="30">{"30"}</option>
            <option value="40">{"40"}</option>
            <option value="50">{"50"}</option>
        </select>
        <p>{"Requesting "}{credit_state.clone().to_string()}{" credits"}</p>
        <button>{"Request"}</button>
        </form>
    }
}
