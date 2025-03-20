#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree, Group, Delimiter};
use quote::quote;

// ===============
// === Helpers ===
// ===============

fn is_brace(tt: &TokenTree) -> bool {
    if let TokenTree::Group(group) = tt {
        group.delimiter() == Delimiter::Brace
    } else {
        false
    }
}

fn is_for(tt: &TokenTree) -> bool {
    if let TokenTree::Ident(ident) = tt {
        ident.to_string() == "for"
    } else {
        false
    }
}

fn is_comma(tt: &TokenTree) -> bool {
    if let TokenTree::Punct(punct) = tt {
        punct.as_char() == ','
    } else {
        false
    }
}

fn is_brace_or_for(tt: &TokenTree) -> bool {
    is_brace(tt) || is_for(tt)
}

// =================
// === Imp Macro ===
// =================

#[proc_macro]
pub fn imp(input: TokenStream) -> TokenStream {
    let input: TokenStream2 = input.into();

    let mut target = Vec::new();
    let mut clauses = Vec::new();
    let mut body = TokenStream2::new();
    let mut tokens = input.into_iter().peekable();

    while !tokens.peek().map(is_for).unwrap_or_default() {
        target.push(tokens.next().unwrap());
    }

    // Extract for-clauses (handle multiple "for ... where ..." patterns)
    while let Some(tt) = tokens.peek() {
        match tt {
            _ if is_for(tt) => {
                let mut clause = Vec::new();
                clause.push(tokens.next().unwrap());
                while !tokens.peek().map(is_brace_or_for).unwrap_or_default() {
                    clause.push(tokens.next().unwrap());
                }
                if clause.last().map(is_comma).unwrap_or_default() {
                    clause.pop();
                }
                clauses.push(TokenStream2::from_iter(clause));
            }
            _ if is_comma(tt) => {
                tokens.next();
            }
            _ if is_brace(tt) => {
                body = tt.clone().into();
                tokens.next();
                break;
            }
            _ => panic!("Unexpected token: {:?}", tt),
        }
    }

    let target = TokenStream2::from_iter(target);
    let impls = clauses.iter().map(|for_clause| {
        quote! { impl #target #for_clause #body }
    }).collect::<Vec<_>>();

    let output = quote! {
        #(#impls)*
    };

    println!("{}", output);
    output.into()
}
