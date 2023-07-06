
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(ReconcilerDefault)]
pub fn derive_answer_fn(input: TokenStream) -> TokenStream {
    
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = input.ident;

    let expanded = quote! {
        #[async_trait]
        impl Reconciler<v4::VirtualNetworkSpec,v4::VirtualNetworkStatus> for #name {
            fn new(client: Client, group: &'static str, version: &'static str, kind: &'static str) -> Self {
                Self{
                    client,
                    group,
                    version,
                    kind,
                }
            }
            fn group(&self) -> &'static str {
                self.group
            }
            fn version(&self) -> &'static str {
                self.version
            }
            fn kind(&self) -> &'static str {
                self.kind
            }
            fn client(&self) -> Client {
                self.client.clone()
            }
        }
    };
    TokenStream::from(expanded)
}
