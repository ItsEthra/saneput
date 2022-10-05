use proc_macro2::TokenStream as TokenStream2;
use proc_macro::{TokenStream, TokenTree};
use quote::quote;

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
                    {
                        let mut _cin = ::std::io::stdin();
                        <#ty as ::saeput::FromStdin>::read_cin(&mut _cin, ::std::option::Option::Some(#radix)).unwrap()
                    }
                }
            } else if gs.len() > 1 {
                let tupitems = gs.into_iter()
                    .map(|Group { ty, radix }| {
                        quote! {
                            <#ty as ::saeput::FromStdin>::read_cin(&mut _cin, ::std::option::Option::Some(#radix)).unwrap()
                        }
                    });

                quote! {
                    {
                        let mut _cin = ::std::io::stdin();
                        (
                            #(#tupitems),*
                        )
                    }
                }
            } else {
                panic!("Input string must contain at least one group")
            };

            out.into()
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
        Group { ty: s.parse().unwrap(), radix: quote!(::saeput::ExpectedRadix::Dec) }
    }
}

fn radix_from_str(s: &str) -> TokenStream2 {
    match s {
        "b" => quote!(::saeput::ExpectedRadix::Bin),
        "o" => quote!(::saeput::ExpectedRadix::Oct),
        "d" => quote!(::saeput::ExpectedRadix::Dec),
        "x" => quote!(::saeput::ExpectedRadix::Hex),
        _ => panic!("Invalid radix. Expected: `b`, `o`, `d`, `x`")
    }
}

