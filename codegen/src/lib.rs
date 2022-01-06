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
#![allow(non_upper_case_globals)]

extern crate proc_macro;

#[macro_use]
extern crate maplit;

use futures::TryStreamExt;
use lazy_static::lazy_static;
use proc_macro::{Delimiter, Group, Ident, TokenStream, TokenTree};
use sqlx::{Connection, Row};

// Note: the use of that macro is a bit unusial. It works like this:
//     𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! {
//         𝓼𝓸𝓶𝓮_𝓸𝓽𝓱𝓮𝓻_𝓶𝓪𝓬𝓻𝓸! {
//             [𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 𝓭𝓪𝓽𝓪_𝓼𝓲𝔃𝓮 𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼…]
//             … 𝓭𝓪𝓽𝓪 𝓽𝓸 𝓯𝓲𝓵𝓽𝓮𝓻 …
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
// Additional expandable markers:
//     𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟𝕤𝕀𝕟𝕥𝕖𝕣𝕗𝕒𝕔𝕖 — List of instructions from SQL database
//     𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟𝕤𝔽𝕠𝕣𝕨𝕒𝕣𝕕𝕖𝕣𝕤 — Forwarders from ₓₓₓ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏 to ₓₓₓ_𝒍𝒆𝒈𝒂𝒄𝒚_𝒎𝒐𝒅𝒆_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏/ₓₓₓ_ₓ86_64_𝒎𝒐𝒅𝒆_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏.
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
    let mut attributes_iter = main_group_iter.clone();
    let attributes_group = if let Some(TokenTree::Group(attributes_group)) = attributes_iter.next() {
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

    let 𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌 =
        match 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐞𝐱𝐭𝐫𝐚_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬::new(
            &mut attributes_iter,
        ) {
            Ok(𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌) => {
                𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌
            }
            Err(err) => return err.parse().unwrap(),
        };
    let mut attributes_iter = attributes_group.stream().into_iter();
    let attributes = match 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬::new(
        &mut attributes_iter,
        &𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌,
    ) {
        Ok(attributes) => attributes,
        Err(err) => return err.parse().unwrap(),
    };

    let mut arguments = TokenStream::new();
    filter_x86_markers_iterable(&mut arguments, &mut main_group_iter, attributes);
    let mut result = TokenStream::new();
    result.extend([
        TokenTree::Ident(macro_name),
        TokenTree::Punct(exclamation_mark),
        Into::<TokenTree>::into(Group::new(Delimiter::Brace, arguments)),
    ]);
    result
}

#[derive(Clone, Copy, Debug)]
struct 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬<'ᵉˣᵗʳᵃ> {
    𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾: Option<core::num::NonZeroI8>,
    𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾: Option<core::num::NonZeroI8>,
    ₓ𝗂𝗓: Option<i8>,
    𝖺𝗏𝗑𝟧𝟣𝟤: Option<bool>,
    𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌: &'ᵉˣᵗʳᵃ 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐞𝐱𝐭𝐫𝐚_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬,
}

impl<'ᵉˣᵗʳᵃ> 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬<'ᵉˣᵗʳᵃ> {
    fn new(
        input: &mut impl Iterator<Item = TokenTree>,
        𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌: &'ᵉˣᵗʳᵃ 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐞𝐱𝐭𝐫𝐚_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬,
    ) -> Result<Self, &'static str> {
        let mut result: Self = 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬 {
            𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾: None,
            𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾: None,
            ₓ𝗂𝗓: None,
            𝖺𝗏𝗑𝟧𝟣𝟤: None,
            𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌,
        };
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

#[derive(Clone, Default, Debug)]
struct 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐞𝐱𝐭𝐫𝐚_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬 {
    𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾: Option<TokenTree>,
    𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾: Option<TokenTree>,
    𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇: Option<TokenStream>,
    𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼: Option<TokenStream>,
}

impl 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐞𝐱𝐭𝐫𝐚_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬 {
    // Note: it's not an error to have unparseable data after initial, mandatory, group.
    // We just don't get extra info in that case.
    fn new(input: &mut impl Iterator<Item = TokenTree>) -> Result<Self, &'static str> {
        match input.next() {
            Some(TokenTree::Ident(impl_ident)) if impl_ident.to_string() == "impl" => (),
            _ => return Ok(Default::default()),
        }
        let (restrictions_stream, mut next_item) = match input.next() {
            Some(TokenTree::Group(restrictions_group)) if matches!(restrictions_group.delimiter(), Delimiter::Bracket) => {
                (Some(restrictions_group.stream()), input.next())
            }
            next_item => (None, next_item),
        };
        let 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 = match next_item {
            Some(TokenTree::Ident(_)) => next_item,
            Some(TokenTree::Group(trait_group)) if matches!(trait_group.delimiter(), Delimiter::None) => {
                let mut trait_iter = trait_group.stream().into_iter();
                let 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 = trait_iter.next();
                if let Some(TokenTree::Ident(_)) = 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 {
                    if !trait_iter.next().is_none() {
                        return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't parse optional data.\");");
                    }
                    𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾
                } else {
                    return Ok(Default::default());
                }
            }
            _ => return Ok(Default::default()),
        };
        match input.next() {
            Some(TokenTree::Ident(for_ident)) if for_ident.to_string() == "for" => (),
            _ => return Ok(Default::default()),
        }
        next_item = input.next();
        let 𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾 = match next_item {
            Some(TokenTree::Ident(_)) => next_item,
            Some(TokenTree::Group(struct_group)) if matches!(struct_group.delimiter(), Delimiter::None) => {
                let mut struct_iter = struct_group.stream().into_iter();
                let 𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾 = struct_iter.next();
                if let Some(TokenTree::Ident(_)) = 𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾 {
                    if !struct_iter.next().is_none() {
                        return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't parse optional data.\");");
                    }
                    𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾
                } else {
                    return Ok(Default::default());
                }
            }
            _ => return Ok(Default::default()),
        };
        let params_stream = match input.next() {
            Some(TokenTree::Group(params_group)) if matches!(params_group.delimiter(), Delimiter::Bracket) => {
                Some(params_group.stream())
            }
            _ => None,
        };
        Ok(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐞𝐱𝐭𝐫𝐚_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬 {
            𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾,
            𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾,
            𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇: restrictions_stream,
            𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼: params_stream,
        })
    }
}

fn filter_x86_markers_iterable(
    output: &mut impl Extend<TokenTree>,
    input: &mut impl Iterator<Item = TokenTree>,
    attributes: 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬,
) {
    fn emit_or_expand_token(
        output: &mut impl Extend<TokenTree>, token: TokenTree, attributes: 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬
    ) {
        if let TokenTree::Ident(ref ident) = token {
            match ident.to_string().as_str() {
                "𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟𝕤𝕀𝕟𝕥𝕖𝕣𝕗𝕒𝕔𝕖" => {
                    let additional_info: TokenStream = if attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 != core::num::NonZeroI8::new(64)
                    {
                        𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.0.parse().unwrap()
                    } else {
                        𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.1.parse().unwrap()
                    };
                    output.extend(additional_info)
                }
                "𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟𝕤𝔽𝕠𝕣𝕨𝕒𝕣𝕕𝕖𝕣𝕤" => {
                    if attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 != core::num::NonZeroI8::new(64) {
                        let token_stream: TokenStream = format!(
                            concat!(
                                "impl Æ 𝒍𝒆𝒈𝒂𝒄𝒚_𝒎𝒐𝒅𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓 for æ",
                                "{{type 𝐝𝐚𝐭𝐚_𝐩𝐫𝐞𝐟𝐢𝐱_16ᵇⁱᵗ={};type 𝐝𝐚𝐭𝐚_𝐩𝐫𝐞𝐟𝐢𝐱_32ᵇⁱᵗ={};",
                                "type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐩𝐫𝐞𝐟𝐢𝐱_16ᵇⁱᵗ={};type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐩𝐫𝐞𝐟𝐢𝐱_32ᵇⁱᵗ={};}}"
                            ),
                            if attributes.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾 == core::num::NonZeroI8::new(16) {
                                "𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞"
                            } else {
                                "𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x66>"
                            },
                            if attributes.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾 == core::num::NonZeroI8::new(16) {
                                "𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x66>"
                            } else {
                                "𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞"
                            },
                            if attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 == core::num::NonZeroI8::new(16) {
                                "𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞"
                            } else {
                                "𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x67>"
                            },
                            if attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 == core::num::NonZeroI8::new(16) {
                                "𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x67>"
                            } else {
                                "𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞"
                            }
                        )
                        .parse()
                        .unwrap();
                        for token in token_stream.into_iter() {
                            match token {
                                TokenTree::Ident(ref ident) if ident.to_string() == "Æ" => {
                                    if let Some(ref 𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇) = attributes.𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌.𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇
                                    {
                                        output.extend(𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇.clone().into_iter())
                                    }
                                }
                                TokenTree::Ident(ref ident) if ident.to_string() == "æ" => {
                                    if let Some(ref 𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾) = attributes.𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌.𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾
                                    {
                                        output.extend([𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾.clone()])
                                    }
                                    if let Some(ref 𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼) = attributes.𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌.𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼
                                    {
                                        output.extend(𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼.clone().into_iter())
                                    }
                                }
                                _ => output.extend([token]),
                            }
                        }
                    }
                    let token_stream: TokenStream = if attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 == core::num::NonZeroI8::new(64)
                    {
                        format!(
                            concat!(
                                "impl Æ 𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>𝒂𝒅𝒅_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮> for æ ",
                                "where Self:{1}_ₓ86_64_𝒎𝒐𝒅𝒆_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>{{",
                                "type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞=<Self as {1}_ₓ86_64_𝒎𝒐𝒅𝒆_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;",
                                "type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞=<Self as {1}_ₓ86_64_𝒎𝒐𝒅𝒆_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;",
                                "fn {0}_implementation(&mut self,arguments:𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮)->",
                                "Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>{{self.{0}_x86_64_mode(arguments)}}}}"
                            ),
                            "add",
                            𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾("add")
                        )
                    } else {
                        format!(
                            concat!(
                                "impl Æ 𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>𝒂𝒅𝒅_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮> for æ ",
                                "where Self:{1}_𝒍𝒆𝒈𝒂𝒄𝒚_𝒎𝒐𝒅𝒆_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>{{",
                                "type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞=<Self as {1}_𝒍𝒆𝒈𝒂𝒄𝒚_𝒎𝒐𝒅𝒆_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;",
                                "type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞=<Self as {1}_𝒍𝒆𝒈𝒂𝒄𝒚_𝒎𝒐𝒅𝒆_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;",
                                "fn {0}_implementation(&mut self,arguments:𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮)->",
                                "Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>{{self.{0}_legacy_mode(arguments)}}}}"
                            ),
                            "add",
                            𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾("add")
                        )
                    }
                    .parse()
                    .unwrap();
                    for token in token_stream.into_iter() {
                        match token {
                            TokenTree::Ident(ref ident) if ident.to_string() == "Æ" => {
                                if let Some(ref 𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇) = attributes.𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌.𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇
                                {
                                    let mut previous_token = None;
                                    for token in 𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇.clone().into_iter() {
                                        if let Some(previous_token) = previous_token.replace(token) {
                                            output.extend([previous_token]);
                                        }
                                    }
                                    let token_stream: TokenStream = ",".parse().unwrap();
                                    output.extend(token_stream.into_iter());
                                } else {
                                    let token_stream: TokenStream = "<".parse().unwrap();
                                    output.extend(token_stream.into_iter());
                                }
                            }
                            TokenTree::Ident(ref ident) if ident.to_string() == "æ" => {
                                if let Some(ref 𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾) = attributes.𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌.𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾
                                {
                                    output.extend([𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾.clone()])
                                }
                                if let Some(ref 𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼) = attributes.𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌.𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼
                                {
                                    output.extend(𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼.clone().into_iter())
                                }
                            }
                            _ => output.extend([token]),
                        }
                    }
                }
                _ => output.extend([token]),
            }
        } else {
            output.extend([token])
        }
    }
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
                        (None, _) => {
                            emit_or_expand_token(output, unwrapped_token, attributes);
                            output.extend([filter_x86_markers_group(&mut data_group_to_process, attributes)])
                        }
                    }
                }
                TokenTree::Group(mut data_group_to_process) => {
                    emit_or_expand_token(output, unwrapped_token, attributes);
                    output.extend([filter_x86_markers_group(&mut data_group_to_process, attributes)])
                }
                TokenTree::Ident(_) => {
                    emit_or_expand_token(output, unwrapped_token, attributes);
                    last_token = Some(token)
                }
                _ => {
                    emit_or_expand_token(output, unwrapped_token, attributes);
                    output.extend([token])
                }
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
        emit_or_expand_token(output, unwrapped_token, attributes);
    }
}

fn filter_x86_markers_group(
    input: &mut Group, attributes: 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬
) -> TokenTree {
    let mut content = TokenStream::new();
    filter_x86_markers_iterable(&mut content, &mut input.stream().into_iter(), attributes);
    Group::new(input.delimiter(), content).into()
}

fn marker_is_compatible<'ᵉˣᵗʳᵃ>(
    marker: &str,
    attributes: 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬<'ᵉˣᵗʳᵃ>,
) -> (Option<bool>, 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬<'ᵉˣᵗʳᵃ>) {
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
                    ₓ𝗂𝗓: None, ..attributes
                },
            ),
            Some(count) => (
                Some(true),
                𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬 {
                    ₓ𝗂𝗓: Some(count - 1),
                    ..attributes
                },
            ),
        },
        "Χ𝔦𝔷" => (Some(attributes.ₓ𝗂𝗓 == None), attributes),
        "Ξ𝔷𝔷" => (Some(attributes.𝖺𝗏𝗑𝟧𝟣𝟤 == Some(true)), attributes),
        "Χ𝔷𝔷" => (Some(attributes.𝖺𝗏𝗑𝟧𝟣𝟤 != Some(true)), attributes),
        _ => (None, attributes),
    }
}

lazy_static! {
    static ref 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬: (String, String) = get_instrution_info();
    static ref 𝔱𝔞𝔯𝔤𝔢𝔱𝔰_𝔪𝔞𝔭_𝔩𝔢𝔤𝔞𝔠𝔶: std::collections::HashMap<&'static str, std::vec::Vec<&'static str>> = hashmap! {
        "reg8" => vec!["𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ"],
        "reg16" => vec!["𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ"],
        "reg32" => vec!["𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ"],
        "reg64" => vec![],
        "reg/acc8" => vec!["𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ"],
        "reg/acc16" => vec!["𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ"],
        "reg/acc32" => vec!["𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ"],
        "reg/acc64" => vec![],
    };
    static ref 𝔱𝔞𝔯𝔤𝔢𝔱𝔰_𝔪𝔞𝔭_𝔵86_64: std::collections::HashMap<&'static str, std::vec::Vec<&'static str>> = hashmap! {
        "reg8" => vec!["𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ"],
        "reg16" => vec!["𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ"],
        "reg32" => vec!["𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ"],
        "reg64" => vec!["𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ"],
        "reg/acc8" => vec!["𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ"],
        "reg/acc16" => vec!["𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ"],
        "reg/acc32" => vec!["𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ"],
        "reg/acc64" => vec!["𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ", "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ"],
    };
}

#[tokio::main]
async fn get_instrution_info() -> (String, String) {
    let root_path = std::env::current_dir().expect("Obtaining crate root path");
    let root_path = root_path.to_str().expect("Turning crate root path into unicode string");
    // Note: during regular build root_path points to the yace workspace root, but in doctests
    // we get nested crate root.  Try to access both paths.
    let database_url = format!("sqlite:{}/test.db", root_path);
    let database_url_fallback = format!("sqlite:{}/../test.db", root_path);
    let mut pool = if let Ok(pool) = sqlx::SqliteConnection::connect(database_url.as_str()).await {
        pool
    } else {
        sqlx::SqliteConnection::connect(database_url_fallback.as_str())
            .await
            .expect("Failed to connect to test.db database")
    };
    let mut rows = sqlx::query("SELECT * FROM instructions")
        .fetch(&mut pool);
        let mut instruction_info_legacy = Vec::new();
        let mut instruction_info_x64 = Vec::new();
        while let Some (row) = rows.try_next().await.expect("Heh") {
        let instruction_name: &str =row.try_get("instruction_name").expect("whatever");
        let instruction_argument0: &str =row.try_get("instruction_argument0").expect("whatever");
        let instruction_argument1: &str =row.try_get("instruction_argument1").expect("whatever");
        if let Some(instruction_argument_cases0) = 𝔱𝔞𝔯𝔤𝔢𝔱𝔰_𝔪𝔞𝔭_𝔩𝔢𝔤𝔞𝔠𝔶.get(instruction_argument0) {
            for instruction_argument_case0 in instruction_argument_cases0 {
                if let Some(instruction_argument_cases1) = 𝔱𝔞𝔯𝔤𝔢𝔱𝔰_𝔪𝔞𝔭_𝔩𝔢𝔤𝔞𝔠𝔶.get(instruction_argument1) {
                    for instruction_argument_case1 in instruction_argument_cases1 {
                        instruction_info_legacy.push(format!("{}_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<(Self::{}, Self::{})>", 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾(instruction_name), instruction_argument_case0, instruction_argument_case1));
                    }
                }
            }
        }
        if let Some(instruction_argument_cases0) = 𝔱𝔞𝔯𝔤𝔢𝔱𝔰_𝔪𝔞𝔭_𝔵86_64.get(instruction_argument0) {
            for instruction_argument_case0 in instruction_argument_cases0 {
                if let Some(instruction_argument_cases1) = 𝔱𝔞𝔯𝔤𝔢𝔱𝔰_𝔪𝔞𝔭_𝔵86_64.get(instruction_argument1) {
                    for instruction_argument_case1 in instruction_argument_cases1 {
                        if (*instruction_argument_case0 != "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ" || *instruction_argument_case1 != "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ") &&
                           (*instruction_argument_case0 != "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ" || *instruction_argument_case1 != "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ") {
                            instruction_info_x64.push(format!("{}_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<(Self::{}, Self::{})>", 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾(instruction_name), instruction_argument_case0, instruction_argument_case1));
                        }
                    }
                }
            }
        }
    }
    (instruction_info_legacy.join(" + "), instruction_info_x64.join(" + "))
}

fn 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾(name: &str) -> String {
    let mut 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 = Vec::new();
    for c in name.as_bytes() {
        if *c >= 128 {
            𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.push(*c)
        } else {
            #[allow(non_upper_case_globals)]
            const 𝔱𝔯𝔞𝔦𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰: [u8; 512] = [
                0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9, 0,
                0, 0, 10, 0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0, 16, 0, 0, 0, 17, 0, 0, 0, 18,
                0, 0, 0, 19, 0, 0, 0, 20, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0,
                27, 0, 0, 0, 28, 0, 0, 0, 29, 0, 0, 0, 30, 0, 0, 0, 31, 0, 0, 0, 32, 0, 0, 0, 33, 0, 0, 0, 34, 0, 0, 0, 35, 0, 0,
                0, 36, 0, 0, 0, 37, 0, 0, 0, 38, 0, 0, 0, 39, 0, 0, 0, 40, 0, 0, 0, 41, 0, 0, 0, 42, 0, 0, 0, 43, 0, 0, 0, 44, 0,
                0, 0, 45, 0, 0, 0, 46, 0, 0, 0, 47, 0, 0, 0, 48, 0, 0, 0, 49, 0, 0, 0, 50, 0, 0, 0, 52, 0, 0, 0, 52, 0, 0, 0, 53,
                0, 0, 0, 54, 0, 0, 0, 55, 0, 0, 0, 56, 0, 0, 0, 57, 0, 0, 0, 58, 0, 0, 0, 59, 0, 0, 0, 60, 0, 0, 0, 61, 0, 0, 0,
                62, 0, 0, 0, 63, 0, 0, 0, 64, 0, 0, 0, 240, 157, 145, 168, 240, 157, 145, 169, 240, 157, 145, 170, 240, 157, 145,
                171, 240, 157, 145, 172, 240, 157, 145, 173, 240, 157, 145, 174, 240, 157, 145, 175, 240, 157, 145, 176, 240, 157,
                145, 177, 240, 157, 145, 178, 240, 157, 145, 179, 240, 157, 145, 180, 240, 157, 145, 181, 240, 157, 145, 182, 240,
                157, 145, 183, 240, 157, 145, 184, 240, 157, 145, 185, 240, 157, 145, 186, 240, 157, 145, 187, 240, 157, 145, 188,
                240, 157, 145, 189, 240, 157, 145, 190, 240, 157, 145, 191, 240, 157, 146, 128, 240, 157, 146, 129, 91, 0, 0, 0,
                92, 0, 0, 0, 93, 0, 0, 0, 94, 0, 0, 0, 95, 0, 0, 0, 96, 0, 0, 0, 240, 157, 146, 130, 240, 157, 146, 131, 240, 157,
                146, 132, 240, 157, 146, 133, 240, 157, 146, 134, 240, 157, 146, 135, 240, 157, 146, 136, 240, 157, 146, 137, 240,
                157, 146, 138, 240, 157, 146, 139, 240, 157, 146, 140, 240, 157, 146, 141, 240, 157, 146, 142, 240, 157, 146, 143,
                240, 157, 146, 144, 240, 157, 146, 145, 240, 157, 146, 146, 240, 157, 146, 147, 240, 157, 146, 148, 240, 157, 146,
                149, 240, 157, 146, 150, 240, 157, 146, 151, 240, 157, 146, 152, 240, 157, 146, 153, 240, 157, 146, 154, 240, 157,
                146, 155, 123, 0, 0, 0, 124, 0, 0, 0, 125, 0, 0, 0, 126, 0, 0, 0, 127, 0, 0, 0,
            ];
            if 𝔱𝔯𝔞𝔦𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰[*c as usize * 4] > 128 {
                𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.extend_from_slice(&𝔱𝔯𝔞𝔦𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰[*c as usize * 4..*c as usize * 4 + 4])
            } else {
                𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.push(𝔱𝔯𝔞𝔦𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰[*c as usize])
            }
        }
    }
    return unsafe { String::from_utf8_unchecked(𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾) };
}
