use std::borrow::Cow;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
/*
struct Encoding{
    name: Cow<'static, str>,
    starts_at: u32,
    ends_at: u32,
}

const ENCODINGS: [Encoding; 1] = [
    Encoding{
        name: Cow::Borrowed("hello"),
        starts_at: 8,
        ends_at: 10,
    }
];
*/
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}


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
    /*
    #[wasm_bindgen(constructor)]
    pub fn new(val: i32) -> Encoding {
        Encoding {
            name: Cow::Borrowed("hello"),
            starts_at: val,
            ends_at: val+6,
        }
    }

    pub fn get_start(&self) -> i32 {
        self.starts_at
    }

    pub fn get_end(&self)->i32 {
        self.ends_at
    }*/
    pub fn parcours(nom_encodage: &str)->u32 {
        let mut resultat:u32=0;
        for i in 0..ENCODINGS.len() {
            resultat = ENCODINGS[i].starts_at;
        }
        return resultat;
    }
}