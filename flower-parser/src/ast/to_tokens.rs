use quote::ToTokens;

use super::Flow;
use proc_macro2::TokenStream;
use quote::quote;

use super::*;

impl ToTokens for Resource {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

impl ToTokens for State {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

impl ToTokens for Intermediate {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

impl ToTokens for Reference {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let state = &self.state;
        let lt_add_token = &self.sub_lt_token;
        let mut_token = &self.mut_token;
        let resource = &self.resource;

        tokens.extend(quote! {
            #state #lt_add_token #mut_token #resource
        });
    }
}

impl ToTokens for Transition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let from = &self.from;
        let gt_sub_token = &self.gt_sub_token;
        let intermediate = &self.intermediate;
        let rarrow_token = &self.rarrow_token;
        let to = &self.to;

        tokens.extend(quote! {
            #from #gt_sub_token #intermediate #rarrow_token #to
        })
    }
}

impl ToTokens for Overlay {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let back = &self.back;
        let caret_token = &self.caret_token;
        let front = &self.front;
        tokens.extend(quote! {
            #back #caret_token #front
        });
    }
}

impl<K, T, P> ToTokens for Item<K, T, P>
where
    K: ToTokens,
    T: ToTokens,
    P: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let keyword = &self.keyword;
        let punct = &self.punct;
        let bracket = &self.bracket;
        let colon_token = &self.colon_token;

        tokens.extend(quote! { #keyword #colon_token });
        bracket.surround(tokens, |tokens| punct.to_tokens(tokens));
    }
}

impl ToTokens for Flow {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.resources.to_tokens(tokens);
        self.states.to_tokens(tokens);
        self.intermediates.to_tokens(tokens);
        self.references.to_tokens(tokens);
        self.transitions.to_tokens(tokens);
        self.overlays.to_tokens(tokens);
    }
}
