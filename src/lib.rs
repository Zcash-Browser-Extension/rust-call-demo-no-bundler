mod utils;

use wasm_bindgen::prelude::*;
use std::string;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn test_return() -> String {
    return String::from("rv");
}

#[wasm_bindgen(module = "/defined-in-js.js")]
extern "C" {
    fn name() -> String;
    fn getInfo() -> js_sys::Promise;
}

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn go_action() -> Result<JsValue, JsValue> {
    let promise = getInfo();
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(result)
}

//#[wasm_bindgen]
//pub async fn go_action() -> String {
//    let promise = getInfo();
//    let lightdinfo = wasm_bindgen_futures::JsFuture::from(promise).await;
//
//    match lightdinfo {
//        Ok(js) => {
//            return js.as_string().unwrap();
//       },
//        Err(_e) => {
//            return String::from("error");
//        }
//    }
//
//    log(&format!("Hello from {}!", name())); // should output "Hello from Rust!"
    //return String::from("name_Return_Hardcode");
//}
