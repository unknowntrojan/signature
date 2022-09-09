use core::panic;

use findpattern::find_pattern;
use proc_macro::TokenStream;
use syn::{parse::Parser, punctuated::Punctuated, Expr, Lit, Token};

#[proc_macro]
pub fn static_macro(tokens: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;

    let entries = parser.parse(tokens).unwrap();
    let mut image: Option<String> = None;
    let mut pattern: Vec<Option<u8>> = vec![];

    entries.iter().for_each(|f| {
        match f {
            Expr::Lit(f) => match &f.lit {
                Lit::Str(x) => {
                    image = Some(x.value());
                }
                Lit::Int(x) => {
                    pattern.push(Some(x.base10_parse::<u8>().unwrap()));
                }
                _ => {
                    panic!("static_macro: received invalid arguments.");
                }
            },
            Expr::Verbatim(_) => {
                pattern.push(None);
            }
            _ => {
                panic!("static_macro: received invalid arguments.");
            }
        };
    });

    let image = image.unwrap();
    let file = module::get_static(image.to_owned()).expect("static_macro: module not found");

    let offset = find_pattern(file.as_slice(), &pattern).expect("static_macro: pattern not found");
    let sanity = file[offset];

    // need to return:
    // module: String, offset: usize, sanity: u8

    // the first 0xC00 bytes are the header, and not included
    // when querying lpbaseofdll. we adjust for this by using the same
    // base address as dynamicsignature.
    const HEADER_SIZE: usize = 0xC00;

    format!(
        "(String::from(\"{}\"), {}, {:#x})",
        &image,
        offset + HEADER_SIZE,
        sanity
    )
    .parse()
    .unwrap()
}
