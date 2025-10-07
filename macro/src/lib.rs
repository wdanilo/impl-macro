#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use proc_macro2::Delimiter;
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

fn is_where(tt: &TokenTree) -> bool {
    if let TokenTree::Ident(ident) = tt {
        ident.to_string() == "where"
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

fn is_open_angle(tt: &TokenTree) -> bool {
    if let TokenTree::Punct(punct) = tt {
        punct.as_char() == '<'
    } else {
        false
    }
}

fn is_close_angle(tt: &TokenTree) -> bool {
    if let TokenTree::Punct(punct) = tt {
        punct.as_char() == '>'
    } else {
        false
    }
}

// =================
// === Imp Macro ===
// =================

#[proc_macro]
pub fn imp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();

    let mut prefix = Vec::new();
    let mut targets = Vec::new();
    let mut where_clause = Vec::new();
    let mut body = TokenStream::new();
    let mut tokens = input.into_iter().peekable();

    while !tokens.peek().map(is_for).unwrap_or_default() {
        prefix.push(tokens.next().unwrap());
    }

    if tokens.peek().map(is_for).unwrap_or_default() {
        tokens.next();
    } else {
        panic!("Expected 'for' keyword");
    }

    // Extract for-targets (handle multiple "for ... where ..." patterns)
    while let Some(tt) = tokens.peek() {
        match tt {
            _ if is_where(tt) => {
                let mut clause = Vec::new();
                clause.push(tokens.next().unwrap());
                while !tokens.peek().map(is_brace).unwrap_or_default() {
                    clause.push(tokens.next().unwrap());
                }
                where_clause.push(TokenStream::from_iter(clause));
            }
            _ if is_comma(tt) => {
                tokens.next();
            }
            _ if is_for(tt) => {
                tokens.next();
            }
            _ if is_brace(tt) => {
                break;
            }
            _ => {
                let mut depth: i32 = 0;
                let mut target = Vec::new();
                target.push(tokens.next().unwrap());
                while let Some(tt2) = tokens.peek() {
                    if is_brace(tt2) || is_for(tt2) || is_where(tt2) {
                        break;
                    }
                    if is_open_angle(tt2) {
                        depth += 1;
                    } else if is_close_angle(tt2) {
                        depth -= 1;
                    }
                    if depth == 0 && is_comma(tt2) {
                        break;
                    }
                    target.push(tokens.next().unwrap());
                }
                if target.last().map(is_comma).unwrap_or_default() {
                    target.pop();
                }
                targets.push(TokenStream::from_iter(target));
            }
        }
    }

    if let Some(tt) = tokens.peek() {
        if is_brace(tt) {
            body = tt.clone().into();
            tokens.next();
        }
    } else {
        panic!("Unexpected end of input");
    }

    let prefix = TokenStream::from_iter(prefix);
    let where_clause = TokenStream::from_iter(where_clause);
    let impls = targets.iter().map(|target| {
        quote! { impl #prefix for #target #where_clause #body }
    }).collect::<Vec<_>>();

    let output = quote! {
        #(#impls)*
    };

    // println!("{}", output);
    output.into()
}
