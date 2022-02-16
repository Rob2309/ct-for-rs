use proc_macro2::Group;
use quote::{quote, quote_spanned};

/// Expands a code snippet `n` times and substitutes an identifier with given values
/// 
/// # Example
/// ```
/// use ct_for::ct_for;
/// 
/// let c = 8;
/// ct_for!(x in ["5", 6, c] do
///     println!("{}", x);
/// );
/// // expands to
/// println!("{}", "5");
/// println!("{}", 6);
/// println!("{}", c);
/// ```
/// 
#[proc_macro]
pub fn ct_for(args: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = proc_macro2::TokenStream::from(args);
    let mut iter = args.into_iter();

    let subst_name = match iter.next() {
        Some(proc_macro2::TokenTree::Ident(ident)) => ident,
        Some(tok) => return error_span("Expected identifier", tok.span()),
        None => return error("Unexpected end of macro"),
    };

    match iter.next() {
        Some(proc_macro2::TokenTree::Ident(ident)) if ident == "in" => {}
        Some(tok) => return error_span("Expected 'in'", tok.span()),
        None => return error("Unexpected end of macro"),
    }

    let subst_group = match iter.next() {
        Some(proc_macro2::TokenTree::Group(group)) => group,
        Some(tok) => return error_span("Expected (...)", tok.span()),
        None => return error("Unexpected end of macro"),
    };

    let subst_values = collect_subst_values(subst_group.stream().into_iter());

    match iter.next() {
        Some(proc_macro2::TokenTree::Ident(ident)) if ident == "do" => {}
        Some(tok) => return error_span("Expected 'do'", tok.span()),
        None => return error("Unexpected end of macro"),
    }

    let mut res = proc_macro2::TokenStream::new();

    for subst in subst_values {
        let iter = iter.clone();
        res.extend(subst_recursive(iter, subst_name.clone(), subst));
    }

    res.into()
}

fn collect_subst_values(
    iter: impl Iterator<Item = proc_macro2::TokenTree>,
) -> Vec<proc_macro2::TokenStream> {
    let mut res = Vec::new();

    let mut val = proc_macro2::TokenStream::new();
    for tok in iter {
        match tok {
            proc_macro2::TokenTree::Punct(punct) if punct.as_char() == ',' => {
                res.push(val);
                val = proc_macro2::TokenStream::new();
            }
            _ => {
                val.extend(std::iter::once(tok));
            }
        }
    }
    if !val.is_empty() {
        res.push(val);
    }

    res
}

fn subst_recursive(
    iter: impl Iterator<Item = proc_macro2::TokenTree>,
    subst_name: proc_macro2::Ident,
    subst_value: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let mut res = proc_macro2::TokenStream::new();

    for tok in iter {
        match tok {
            proc_macro2::TokenTree::Group(group) => {
                let group_tokens = subst_recursive(
                    group.stream().into_iter(),
                    subst_name.clone(),
                    subst_value.clone(),
                );
                res.extend(std::iter::once(proc_macro2::TokenTree::Group(Group::new(
                    group.delimiter(),
                    group_tokens,
                ))));
            }
            proc_macro2::TokenTree::Ident(ident) if ident == subst_name => {
                res.extend(subst_value.clone());
            }
            _ => {
                res.extend(std::iter::once(tok));
            }
        }
    }

    res
}

fn error(msg: &str) -> proc_macro::TokenStream {
    quote! {
        compile_error!(#msg);
    }
    .into()
}

fn error_span(msg: &str, span: proc_macro2::Span) -> proc_macro::TokenStream {
    quote_spanned! {span=>
        compile_error!(#msg);
    }
    .into()
}
