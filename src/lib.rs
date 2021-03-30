use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use std::{collections::HashSet};

extern crate wasm_bindgen;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
/* 
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
*/
//mod summarizer;
//pub use crate::summarizer::summarize;

mod doc_extracter;
pub use crate::doc_extracter::markdown;

#[wasm_bindgen]
pub fn add(n1: i32, n2:i32) -> i32 {
    n1 + n2
}

#[wasm_bindgen]
pub fn markdown_extract(content:&str) -> JsValue {
    let headings = markdown::extract_headings(&content[..]);
    let bolded_words = markdown::extract_bolded(&content[..]);
    let italicised_words = markdown::extract_italicised(&content[..]);
    let colored_words = markdown::extract_colored(&content[..]);
    let block_quotes = markdown::extract_blockquote(&content[..]);

    let mut total = HashSet::new();
    total.extend(headings);
    total.extend(bolded_words);
    total.extend(italicised_words);
    total.extend(colored_words);
    total.extend(block_quotes);
    
    let v: Vec<String> = total.into_iter().collect();
    JsValue::from_serde(&v).unwrap()
}