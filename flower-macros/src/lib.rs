use flower_parser::Flow;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn flow(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let flow = parse_macro_input!(input as Flow);
    log::debug!("{:?}", flow);
    /* (quote! {
        #flow
    })
    .into() */
    (quote! {}).into()
}
