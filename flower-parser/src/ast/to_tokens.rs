use quote::ToTokens;

use super::Flow;

impl ToTokens for Flow {
    fn to_tokens(&self, _tokens: &mut proc_macro2::TokenStream) {
        unimplemented!()
    }
}
