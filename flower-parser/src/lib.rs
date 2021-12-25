mod ast;
mod raw_ast;

pub use ast::{Flow, Intermediate, Overlay, Reference, Resource, State, Transition};
use proc_macro2::TokenStream;
use quote::ToTokens;
pub use raw_ast::RawFlow;
use std::str::FromStr;
use syn::parse2;
use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn flow2json(flowString: String) -> String {
    let input = TokenStream::from_str(&flowString).unwrap_or_else(|e| {
        let error_message = format!("{}", e);
        throw_str(&error_message)
    });
    let flow: Flow = parse2(input).unwrap_or_else(|e| {
        let error_message = format!("{}", e);
        throw_str(&error_message)
    });
    let raw_flow: RawFlow = flow.into();
    serde_json::to_string(&raw_flow).unwrap_or_else(|e| {
        let error_message = format!("{}", e);
        throw_str(&error_message)
    })
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn json2flow(jsonString: String) -> String {
    let raw_flow: RawFlow = serde_json::from_str(&jsonString).unwrap_or_else(|e| {
        let error_message = format!("{}", e);
        throw_str(&error_message)
    });
    let flow = Flow::try_from(raw_flow).unwrap_or_else(|e| {
        let error_message = format!("{:?}", e);
        throw_str(&error_message);
    });
    flow.into_token_stream().to_string()
}
