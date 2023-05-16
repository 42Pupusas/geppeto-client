use std::ops::Deref;

use secp256k1::{KeyPair, SecretKey};
use yew::prelude::*;

mod components;

use crate::components::atoms::chat_box::ChatBox;
use crate::components::atoms::credit_box::CreditBox;
use crate::components::atoms::event_box::EventBox;
use crate::components::atoms::key_gen::KeyGen;
use crate::components::atoms::login::get_public_key;
use crate::components::atoms::login::LoginForm;
use crate::components::atoms::credit_counter::CreditCounter;
use crate::components::atoms::invoice_display::InvoiceDisplay;

#[derive(Clone, PartialEq, Default)]
pub struct UserKeypair {
    private_key: String,
    pub public_key: String,
}

impl UserKeypair {
    fn new(private_key: String) -> Self {
        let secret_key = SecretKey::from_slice(&hex::decode(private_key.clone()).unwrap()).unwrap();
        let public_key = get_public_key(&secret_key);
        let public_key = hex::encode(public_key.serialize());
        Self {
            private_key,
            public_key,
        }
    }

    fn get_keypair(&self) -> KeyPair {
        let secret_key =
            SecretKey::from_slice(&hex::decode(self.private_key.clone()).unwrap()).unwrap();
        let keypair = KeyPair::from_secret_key(&secp256k1::Secp256k1::new(), &secret_key);
        keypair
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let userkeys_state = use_state(|| UserKeypair::default());

    let handle_login = {
        let userkeys_state = userkeys_state.clone();
        Callback::from(move |keypair: UserKeypair| {
            userkeys_state.set(keypair);
        })
    };
    html! {
        <>
        <div class="background">
        </div>
        <ContextProvider<UserKeypair> context={userkeys_state.deref().clone()}>
            <h1>{ "GePpeTto" }</h1>
            <br />
            <span>{ "A GPT-3 access point through Nostr, powered by lightning" }</span>
            
            <div class="container">
                <div>

                    <div class="container">
                        <div class="login content">
                            <KeyGen />
                            <LoginForm handle_login={handle_login}/>
                        </div>
                        
                        <div class="chat content">
                            <ChatBox />
                        </div>
                    </div>

                    <div class="container">
                        <div class="credits content">
                            <CreditBox />
                            <CreditCounter />
                        </div>
                        <InvoiceDisplay />
                    </div>
                </div>

                <EventBox />
            </div>
            
        </ContextProvider<UserKeypair>>
        </>
    }
}
