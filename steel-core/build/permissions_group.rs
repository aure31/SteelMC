use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;

#[derive(Deserialize, Default)]
struct Group {
    allow: Vec<String>,
    #[serde(default)]
    inherits: Option<String>,
}

impl ToTokens for Group {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let allow = &self.allow;
        let inherits = if let Some(inherits) = self.inherits {
            quote! { #inherits }
        } else {
            TokenStream::new()
        };
        tokens.extend(quote! {
            PermissionGroupConfig {
                allow: vec![#(#allow),*],
                inherits: vec![#inherits]
                ..PermissionGroupConfig::default()
            }
        })
    }
}
pub fn build() -> String {
    println!("cargo:rerun-if-changed=build/permissions_group.json");
    let json = fs::read_to_string("build/permissions_group.json")
        .expect("Failed to read permissions_group.json");
    let raw: BTreeMap<String, Group> =
        serde_json::from_str(&json).expect("Failed to parse permissions_group.json");
    let mut groups = TokenStream::new();
    for (id, group) in raw {
        groups.extend(quote! {
            groups.insert(#id,#group);
        })
    }

    let output = quote! {
        fn build(groups:&mut BTreeMap<String, PermissionGroupConfig>){

        }
    };
    output.to_string()
}
