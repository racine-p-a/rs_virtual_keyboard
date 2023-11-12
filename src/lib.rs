use std::borrow::Cow;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

/*
#[wasm_bindgen]
pub fn encodings(chosen_encoding: &str)->(u32, u32){
    return (7,9);
}*/



#[wasm_bindgen]
pub struct Datum {
    internal: i32,
    external: i32,
}

#[wasm_bindgen]
pub struct Titi {
    internal: i32,
    external: i32,
}

#[wasm_bindgen]
impl Titi {
    #[wasm_bindgen(constructor)]
    pub fn new(val: i32) -> Titi {
        Titi {
            internal: val,
            external: val+6,
        }
    }

    pub fn get_start(&self) -> i32 {
        self.internal
    }

    pub fn get_end(&self)->i32 {
        self.external
    }
}