use yew::prelude::*;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

use crate::UserKeypair;

pub fn get_public_key(private_key: &SecretKey) -> PublicKey {
    // Create a secp256k1 context
    let secp = Secp256k1::new();

    // Generate the public key from the private key
    let public_key = PublicKey::from_secret_key(&secp, private_key);

    public_key
}

#[derive(PartialEq, Properties)]
pub struct LoginProps {
    pub handle_login: Callback<UserKeypair>,
}
#[function_component(LoginForm)]
pub fn login_form(props: &LoginProps) -> Html {
    let input = use_state(|| "".to_owned());
    let cloned_input = input.clone();
    let handle_onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        cloned_input.set(value);
    });
    
    let handle_login = props.handle_login.clone();
    let userkeys_context = use_context::<UserKeypair>().unwrap();
    let handle_login = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        let new_keypair = UserKeypair::new(input.clone().to_string());
        handle_login.emit(new_keypair);
    });
    
    html! {
        <form onsubmit={handle_login}>
            <h2>{"Login"}</h2>
            <input name="private_key" onchange={handle_onchange} placeholder="32-bit Hex Private Key Here"/>
            <button>{"Login"}</button>
            <p>{"Logged in as: "}</p>
            <p class="keys">{userkeys_context.public_key}</p>
        </form>
    }
}