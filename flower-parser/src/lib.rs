mod ast;
mod raw_ast;

pub use ast::{Flow, Intermediate, Overlay, Reference, Resource, State, Transition};
use quote::ToTokens;
pub use raw_ast::RawFlow;
use syn::parse2;
use wasm_bindgen::prelude::*;
use wasm_bindgen::UnwrapThrowExt;

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn flow2json(flowString: String) -> String {
    let input = flowString.to_token_stream();
    let flow: Flow = parse2(input).unwrap_throw();
    let raw_flow: RawFlow = flow.into();
    serde_json::to_string(&raw_flow).unwrap_throw()
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn json2flow(jsonString: String) -> String {
    let raw_flow: RawFlow = serde_json::from_str(&jsonString).unwrap_throw();
    let flow = Flow::try_from(raw_flow).unwrap_throw();
    flow.into_token_stream().to_string()
}
