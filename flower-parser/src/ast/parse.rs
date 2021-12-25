use super::{Flow, Intermediate, Overlay, Reference, Resource, State, Transition};
use proc_macro2::Span;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token,
};

mod kw {
    use syn::custom_keyword;
    custom_keyword!(resource);
    custom_keyword!(state);
    custom_keyword!(intermediate);
    custom_keyword!(reference);
    custom_keyword!(transition);
    custom_keyword!(overlay);
}

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
            lt_add_token,
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

impl Parse for Flow {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut resources = Punctuated::new();
        let mut states = Punctuated::new();
        let mut intermediates = Punctuated::new();
        let mut references = Punctuated::new();
        let mut transitions = Punctuated::new();
        let mut overlays = Punctuated::new();

        while !input.is_empty() {
            let lookahead1 = input.lookahead1();
            if lookahead1.peek(kw::resource) {
                let content;
                let _ = input.parse::<kw::resource>()?;
                let _ = input.parse::<Token![:]>()?;
                let _ = bracketed!(content in input);
                let mut rs = Punctuated::parse_terminated_with(&content, Resource::parse)?;
                if !rs.empty_or_trailing() {
                    rs.push_punct(Token![,](Span::call_site()));
                }
                resources.extend(rs.into_pairs());
            } else if lookahead1.peek(kw::state) {
                let content;
                let _ = input.parse::<kw::state>()?;
                let _ = input.parse::<Token![:]>()?;
                let _ = bracketed!(content in input);
                let mut ss = Punctuated::parse_terminated_with(&content, State::parse)?;
                if !ss.empty_or_trailing() {
                    ss.push_punct(Token![,](Span::call_site()))
                }
                states.extend(ss.into_pairs());
            } else if lookahead1.peek(kw::intermediate) {
                let content;
                let _ = input.parse::<kw::intermediate>()?;
                let _ = input.parse::<Token![:]>()?;
                let _ = bracketed!(content in input);
                let mut is = Punctuated::parse_terminated_with(&content, Intermediate::parse)?;
                if !is.empty_or_trailing() {
                    is.push_punct(Token![,](Span::call_site()))
                }
                intermediates.extend(is.into_pairs());
            } else if lookahead1.peek(kw::reference) {
                let content;
                let _ = input.parse::<kw::reference>()?;
                let _ = input.parse::<Token![:]>()?;
                let _ = bracketed!(content in input);
                let mut rs = Punctuated::parse_terminated_with(&content, Reference::parse)?;
                if !rs.empty_or_trailing() {
                    rs.push_punct(Token![,](Span::call_site()))
                }
                references.extend(rs.into_pairs());
            } else if lookahead1.peek(kw::transition) {
                let content;
                let _ = input.parse::<kw::transition>()?;
                let _ = input.parse::<Token![:]>()?;
                let _ = bracketed!(content in input);
                let mut ts = Punctuated::parse_terminated_with(&content, Transition::parse)?;
                if !ts.empty_or_trailing() {
                    ts.push_punct(Token![,](Span::call_site()))
                }
                transitions.extend(ts.into_pairs());
            } else if lookahead1.peek(kw::overlay) {
                let content;
                let _ = input.parse::<kw::overlay>()?;
                let _ = input.parse::<Token![:]>()?;
                let _ = bracketed!(content in input);
                let mut os = Punctuated::parse_terminated_with(&content, Overlay::parse)?;
                if !os.empty_or_trailing() {
                    os.push_punct(Token![,](Span::call_site()))
                }
                overlays.extend(os.into_pairs());
            } else {
                return Err(lookahead1.error());
            }
        }
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
