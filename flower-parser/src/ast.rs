mod parse;
mod to_tokens;

pub mod kw {
    use syn::custom_keyword;
    custom_keyword!(resource);
    custom_keyword!(state);
    custom_keyword!(intermediate);
    custom_keyword!(reference);
    custom_keyword!(transition);
    custom_keyword!(overlay);
}

use std::str::FromStr;

use proc_macro2::TokenStream;
use syn::{
    custom_punctuation, parse2,
    punctuated::Punctuated,
    token::{Bracket, Colon},
    Token, TypePath,
};
custom_punctuation!(SubLt, -<);
custom_punctuation!(GtSub, >-);

#[derive(Debug)]
pub struct Resource(pub TypePath);

#[derive(Debug)]
pub struct State(pub TypePath);

#[derive(Debug)]
pub struct Reference {
    pub state: State,
    pub sub_lt_token: SubLt,
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
pub struct Item<K, T, P = Token![,]> {
    pub keyword: K,
    pub colon_token: Colon,
    pub bracket: Bracket,
    pub punct: Punctuated<T, P>,
}

#[derive(Debug)]
pub struct Flow {
    pub resources: Item<kw::resource, Resource>,
    pub states: Item<kw::state, State>,
    pub intermediates: Item<kw::intermediate, Intermediate>,
    pub references: Item<kw::reference, Reference>,
    pub transitions: Item<kw::transition, Transition>,
    pub overlays: Item<kw::overlay, Overlay>,
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
