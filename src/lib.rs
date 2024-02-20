#![allow(dead_code)]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Lit, Ident};

#[proc_macro]
pub fn make_opcode(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a literal
    //let input = parse_macro_input!(input as Lit);
    let input_clone = input.clone();
    let input_clone_2 = input.clone();
    let opcode_quote = match syn::parse::<Lit>(input) {
        Ok(lit) => match lit {
        Lit::Int(literal) => {
            quote! {
                Opcode::Value(#literal as i32)
            }
        }
        Lit::Verbatim(literal) => {

            if !literal.to_string().starts_with("OP") {
                quote! {
                    Opcode::Value(#literal as i32)
                }
            } else {
                quote! {
                    HI #literal
                }
            }
        }
        _ => return input_clone,
        },

        Err(_) => match syn::parse::<Ident>(input_clone) {
            Ok(identifier) => {
                if identifier.to_string().starts_with("OP_") {
                    quote! {
                        #identifier
                    }
                } else {
                    quote! {
                        Opcode::Value(#identifier)
                    }
                }

            },
            Err(_) => return input_clone_2,
        }
    };

    opcode_quote.into()
}
