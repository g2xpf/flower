use super::{kw, Flow, Intermediate, Item, Overlay, Reference, Resource, State, Transition};
use proc_macro2::Span;
use std::fmt;
use syn::Error;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token,
};

impl Parse for Resource {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty = input.parse()?;
        Ok(Resource(ty))
    }
}

impl Parse for State {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty = input.parse()?;
        Ok(State(ty))
    }
}

impl Parse for Reference {
    fn parse(input: ParseStream) -> Result<Self> {
        let state = input.parse()?;
        let lt_add_token = input.parse()?;
        let mut_token = input.parse()?;
        let resource = input.parse()?;
        Ok(Reference {
            state,
            sub_lt_token: lt_add_token,
            mut_token,
            resource,
        })
    }
}

impl Parse for Intermediate {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty = input.parse()?;
        Ok(Intermediate(ty))
    }
}

impl Parse for Transition {
    fn parse(input: ParseStream) -> Result<Self> {
        let from = input.parse()?;
        let gt_sub_token = input.parse()?;
        let intermediate = if !input.peek(Token![-]) {
            Some(input.parse()?)
        } else {
            None
        };
        let rarrow_token = input.parse()?;
        let to = input.parse()?;
        Ok(Transition {
            from,
            gt_sub_token,
            intermediate,
            rarrow_token,
            to,
        })
    }
}

impl Parse for Overlay {
    fn parse(input: ParseStream) -> Result<Self> {
        let back = input.parse()?;
        let caret_token = input.parse()?;
        let front = input.parse()?;
        Ok(Overlay {
            back,
            caret_token,
            front,
        })
    }
}

impl<K, T, P> Parse for Item<K, T, P>
where
    K: Parse,
    T: Parse,
    P: Parse + Default,
{
    fn parse(input: ParseStream) -> Result<Self> {
        let keyword = input.parse()?;
        let colon_token = input.parse()?;
        let content;
        let bracket = bracketed!(content in input);
        let mut punct = Punctuated::parse_terminated_with(&content, T::parse)?;
        if !punct.empty_or_trailing() {
            punct.push_punct(P::default());
        }
        Ok(Item {
            keyword,
            colon_token,
            bracket,
            punct,
        })
    }
}

impl Parse for Flow {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut resources = None;
        let mut states = None;
        let mut intermediates = None;
        let mut references = None;
        let mut transitions = None;
        let mut overlays = None;

        while !input.is_empty() {
            let lookahead1 = input.lookahead1();
            if lookahead1.peek(kw::resource) {
                resources = Some(input.parse()?);
            } else if lookahead1.peek(kw::state) {
                states = Some(input.parse()?);
            } else if lookahead1.peek(kw::intermediate) {
                intermediates = Some(input.parse()?);
            } else if lookahead1.peek(kw::reference) {
                references = Some(input.parse()?);
            } else if lookahead1.peek(kw::transition) {
                transitions = Some(input.parse()?);
            } else if lookahead1.peek(kw::overlay) {
                overlays = Some(input.parse()?);
            } else {
                return Err(lookahead1.error());
            }
        }

        fn item_not_given_error<K: FnOnce(Span) -> D, D: fmt::Debug>(keyword: K) -> Error {
            Error::new(
                Span::call_site(),
                format!(
                    "required item not given: `{:?}`",
                    keyword(Span::call_site())
                ),
            )
        }
        let resources = resources.ok_or_else(|| item_not_given_error(kw::resource))?;
        let states = states.ok_or_else(|| item_not_given_error(kw::state))?;
        let references = references.ok_or_else(|| item_not_given_error(kw::reference))?;
        let transitions = transitions.ok_or_else(|| item_not_given_error(kw::transition))?;
        let overlays = overlays.ok_or_else(|| item_not_given_error(kw::overlay))?;
        let intermediates = intermediates.ok_or_else(|| item_not_given_error(kw::intermediate))?;

        Ok(Flow {
            resources,
            states,
            references,
            transitions,
            overlays,
            intermediates,
        })
    }
}
