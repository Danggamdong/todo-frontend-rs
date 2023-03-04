use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug)]
pub struct State {
    pub todos: Vec<Todo>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct Todo {
    pub id: usize,
    pub description: String,
    pub created_at: i64,
    pub is_finished: bool,
}

#[wasm_bindgen]
impl Todo {
    pub fn to_js_value(&self) -> JsValue {
        JsValue::from_serde(self).unwrap()
    }
}
