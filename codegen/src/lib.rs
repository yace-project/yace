/*
 * Permission is hereby granted, free of charge, to any human obtaining a copy of this software and associated documentation files
 * (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify,
 * merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit humans to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
 * OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
 * LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#![allow(uncommon_codepoints)]
#![allow(non_camel_case_types)]

extern crate proc_macro;

use proc_macro::{Delimiter, Group, Ident, TokenStream, TokenTree};

// Note: the use of that macro is a bit unusial. It works like this:
//     𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! {
//         𝓼𝓸𝓶𝓮_𝓸𝓽𝓱𝓮𝓻_𝓶𝓪𝓬𝓻𝓸! {
//             [𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 𝓭𝓪𝓽𝓪_𝓼𝓲𝔃𝓮 𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼…]
//             [… 𝓪𝓭𝓭𝓲𝓭𝓽𝓲𝓸𝓷𝓪𝓵 𝓾𝓷𝓯𝓲𝓵𝓽𝓮𝓻𝓮𝓭 𝓭𝓪𝓽𝓪 …]
//             … 𝓪𝓭𝓭𝓲𝓭𝓽𝓲𝓸𝓷𝓪𝓵 𝓭𝓪𝓽𝓪 𝓽𝓸 𝓯𝓲𝓵𝓽𝓮𝓻 …
//         }
//    }
// 𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! first moves filtered data to unfiltered one, then removes square brackets and expands 𝓼𝓸𝓶𝓮_𝓸𝓽𝓱𝓮𝓻_𝓶𝓪𝓬𝓻𝓸!
// We couldn't use “more obvious” alternative where 𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! would be called from the 𝓼𝓸𝓶𝓮_𝓸𝓽𝓱𝓮𝓻_𝓶𝓪𝓬𝓻𝓸! because in Rust
// macro can only be called in certain, limited, positions and sometimes (e.g. in 𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖙𝖗𝖆𝖎𝖙) we need to filter data in
// some positions where this call is not allowed.
//
// Supported markers:
//     ℜ16 — 8086 data (𝓭𝓪𝓽𝓪_𝓼𝓲𝔃𝓮 == 𝔞𝔡𝔡𝔯16)
//     ℜ32 — 80386 data (𝓭𝓪𝓽𝓪_𝓼𝓲𝔃𝓮 == 𝔞𝔡𝔡𝔯32, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 𝔞𝔡𝔡𝔯64)
//     Ξ16 — 8086 address (𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 𝔞𝔡𝔡𝔯16)
//     Ξ32 — 80386 address (𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 𝔞𝔡𝔡𝔯32)
//     Ξ86 — “legacy” ₓ86 mode (𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 ≠ 𝔞𝔡𝔡𝔯64)
//     Ξ64 — ₓ86_64 mode (𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 𝔞𝔡𝔡𝔯64)
//     Ξ𝔦𝔷 — expaded if ₓ𝔦𝔷 mode requested.
//     Χ𝔦𝔷 — expaded if ₓ𝔦𝔷 mode anot requested.
//     Ξ𝔷𝔷 — expaded if 𝔞𝔡𝔡𝔯64 with ₐᵥₓ512 mode requested.
//     Χ𝔷𝔷 — expaded if 𝔞𝔡𝔡𝔯64 with ₐᵥₓ512 mode not requested.
#[proc_macro]
pub fn 𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘(items: TokenStream) -> TokenStream {
    let mut iter = items.into_iter();
    let macro_name = if let Some(TokenTree::Ident(macro_name)) = iter.next() {
        macro_name
    } else {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't find the name of nested macro.\");"
            .parse()
            .unwrap();
    };
    let exclamation_mark = if let Some(TokenTree::Punct(exclamation_mark)) = iter.next() {
        if exclamation_mark.as_char() == '!' {
            exclamation_mark
        } else {
            return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't find ! after the name of nested macro.\");"
                .parse()
                .unwrap();
        }
    } else {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't find ! after the name of nested macro.\");"
            .parse()
            .unwrap();
    };
    let main_group = if let Some(TokenTree::Group(main_group)) = iter.next() {
        if let Delimiter::Brace = main_group.delimiter() {
            main_group
        } else {
            return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — main group should use braces.\");"
                .parse()
                .unwrap();
        }
    } else {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't find main group to process.\");"
            .parse()
            .unwrap();
    };
    if iter.next().is_some() {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — spurious tokens after main group.\");"
            .parse()
            .unwrap();
    }

    let mut main_group_iter = main_group.stream().into_iter();
    let attributes_group = if let Some(TokenTree::Group(attributes_group)) = main_group_iter.next() {
        if let Delimiter::Bracket = attributes_group.delimiter() {
            attributes_group
        } else {
            return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — attributes_group group should use brackets.\");"
                .parse()
                .unwrap();
        }
    } else {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't find attributes group to process.\");"
            .parse()
            .unwrap();
    };

    let mut attributes_iter = attributes_group.stream().into_iter();
    let attributes = 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬::new(&mut attributes_iter);
    let attributes = match attributes {
        Ok(attributes) => attributes,
        Err(err) => return err.parse().unwrap(),
    };

    let mut unfiletered_data_group_iter = main_group_iter.clone();
    let unfiletered_data_group = match unfiletered_data_group_iter.next() {
        Some(TokenTree::Group(unfiletered_data_group)) if matches!(unfiletered_data_group.delimiter(), Delimiter::Bracket) => {
            main_group_iter = unfiletered_data_group_iter;
            Some(unfiletered_data_group)
        }
        _ => None,
    };
    let mut arguments = TokenStream::new();
    arguments.extend([TokenTree::Group(attributes_group)]);
    if let Some(unfiletered_data_group) = unfiletered_data_group {
        arguments.extend([TokenTree::Group(unfiletered_data_group)]);
    }
    filter_x86_markers_iterable(&mut arguments, &mut main_group_iter, attributes);
    let mut result = TokenStream::new();
    result.extend([
        TokenTree::Ident(macro_name),
        TokenTree::Punct(exclamation_mark),
        Into::<TokenTree>::into(Group::new(Delimiter::Brace, arguments)),
    ]);
    result
}

#[derive(Clone, Copy, Default, Debug)]
struct 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬 {
    𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾: Option<core::num::NonZeroI8>,
    𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾: Option<core::num::NonZeroI8>,
    ₓ𝗂𝗓: Option<i8>,
    𝖺𝗏𝗑𝟧𝟣𝟤: Option<bool>,
}

impl 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬 {
    fn new(input: &mut impl Iterator<Item = TokenTree>) -> Result<Self, &'static str> {
        let mut result: Self = Default::default();
        let mut process = |identifier: &Ident| -> Option<&'static str> {
            match identifier.to_string().as_str() {
                "𝔞𝔡𝔡𝔯16" => {
                    if result.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾.is_some() {
                        return Some("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated address size.\");");
                    } else {
                        result.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 = core::num::NonZeroI8::new(16)
                    }
                }
                "𝔞𝔡𝔡𝔯32" => {
                    if result.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾.is_some() {
                        return Some("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated address size.\");");
                    } else {
                        result.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 = core::num::NonZeroI8::new(32)
                    }
                }
                "𝔞𝔡𝔡𝔯64" => {
                    if result.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾.is_some() {
                        return Some("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated address size.\");");
                    } else {
                        result.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 = core::num::NonZeroI8::new(64)
                    }
                }
                "𝔡𝔞𝔱𝔞16" => {
                    if result.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾.is_some() {
                        return Some("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated data size.\");");
                    } else {
                        result.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾 = core::num::NonZeroI8::new(16)
                    }
                }
                "𝔡𝔞𝔱𝔞32" => {
                    if result.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾.is_some() {
                        return Some("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated data size.\");");
                    } else {
                        result.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾 = core::num::NonZeroI8::new(32)
                    }
                }
                "ₓ𝔦𝔷" => {
                    result.ₓ𝗂𝗓 = match result.ₓ𝗂𝗓 {
                        Some(count) => Some(count + 1),
                        None => Some(1),
                    }
                }
                "ₐᵥₓ512" => {
                    if result.𝖺𝗏𝗑𝟧𝟣𝟤.is_some() {
                        return Some("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated ₐᵥₓ512 marker.\");");
                    } else {
                        result.𝖺𝗏𝗑𝟧𝟣𝟤 = Some(true)
                    }
                }
                _ => return Some("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — unknown token.\");"),
            }
            None
        };
        for token_tree in input {
            match &token_tree {
                TokenTree::Ident(identifier) => {
                    if let Some(err) = process(identifier) {
                        return Err(err);
                    }
                }
                TokenTree::Group(group) if matches!(group.delimiter(), Delimiter::None) => {
                    for token_tree in group.stream().into_iter() {
                        if let TokenTree::Ident(identifier) = &token_tree {
                            if let Some(err) = process(identifier) {
                                return Err(err);
                            }
                        } else {
                            return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — attributes group includes unknown item.\");");
                        }
                    }
                }
                _ => return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — attributes group includes unknown item.\");"),
            }
        }
        Ok(result)
    }
}

fn filter_x86_markers_iterable(
    output: &mut impl Extend<TokenTree>,
    input: &mut impl Iterator<Item = TokenTree>,
    attributes: 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬,
) {
    let mut last_token: Option<TokenTree> = None;
    for token in input {
        if let Some(unwrapped_token) = last_token.take() {
            match token {
                TokenTree::Group(mut data_group_to_process) if matches!(data_group_to_process.delimiter(), Delimiter::Bracket) => {
                    match marker_is_compatible(unwrapped_token.to_string().as_ref(), attributes) {
                        (Some(true), attributes) => {
                            filter_x86_markers_iterable(output, &mut data_group_to_process.stream().into_iter(), attributes)
                        }
                        (Some(false), _) => (),
                        (None, _) => output.extend([
                            unwrapped_token,
                            filter_x86_markers_group(&mut data_group_to_process, attributes),
                        ]),
                    }
                }
                TokenTree::Group(mut data_group_to_process) => output.extend([
                    unwrapped_token,
                    filter_x86_markers_group(&mut data_group_to_process, attributes),
                ]),
                TokenTree::Ident(_) => {
                    output.extend([unwrapped_token]);
                    last_token = Some(token)
                }
                _ => output.extend([unwrapped_token, token]),
            }
        } else if let TokenTree::Ident(_) = token {
            last_token = Some(token)
        } else if let TokenTree::Group(mut data_group_to_process) = token {
            output.extend([filter_x86_markers_group(&mut data_group_to_process, attributes)])
        } else {
            output.extend([token])
        }
    }
    if let Some(unwrapped_token) = last_token.take() {
        output.extend([unwrapped_token])
    }
}

fn filter_x86_markers_group(
    input: &mut Group, attributes: 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬
) -> TokenTree {
    let mut content = TokenStream::new();
    filter_x86_markers_iterable(&mut content, &mut input.stream().into_iter(), attributes);
    Group::new(input.delimiter(), content).into()
}

fn marker_is_compatible(
    marker: &str,
    attributes: 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬,
) -> (Option<bool>, 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬) {
    match marker {
        "ℜ16" => (Some(attributes.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾 == core::num::NonZeroI8::new(16)), attributes),
        "ℜ32" => (Some(attributes.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾 == core::num::NonZeroI8::new(32)), attributes),
        "Ξ16" => (Some(attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 == core::num::NonZeroI8::new(16)), attributes),
        "Ξ32" => (Some(attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 == core::num::NonZeroI8::new(32)), attributes),
        "Ξ86" => (Some(attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 != core::num::NonZeroI8::new(64)), attributes),
        "Ξ64" => (Some(attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 == core::num::NonZeroI8::new(64)), attributes),
        "Ξ𝔦𝔷" => match attributes.ₓ𝗂𝗓 {
            None => (Some(false), attributes),
            Some(1) => (
                Some(true),
                𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬 {
                    𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾: attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾,
                    𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾: attributes.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾,
                    ₓ𝗂𝗓: None,
                    𝖺𝗏𝗑𝟧𝟣𝟤: attributes.𝖺𝗏𝗑𝟧𝟣𝟤,
                },
            ),
            Some(count) => (
                Some(true),
                𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬 {
                    𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾: attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾,
                    𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾: attributes.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾,
                    ₓ𝗂𝗓: Some(count - 1),
                    𝖺𝗏𝗑𝟧𝟣𝟤: attributes.𝖺𝗏𝗑𝟧𝟣𝟤,
                },
            ),
        },
        "Χ𝔦𝔷" => (Some(attributes.ₓ𝗂𝗓 == None), attributes),
        "Ξ𝔷𝔷" => (Some(attributes.𝖺𝗏𝗑𝟧𝟣𝟤 == Some(true)), attributes),
        "Χ𝔷𝔷" => (Some(attributes.𝖺𝗏𝗑𝟧𝟣𝟤 != Some(true)), attributes),
        _ => (None, attributes),
    }
}
