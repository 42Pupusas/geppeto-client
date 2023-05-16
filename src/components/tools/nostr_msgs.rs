use rand::{Rng, thread_rng};
use secp256k1::{Secp256k1, SecretKey, Message};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use serde_json::{json, to_value};
use instant::now;
use crate::UserKeypair;

#[derive(Serialize)]
pub struct NostrEvent {
    pubkey: String,
    created_at: u64,
    kind: u32,
    tags: Vec<Vec<String>>,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedEvent {
    pub id: String,
    pubkey: String,
    created_at: u64,
    kind: u32,
    tags: Vec<Vec<String>>,
    content: String,
    sig: String,
}

pub fn new_keys() -> SecretKey{
    let mut rng = thread_rng();

    // Generate a random 256-bit integer as the private key
    let private_key: [u8; 32] = rng.gen();

    // Convert the private key to a secp256k1 SecretKey object
    let secret_key = SecretKey::from_slice(&private_key).unwrap();

    // Return the private key in hexadecimal format
     secret_key
}

pub fn get_unix_timestamp() -> u64 {
    // Get the current time as a SystemTime object
    let current_time = now();
    current_time as u64 / 1000
}

pub fn create_chat_event(content: String, user: UserKeypair) -> SignedEvent {

    // Example values for the event fields
    let created_at = get_unix_timestamp();
    let kind = 29000;
    let tags: Vec<Vec<String>> = vec![];

    let my_event = NostrEvent {
        pubkey: user.public_key.to_string()[2..].to_string(),
        created_at,
        kind,
        tags,
        content
    };
    let value = to_value(&my_event).unwrap();

    let json_str = json!([
        0,
        value["pubkey"],
        value["created_at"],
        value["kind"],
        value["tags"],
        value["content"]
    ]);
    // Serialize the event as JSON

    // Compute the SHA256 hash of the serialized JSON string
    let mut hasher = Sha256::new();
    hasher.update(serde_json::to_string(&json_str).unwrap());
    let hash_result = hasher.finalize();
    let event_id = hex::encode(hash_result);

    let secp = Secp256k1::new();
    let id_message = Message::from_slice(&hash_result).unwrap();
    let signature = secp.sign_schnorr_no_aux_rand(&id_message, &user.get_keypair());

    let signed_event = SignedEvent {
        id: event_id,
        pubkey: my_event.pubkey,
        created_at: my_event.created_at,
        kind: my_event.kind,
        tags: my_event.tags,
        content: my_event.content,
        sig: signature.to_string(),
    };

    signed_event
   
}


pub fn create_token_event(content: String, user: UserKeypair) -> SignedEvent {

    // Example values for the event fields
    let created_at = get_unix_timestamp();
    let kind = 29777;
    let tags: Vec<Vec<String>> = vec![];

    let my_event = NostrEvent {
        pubkey: user.public_key.to_string()[2..].to_string(),
        created_at,
        kind,
        tags,
        content
    };
    let value = to_value(&my_event).unwrap();

    let json_str = json!([
        0,
        value["pubkey"],
        value["created_at"],
        value["kind"],
        value["tags"],
        value["content"]
    ]);
    // Serialize the event as JSON

    // Compute the SHA256 hash of the serialized JSON string
    let mut hasher = Sha256::new();
    hasher.update(serde_json::to_string(&json_str).unwrap());
    let hash_result = hasher.finalize();
    let event_id = hex::encode(hash_result);

    let secp = Secp256k1::new();
    let id_message = Message::from_slice(&hash_result).unwrap();
    let signature = secp.sign_schnorr_no_aux_rand(&id_message, &user.get_keypair());

    let signed_event = SignedEvent {
        id: event_id,
        pubkey: my_event.pubkey,
        created_at: my_event.created_at,
        kind: my_event.kind,
        tags: my_event.tags,
        content: my_event.content,
        sig: signature.to_string(),
    };

    signed_event
   
}

