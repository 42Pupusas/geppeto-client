use crate::components::tools::nostr_msgs::{get_unix_timestamp, new_keys};
use futures::{SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use serde_json::{json, Value};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(EventBox)]
pub fn event_box() -> Html {
    let prompt_state = use_state(|| "".to_owned());
    let cloned_prompt_state = prompt_state.clone();

    let _prompt_feed = use_effect_with_deps(
        move |_| {
            let subscription_id: String = hex::encode(&new_keys()[..]);
            let filters = json!({
                "kinds": [29001],
                "since": get_unix_timestamp()
            });

            let token_subscription = json!(["REQ", subscription_id, filters]);
            // Convert the JSON array to a Message format
            let token_subscription_msg = Message::Text(token_subscription.to_string());
            let ws = WebSocket::open("wss://relay.roadrunner.lat").unwrap();
            let (mut write, mut read) = ws.split();

            spawn_local(async move {
                write.send(token_subscription_msg).await.unwrap();
            });
            spawn_local(async move {
                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(Message::Text(msg)) => {
                            if let Ok((_event_type, _event_id, event)) =
                                serde_json::from_str::<(String, String, Value)>(&msg)
                            {
                                match event["kind"].as_u64().unwrap() {
                                    29001 => {
                                        // log!("event: {:?}", event["content"].as_str().unwrap());
                                        cloned_prompt_state
                                            .set(event["content"].as_str().unwrap().to_owned());
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            });

            || {}
        },
        prompt_state.clone(),
    );

    html! {
            <div class="response">
            <h1>{"GePpeTto Says:"}</h1>
            {if *prompt_state != "".to_owned() {
               html! {  <div class="content">
                <p class="response">{&*prompt_state}</p>
                </div>}
            }    else {
                html! {}
            }}

            </div>
        }
}
