use crate::components::tools::nostr_msgs::{new_keys};
use crate::UserKeypair;
use futures::{SinkExt, StreamExt};
use gloo::console::log;
use gloo_net::websocket::{futures::WebSocket, Message};
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode, Version};
use serde_json::{json, Value};
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, Element};
use yew::prelude::*;

pub fn create_qr_code(data: &str) -> String {
    if data.is_empty() {
        return "".to_owned();
    } else {
    let code = QrCode::with_version(data.as_bytes(), Version::Normal(20), EcLevel::H).unwrap();
    let image = code.render::<svg::Color>().max_dimensions(280, 280).build();
    println!("{}", image);
    image
    }
}


#[function_component(InvoiceDisplay)]
pub fn invoice_display() -> Html {
    let user_context = use_context::<UserKeypair>().unwrap();
    let user_public_key: String = user_context.public_key.clone().chars().skip(2).collect();
    let invoice_state = use_state(|| "".to_owned());
    let invoice_state_clone = invoice_state.clone();
    let invoice_state_qr = invoice_state.clone();
    let node_ref = NodeRef::default();
    let node_ref_clone = node_ref.clone();

    let _invoice_feed = use_effect_with_deps(
        move |_| {
            let cloned_invoice_state = invoice_state_clone.clone();

            let subscription_id: String = hex::encode(&new_keys()[..]);
            let filters = json!({
                "kinds": [29778],
                "#p": [user_public_key],
            });
            log!("filters: {:?}", filters.to_string());
            let token_subscription = json!(["REQ", subscription_id, filters]);
            // Convert the JSON array to a Message format
            let token_subscription_msg = Message::Text(token_subscription.to_string());
            let ws = WebSocket::open("ws://192.168.1.5:6969").unwrap();

            let (mut write, mut read) = ws.split();
            spawn_local(async move {
                write.send(token_subscription_msg).await.unwrap();
            });

            spawn_local(async move {
                while let Some(event) = read.next().await {
                    match event {
                        Ok(Message::Text(text)) => {
                            if let Ok((_event_type, _event_id, event)) =
                                serde_json::from_str::<(String, String, Value)>(&text)
                            {
                                match event["kind"].as_u64().unwrap() {
                                    29778 => {
                                        log!("event: {:?}", event["content"].as_str().unwrap());
                                        cloned_invoice_state
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
        user_context.clone(),
    );

    use_effect(move || {
        let svg_string = create_qr_code(&invoice_state_qr.clone());
        log!("svg_string: {:?}", svg_string.clone());
        if !svg_string.is_empty() {
            let node_ref = node_ref.clone();
            if let Some(div) = node_ref.cast::<Element>() {
                let document = window().unwrap().document().unwrap();
                let svg_element: Element = document
                    .create_element("div")
                    .unwrap();
                svg_element.set_attribute("class", "svg-container").unwrap();
                svg_element.set_inner_html(&svg_string);
                // Clear the div's contents before appending new QR code
                div.set_inner_html("");
                div.append_child(&svg_element).unwrap();
                

            }
        }
        || {}
    });
    {if !invoice_state.is_empty() {
    html! {
        <div class="content">
        <h2>{"Invoice Display"}</h2>
        <p class="keys">{&*invoice_state}</p>
        <div class="svg-container" ref={node_ref_clone}> </div>
        </div>
        }} else {
            html! {}
        }
    }
}
