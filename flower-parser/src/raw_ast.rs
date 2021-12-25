use proc_macro2::LexError;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::str::FromStr;
use syn::{
    parse2,
    punctuated::{Pair, Punctuated},
    token::Bracket,
    Token,
};

use serde::{Deserialize, Serialize};

use super::ast::*;
use std::result;

#[derive(Debug)]
pub enum Error {
    SynError(syn::Error),
    LexError(LexError),
}
pub type Result<T> = result::Result<T, Error>;

#[derive(Serialize, Deserialize)]
pub struct RawResource(pub String);
impl From<Resource> for RawResource {
    fn from(r: Resource) -> Self {
        let s = r.0.into_token_stream().to_string();
        RawResource(s)
    }
}
impl TryFrom<RawResource> for Resource {
    type Error = Error;
    fn try_from(rr: RawResource) -> Result<Self> {
        let input = TokenStream::from_str(&rr.0).map_err(Error::LexError)?;
        let ty = parse2(input).map_err(Error::SynError)?;
        Ok(Resource(ty))
    }
}

#[derive(Serialize, Deserialize)]
pub struct RawState(pub String);
impl From<State> for RawState {
    fn from(state: State) -> Self {
        let s = state.0.into_token_stream().to_string();
        RawState(s)
    }
}
impl TryFrom<RawState> for State {
    type Error = Error;
    fn try_from(rs: RawState) -> Result<Self> {
        let input = TokenStream::from_str(&rs.0).map_err(Error::LexError)?;
        let ty = parse2(input).map_err(Error::SynError)?;
        Ok(State(ty))
    }
}

#[derive(Serialize, Deserialize)]
pub struct RawReference {
    pub state: RawState,
    pub mutable: bool,
    pub resource: RawResource,
}
impl From<Reference> for RawReference {
    fn from(reference: Reference) -> Self {
        let state = reference.state.into();
        let mutable = reference.mut_token.is_some();
        let resource = reference.resource.into();
        RawReference {
            state,
            resource,
            mutable,
        }
    }
}
impl TryFrom<RawReference> for Reference {
    type Error = Error;
    fn try_from(rr: RawReference) -> Result<Self> {
        let state = State::try_from(rr.state)?;
        let resource = Resource::try_from(rr.resource)?;
        let mut_token = if rr.mutable {
            Some(Token![mut](Span::call_site()))
        } else {
            None
        };
        let lt_add_token = SubLt(Span::call_site());
        Ok(Reference {
            state,
            resource,
            sub_lt_token: lt_add_token,
            mut_token,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct RawIntermediate(pub String);
impl From<Intermediate> for RawIntermediate {
    fn from(intermediate: Intermediate) -> Self {
        let s = intermediate.0.into_token_stream().to_string();
        RawIntermediate(s)
    }
}
impl TryFrom<RawIntermediate> for Intermediate {
    type Error = Error;
    fn try_from(ri: RawIntermediate) -> Result<Self> {
        let input = TokenStream::from_str(&ri.0).map_err(Error::LexError)?;
        let ty = parse2(input).map_err(Error::SynError)?;
        Ok(Intermediate(ty))
    }
}

#[derive(Serialize, Deserialize)]
pub struct RawTransition {
    pub from: RawState,
    pub intermediate: Option<RawIntermediate>,
    pub to: RawState,
}
impl From<Transition> for RawTransition {
    fn from(transition: Transition) -> Self {
        let from = transition.from.into();
        let intermediate = transition.intermediate.map(|i| i.into());
        let to = transition.to.into();
        RawTransition {
            from,
            intermediate,
            to,
        }
    }
}
impl TryFrom<RawTransition> for Transition {
    type Error = Error;
    fn try_from(rt: RawTransition) -> Result<Self> {
        let from = rt.from.try_into()?;
        let gt_sub_token = GtSub(Span::call_site());
        let intermediate = rt.intermediate.map(|v| v.try_into()).transpose()?;
        let rarrow_token = Token![->](Span::call_site());
        let to = rt.to.try_into()?;

        Ok(Transition {
            from,
            gt_sub_token,
            intermediate,
            rarrow_token,
            to,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct RawOverlay {
    pub back: RawState,
    pub front: RawState,
}
impl From<Overlay> for RawOverlay {
    fn from(overlay: Overlay) -> Self {
        let front = overlay.front.into();
        let back = overlay.back.into();
        RawOverlay { front, back }
    }
}
impl TryFrom<RawOverlay> for Overlay {
    type Error = Error;
    fn try_from(ro: RawOverlay) -> Result<Self> {
        let back = ro.back.try_into()?;
        let front = ro.front.try_into()?;
        let caret_token = Token![^](Span::call_site());
        Ok(Overlay {
            back,
            front,
            caret_token,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct RawFlow {
    pub resources: Vec<RawResource>,
    pub states: Vec<RawState>,
    pub intermediates: Vec<RawIntermediate>,
    pub references: Vec<RawReference>,
    pub transitions: Vec<RawTransition>,
    pub overlays: Vec<RawOverlay>,
}
impl From<Flow> for RawFlow {
    fn from(flow: Flow) -> Self {
        let Flow {
            resources,
            states,
            intermediates,
            references,
            transitions,
            overlays,
        } = flow;
        let resources = resources.punct.into_iter().map(|r| r.into()).collect();
        let states = states.punct.into_iter().map(|r| r.into()).collect();
        let intermediates = intermediates.punct.into_iter().map(|r| r.into()).collect();
        let references = references.punct.into_iter().map(|r| r.into()).collect();
        let transitions = transitions.punct.into_iter().map(|r| r.into()).collect();
        let overlays = overlays.punct.into_iter().map(|r| r.into()).collect();
        RawFlow {
            resources,
            states,
            intermediates,
            references,
            transitions,
            overlays,
        }
    }
}
impl TryFrom<RawFlow> for Flow {
    type Error = Error;
    fn try_from(rf: RawFlow) -> Result<Self> {
        fn v2i<K, T, U>(keyword: K, v: Vec<T>) -> Result<Item<K, U>>
        where
            U: TryFrom<T, Error = Error>,
        {
            let colon_token = Default::default();
            let bracket = Bracket::default();
            let punct = Punctuated::from_iter(
                v.into_iter()
                    .map(|t| t.try_into())
                    .collect::<Result<Vec<U>>>()?
                    .into_iter()
                    .map(|u| Pair::Punctuated(u, Token![,](Span::call_site()))),
            );
            Ok(Item {
                keyword,
                colon_token,
                bracket,
                punct,
            })
        }

        let resources = v2i(kw::resource::default(), rf.resources)?;
        let states = v2i(kw::state::default(), rf.states)?;
        let intermediates = v2i(kw::intermediate::default(), rf.intermediates)?;
        let references = v2i(kw::reference::default(), rf.references)?;
        let transitions = v2i(kw::transition::default(), rf.transitions)?;
        let overlays = v2i(kw::overlay::default(), rf.overlays)?;

        Ok(Flow {
            resources,
            states,
            intermediates,
            references,
            transitions,
            overlays,
        })
    }
}

#[cfg(test)]
mod raw_ast_test {
    use std::str::FromStr;

    use crate::Transition;

    use super::{Flow, Reference};
    use proc_macro2::TokenStream;
    use syn::parse2;

    const FLOW_STR: &str = r#"resource: [A]
state: [S, T]
reference: [S -< A]
transition: [S >- N -> T]
overlay: [S ^ T]
intermediate: [N]
"#;
    #[test]
    fn raw_ast_from_str() {
        let _ = env_logger::try_init();
        let input = TokenStream::from_str(FLOW_STR).unwrap();
        log::debug!("{}", input);
        let flow: Flow = parse2(input).unwrap();
        log::debug!("{:?}", flow);
    }

    const REF_STR: &str = r#"A -< B"#;
    #[test]
    fn reference_from_str() {
        let _ = env_logger::try_init();
        let input = TokenStream::from_str(REF_STR).unwrap();
        log::debug!("{}", input);
        let reference: Reference = parse2(input).unwrap();
        log::debug!("{:?}", reference);
    }

    const TRANSITION_STR: &str = r#"A >--> B"#;
    #[test]
    fn transition_from_str() {
        let _ = env_logger::try_init();
        let input = TokenStream::from_str(TRANSITION_STR).unwrap();
        log::debug!("{}", input);
        let transition: Transition = parse2(input).unwrap();
        log::debug!("{:?}", transition);
    }

    const TRANSITION_INTERMEDIATE_STR: &str = r#"A >- B -> C"#;
    #[test]
    fn transition_intermediate_from_str() {
        let _ = env_logger::try_init();
        let input = TokenStream::from_str(TRANSITION_INTERMEDIATE_STR).unwrap();
        log::debug!("{}", input);
        let transition: Transition = parse2(input).unwrap();
        log::debug!("{:?}", transition);
    }
}
