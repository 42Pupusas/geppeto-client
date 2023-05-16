use rand::{thread_rng, Rng};
use secp256k1::SecretKey;
use yew::prelude::*;

fn new_keys() -> SecretKey {
    let mut rng = thread_rng();

    // Generate a random 256-bit integer as the private key
    let private_key: [u8; 32] = rng.gen();

    // Convert the private key to a secp256k1 SecretKey object
    let secret_key = SecretKey::from_slice(&private_key).unwrap();

    // Return the private key in hexadecimal format
    secret_key
}

#[function_component(KeyGen)]
pub fn key_gen() -> Html {
    let new_key_state = use_state(|| "".to_owned());
    let cloned_new_key_state = new_key_state.clone();
    let new_key_created = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        cloned_new_key_state.set(hex::encode(new_keys().secret_bytes()));
    });

    html! {
        <form class="keygen" onsubmit={new_key_created}>
            <div>
            <h2>{"KeyGen"}</h2>
            <button>{"Generate"}</button>
            </div>
            <div>
            <h2>{"Private Key: "}</h2>
            <p class="keys">{&*new_key_state}</p>
            </div>
        </form>
    }
}
