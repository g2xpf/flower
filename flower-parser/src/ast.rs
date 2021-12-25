mod parse;
mod to_tokens;

use std::str::FromStr;

use proc_macro2::TokenStream;
use syn::{custom_punctuation, parse2, punctuated::Punctuated, Token, TypePath};
custom_punctuation!(SubLt, -<);
custom_punctuation!(GtSub, >-);

#[derive(Debug)]
pub struct Resource(pub TypePath);

#[derive(Debug)]
pub struct State(pub TypePath);

#[derive(Debug)]
pub struct Reference {
    pub state: State,
    pub lt_add_token: SubLt,
    pub mut_token: Option<Token![mut]>,
    pub resource: Resource,
}

#[derive(Debug)]
pub struct Intermediate(pub TypePath);

#[derive(Debug)]
pub struct Transition {
    pub from: State,
    pub gt_sub_token: GtSub,
    pub intermediate: Option<Intermediate>,
    pub rarrow_token: Token![->],
    pub to: State,
}

#[derive(Debug)]
pub struct Overlay {
    pub back: State,
    pub caret_token: Token![^],
    pub front: State,
}

#[derive(Debug)]
pub struct Flow {
    pub resources: Punctuated<Resource, Token![,]>,
    pub states: Punctuated<State, Token![,]>,
    pub intermediates: Punctuated<Intermediate, Token![,]>,
    pub references: Punctuated<Reference, Token![,]>,
    pub transitions: Punctuated<Transition, Token![,]>,
    pub overlays: Punctuated<Overlay, Token![,]>,
}

pub enum ParseError {
    LexError(proc_macro2::LexError),
    SynError(syn::Error),
}

impl FromStr for Flow {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token = TokenStream::from_str(s).map_err(ParseError::LexError)?;
        let flow = parse2(token).map_err(ParseError::SynError)?;
        Ok(flow)
    }
}
