use std::borrow::Cow;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Encoding {
    name: Cow<'static, str>,
    starts_at: u32,
    ends_at: u32,
}

const ENCODINGS:[Encoding;1]=[
    Encoding {
        name: Cow::Borrowed("latin"),
        starts_at: 8,
        ends_at: 16,
    }
];

#[wasm_bindgen]
impl Encoding {
    // TODO There is certainly a way to send back both starting and ending numbers in a single method...
    pub fn get_encoding_starting_character(nom_encodage: &str)->u32 {
        for i in 0..ENCODINGS.len() {
            if ENCODINGS[i].name==nom_encodage {
                return ENCODINGS[i].starts_at;
            }
        }
        return 0;
    }
    pub fn get_encoding_ending_character(nom_encodage: &str)->u32 {
        for i in 0..ENCODINGS.len() {
            if ENCODINGS[i].name==nom_encodage {
                return ENCODINGS[i].ends_at;
            }
        }
        return 0;
    }
}