use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    parse2,
    punctuated::{Pair, Punctuated},
    Token,
};

use serde::{Deserialize, Serialize};

use super::ast::*;
use std::result;

#[derive(Debug)]
pub enum Error {
    SynError(syn::Error),
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
        let input = rr.0.into_token_stream();
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
        let input = rs.0.into_token_stream();
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
            lt_add_token,
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
        let input = ri.0.into_token_stream();
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
        let resources = resources.into_iter().map(|r| r.into()).collect();
        let states = states.into_iter().map(|r| r.into()).collect();
        let intermediates = intermediates.into_iter().map(|r| r.into()).collect();
        let references = references.into_iter().map(|r| r.into()).collect();
        let transitions = transitions.into_iter().map(|r| r.into()).collect();
        let overlays = overlays.into_iter().map(|r| r.into()).collect();
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
        fn v2p<T, U>(v: Vec<T>) -> Result<Punctuated<U, Token![,]>>
        where
            U: TryFrom<T, Error = Error>,
        {
            Ok(Punctuated::from_iter(
                v.into_iter()
                    .map(|t| t.try_into())
                    .collect::<Result<Vec<U>>>()?
                    .into_iter()
                    .map(|u| Pair::Punctuated(u, Token![,](Span::call_site()))),
            ))
        }

        let resources = v2p(rf.resources)?;
        let states = v2p(rf.states)?;
        let intermediates = v2p(rf.intermediates)?;
        let references = v2p(rf.references)?;
        let transitions = v2p(rf.transitions)?;
        let overlays = v2p(rf.overlays)?;

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
