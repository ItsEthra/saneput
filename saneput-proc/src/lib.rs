use proc_macro2::TokenStream as TokenStream2;
use proc_macro::{TokenStream, TokenTree};
use quote::quote;

/// Parses standard input.
/// Flushes standard out when called. For more info see crate's root.
#[proc_macro]
pub fn input(cnt: TokenStream) -> TokenStream {
    let v = cnt.into_iter().next().expect("Input macro expects a string");

    match v {
        TokenTree::Literal(l) => {
            let s = l.to_string();
            let mut gs = parse_groups(&s[1..][..s.len() - 2]);

            let out = if gs.len() == 1 {
                let Group { ty, radix } = gs.pop().unwrap();

                quote! {
                    <#ty as ::saneput::FromStdin>::read_cin(&mut _cin, #radix).unwrap()
                }
            } else if gs.len() > 1 {
                let tupitems = gs.into_iter()
                    .map(|Group { ty, radix }| {
                        quote! {
                            <#ty as ::saneput::FromStdin>::read_cin(&mut _cin, #radix).unwrap()
                        }
                    });

                quote! {
                    (
                        #(#tupitems),*
                    )
                }
            } else {
                panic!("Input string must contain at least one group")
            };

            (quote! {
                {
                    <::std::io::Stdout as ::std::io::Write>::flush(&mut std::io::stdout()).unwrap();
                    let mut _cin = ::std::io::stdin();
                    #out
                }
            }).into()
        },
        _ => panic!("Input macro expects a string")
    }
}

struct Group {
    ty: TokenStream2,
    radix: TokenStream2,
}

fn parse_groups<'a>(s: &'a str) -> Vec<Group> {
    let mut current_group = None;
    let mut groups = vec![];

    for (i, c) in s.char_indices() {
        if c == '{' {
            if current_group.is_none() {
                current_group = Some(i+1);
            } else {
                panic!("Unexpected `{{`");
            }
        } else if c == '}' {
            if let Some(cg) = current_group {
                groups.push(parse_single_group(&s[cg..i]));
                current_group = None;
            } else {
                panic!("Unexpected `}}`");
            }
        } else if current_group.is_none() {
            panic!("Unexpected character: `{c}`");
        }
    }

    groups
}

fn parse_single_group(mut s: &str) -> Group {
    if let Some((mut ty, radix)) = s.split_once(':') {
        if ty.is_empty() {
            ty = "i32";
        }
        Group { ty: ty.parse().unwrap(), radix: radix_from_str(radix) }
    } else {
        if s.is_empty() {
            s = "i32";
        }
        Group { ty: s.parse().unwrap(), radix: quote!(::std::option::Option::None) }
    }
}

fn radix_from_str(s: &str) -> TokenStream2 {
    let v = match s {
        "b" => quote!(2),
        "o" => quote!(8),
        "d" => quote!(10),
        "x" => quote!(16),
        _ => panic!("Invalid radix. Expected: `b`, `o`, `d`, `x`")
    };

    quote!(::std::option::Option::Some(#v))
}

