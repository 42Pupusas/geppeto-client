use yew::prelude::*;

use crate::components::tools::nostr_msgs::{new_keys, RELAY_URL};
use crate::UserKeypair;
use futures::{SinkExt, StreamExt};
use gloo::console::log;
use gloo_net::websocket::{futures::WebSocket, Message};
use serde_json::json;
use wasm_bindgen_futures::spawn_local;

#[function_component(CreditCounter)]
pub fn credit_counter() -> Html {
    let user_context = use_context::<UserKeypair>().unwrap();
    let user_public_key: String = user_context.public_key.clone().chars().skip(2).collect();
    let credit_state = use_state(|| 0);
    let cloned_credit_state = credit_state.clone();

    let refresh_button = use_state(|| false); 
        let refresh_clone = refresh_button.clone();
    let callback = Callback::from(move |_| {
        let refresh_clone = refresh_button.clone();
        refresh_clone.set(!*refresh_clone); // Reset the credit state
    }); 
    
    let _credit_feed = use_effect_with_deps(
        move |_| {
            let subscription_id: String = hex::encode(&new_keys()[..]);
            let filters = json!({
                "kinds": [9777],
                "#p": [user_public_key],
            });
            log!("filters: {:?}", filters.to_string());
            let token_subscription = json!(["REQ", subscription_id, filters]);
            // Convert the JSON array to a Message format
            let token_subscription_msg = Message::Text(token_subscription.to_string());
            let ws = WebSocket::open(RELAY_URL).unwrap();

            let (mut write, mut read) = ws.split();
            spawn_local(async move {
                write.send(token_subscription_msg).await.unwrap();
            });

            spawn_local(async move {
                let mut counter = 0;

                while let Some(_event) = read.next().await {
                    counter += 1;
                    log!("counter: {:?}", counter);
                    cloned_credit_state.set(counter-1); // Update the credit state
                }
            });

            || {}
        },
        (user_context.clone(), refresh_clone.clone()),
    );
    html! {
        <div>
            <h2>{"CreditCounter"}</h2>
            <p>{"You have "}{credit_state.clone().to_string()}{" credits"}</p>
            <button onclick={callback}>{"Refresh"}</button>
        </div>
    }
}
