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

use {
    futures::{StreamExt, TryStreamExt},
    indoc::indoc,
    maplit::hashmap,
    once_cell::sync::Lazy,
    proc_macro::{Delimiter, Group, Ident, TokenStream, TokenTree},
    std::collections::HashMap,
};

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
    let Some(TokenTree::Ident(macro_name)) = iter.next() else {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't find the name of nested macro.\");"
            .parse()
            .unwrap();
    };

    let Some(TokenTree::Punct(exclamation_mark)) = iter.next() else {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't find ! after the name of nested macro.\");"
            .parse()
            .unwrap();
    };
    if exclamation_mark.as_char() != '!' {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't find ! after the name of nested macro.\");"
            .parse()
            .unwrap();
    }

    let Some(TokenTree::Group(main_group)) = iter.next() else {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't find main group to process.\");"
            .parse()
            .unwrap();
    };
    let Delimiter::Brace = main_group.delimiter() else {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — main group should use braces.\");"
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
    let Some(TokenTree::Group(attributes_group)) = attributes_iter.next() else {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't find attributes group to process.\");"
            .parse()
            .unwrap();
    };
    let Delimiter::Bracket = attributes_group.delimiter() else {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — attributes_group group should use brackets.\");"
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

    let mut 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌 = TokenStream::new();
    filter_x86_markers_iterable(&mut 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌, &mut main_group_iter, attributes);
    let mut result = TokenStream::new();
    result.extend([
        TokenTree::Ident(macro_name),
        TokenTree::Punct(exclamation_mark),
        Into::<TokenTree>::into(Group::new(Delimiter::Brace, 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌)),
    ]);
    result
}

#[proc_macro]
pub fn 𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓𝖘(
    items: TokenStream,
) -> TokenStream {
    let mut iter = items.into_iter();
    if iter.next().is_some() {
        return "compile_error!(\"𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓𝖘! — arguments are not supported.\");"
            .parse()
            .unwrap();
    }
    𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇.parse().unwrap()
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
        let mut process = |identifier: &Ident| -> Result<(), &'static str> {
            match identifier.to_string().as_str() {
                "𝔞𝔡𝔡𝔯16" => {
                    if result.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾.is_some() {
                        return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated address size.\");");
                    } else {
                        result.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 = core::num::NonZeroI8::new(16)
                    }
                }
                "𝔞𝔡𝔡𝔯32" => {
                    if result.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾.is_some() {
                        return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated address size.\");");
                    } else {
                        result.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 = core::num::NonZeroI8::new(32)
                    }
                }
                "𝔞𝔡𝔡𝔯64" => {
                    if result.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾.is_some() {
                        return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated address size.\");");
                    } else {
                        result.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 = core::num::NonZeroI8::new(64)
                    }
                }
                "𝔡𝔞𝔱𝔞16" => {
                    if result.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾.is_some() {
                        return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated data size.\");");
                    } else {
                        result.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾 = core::num::NonZeroI8::new(16)
                    }
                }
                "𝔡𝔞𝔱𝔞32" => {
                    if result.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾.is_some() {
                        return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated data size.\");");
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
                        return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated ₐᵥₓ512 marker.\");");
                    } else {
                        result.𝖺𝗏𝗑𝟧𝟣𝟤 = Some(true)
                    }
                }
                _ => return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — unknown token.\");"),
            }
            Ok(())
        };
        for token_tree in input {
            match &token_tree {
                TokenTree::Ident(identifier) => {
                    process(identifier)?;
                }
                TokenTree::Group(group) if matches!(group.delimiter(), Delimiter::None) => {
                    for token_tree in group.stream().into_iter() {
                        let TokenTree::Ident(identifier) = &token_tree else {
                            return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — attributes group includes unknown item.\");")
			};
                        process(identifier)?;
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
                let Some(TokenTree::Ident(_)) = 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 else {
                    return Ok(Default::default());
                };
                if !trait_iter.next().is_none() {
                    return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't parse optional data.\");");
                }
                𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾
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
                let Some(TokenTree::Ident(_)) = 𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾 else {
                    return Ok(Default::default());
                };
                if !struct_iter.next().is_none() {
                    return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't parse optional data.\");");
                }
                𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾
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
        let TokenTree::Ident(ref ident) = token else {
            return output.extend([token])
        };

        match ident.to_string().as_str() {
            "𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟𝕤𝕀𝕟𝕥𝕖𝕣𝕗𝕒𝕔𝕖" => {
                let additional_info: TokenStream = if attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 != core::num::NonZeroI8::new(64)
                {
                    𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶 as usize]
                        .𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌
                        .parse()
                        .unwrap()
                } else {
                    𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64 as usize]
                        .𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌
                        .parse()
                        .unwrap()
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
                let token_stream: TokenStream = if attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 == core::num::NonZeroI8::new(64) {
                    format!(
                        concat!(
                            "impl Æ 𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>{1}_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮> for æ ",
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
                            "impl Æ 𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>{1}_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮> for æ ",
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

#[tokio::main]
async fn get_instrution_info() -> 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
    let mut connection = get_database_connection().await;
    let mut assembler_instructions = Vec::new();
    let mut 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌 = [Vec::new(), Vec::new()];
    for assembler_kind in [𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶, 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64]
    {
        let mut instructions_stream = get_insructions_info(&mut connection, assembler_kind);
        while let Some(instruction) = instructions_stream.try_next().await.expect("Connection aborted") {
            let 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌 = &instruction.𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌;
            if (𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌[0].𝗌𝗊𝗅_𝗍𝗒𝗉𝖾 == "rex_register_8bit" && 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌[1].𝗌𝗊𝗅_𝗍𝗒𝗉𝖾 == "norex_register_8bit")
                || (𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌[0].𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.starts_with("address") && 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌[1].𝗌𝗊𝗅_𝗍𝗒𝗉𝖾 == "norex_register_8bit")
                || (𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌[0].𝗌𝗊𝗅_𝗍𝗒𝗉𝖾 == "norex_register_8bit" && 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌[1].𝗌𝗊𝗅_𝗍𝗒𝗉𝖾 == "rex_register_8bit")
                || (𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌[0].𝗌𝗊𝗅_𝗍𝗒𝗉𝖾 == "norex_register_8bit" && 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌[1].𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.starts_with("address"))
            {
                continue;
            }

            let 𝖿𝗇_𝗇𝖺𝗆𝖾 = instruction.𝖿𝗇_𝗇𝖺𝗆𝖾.as_str();
            let 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 = instruction.𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.as_str();

            let process_xiz_version = 'ᵃⁿˢʷᵉʳ: {
                for argument in 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌 {
                    if argument.𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾_ₓ𝔦𝔷.is_some() {
                        break 'ᵃⁿˢʷᵉʳ &[false, true][..];
                    }
                }
                break 'ᵃⁿˢʷᵉʳ &[false][..];
            };

            let mut arguments_type = Vec::new();
            let mut arguments_type_xiz = Vec::new();
            let mut arguments_trait_type = Vec::new();
            let mut parameters_list = Vec::new();
            let mut memory_size = None;
            let mut non_memory_size = None;
            for (index, argument) in 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌.into_iter().enumerate() {
                let 𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾 = argument.𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾;
                arguments_type.push(𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾);
                if let Some(𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾_ₓ𝔦𝔷) = argument.𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾_ₓ𝔦𝔷
                {
                    arguments_type_xiz.push(𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾_ₓ𝔦𝔷);
                } else {
                    arguments_type_xiz.push(𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾);
                }

                let 𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍_𝗍𝗒𝗉𝖾 = argument.𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍_𝗍𝗒𝗉𝖾;
                arguments_trait_type.push(𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍_𝗍𝗒𝗉𝖾);

                let 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾 = argument.𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.as_str();
                let operand_size = if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_8bit") {
                    Some(1)
                } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_16bit") {
                    Some(2)
                } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_32bit") {
                    Some(4)
                } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_64bit") {
                    Some(8)
                } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_128bit") {
                    Some(16)
                } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_256bit") {
                    Some(32)
                } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_512bit") {
                    Some(64)
                } else {
                    None
                };

                let operand_size_target = if 𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾.ends_with(">") {
                    &mut memory_size
                } else {
                    &mut non_memory_size
                };

                if operand_size_target.is_none() || *operand_size_target == operand_size {
                    *operand_size_target = operand_size;
                } else {
                    *operand_size_target = Some(0);
                }

                let 𝗌𝗊𝗅_𝗈𝗉𝖾𝗋𝖺𝗇𝖽 = argument.𝗌𝗊𝗅_𝗈𝗉𝖾𝗋𝖺𝗇𝖽.as_str();
                if 𝗌𝗊𝗅_𝗈𝗉𝖾𝗋𝖺𝗇𝖽 == "implicit" {
                    parameters_list.push(format!("_parameter{index}"));
                } else {
                    parameters_list.push(format!("parameter{index}"));
                }
            }
            let arguments_type = format!("({})", arguments_type.join(","));
            let arguments_type_xiz = format!("({})", arguments_type_xiz.join(","));
            let arguments_trait_type = format!("({})", arguments_trait_type.join(","));
            let process_unsized_memory = if memory_size.is_some() && memory_size == non_memory_size {
                &[false, true][..]
            } else {
                &[false][..]
            };

            let parameters_list = parameters_list.join(",");
            let mut need_extra_trait = false;
            let 𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑 = instruction.𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑;
            let 𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑 = match (assembler_kind, 𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑.as_str())
            {
                (_, "") => "𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞",
                (𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶, "address_size_prefix_16bit") => {
                    need_extra_trait = true;
                    "<Self as 𝒍𝒆𝒈𝒂𝒄𝒚_𝒎𝒐𝒅𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓>::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐩𝐫𝐞𝐟𝐢𝐱_16ᵇⁱᵗ"
                }
                (𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶, "address_size_prefix_32bit") => {
                    need_extra_trait = true;
                    "<Self as 𝒍𝒆𝒈𝒂𝒄𝒚_𝒎𝒐𝒅𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓>::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐩𝐫𝐞𝐟𝐢𝐱_32ᵇⁱᵗ"
                }
                (𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64, "address_size_prefix_32bit") => {
                    "𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x67>"
                }
                (𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64, "address_size_prefix_64bit") => {
                    "𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞"
                }
                _ => panic!("Usupported config of address prefixes: {assembler_kind:?} {𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑:?}"),
            };
            let 𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑 = instruction.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑;
            let (𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑, rexw_prefix) = match (assembler_kind, 𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑.as_str())
            {
                (_, "") => ("𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴),
                (𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶, "data_size_prefix_16bit") => {
                    need_extra_trait = true;
                    ("<Self as 𝒍𝒆𝒈𝒂𝒄𝒚_𝒎𝒐𝒅𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓>::𝐝𝐚𝐭𝐚_𝐩𝐫𝐞𝐟𝐢𝐱_16ᵇⁱᵗ", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴)
                }
                (𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶, "data_size_prefix_32bit") => {
                    need_extra_trait = true;
                    ("<Self as 𝒍𝒆𝒈𝒂𝒄𝒚_𝒎𝒐𝒅𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓>::𝐝𝐚𝐭𝐚_𝐩𝐫𝐞𝐟𝐢𝐱_32ᵇⁱᵗ", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴)
                }
                (𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64, "data_size_prefix_16bit") => {
                    ("𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x66>", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴)
                }
                (𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64, "data_size_prefix_32bit") => {
                    ("𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴)
                }
                (𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64, "data_size_prefix_64bit") => {
                    ("𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔯𝔢𝔵𝔴)
                }
                _ => panic!("Usupported config of data prefixes: {assembler_kind:?} {𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑:?}"),
            };
            let instruction_type = format!(
                "𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧<{},{},{},{},{},{},{},𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x{:02x}>,{}>",
                "𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞", // 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓹𝓻𝓮𝓯𝓲𝔁
                𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑,
                𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑,
                "𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞", // 𝓵𝓸𝓬𝓴_𝓹𝓻𝓮𝓯𝓲𝔁
                "𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞", // 𝓻𝓮𝓹ₓ_𝓹𝓻𝓮𝓯𝓲𝔁
                "𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞", // 𝔁𝟬𝗙_𝓹𝓻𝓮𝓯𝓲𝔁
                "𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞", // 𝔁𝟯𝘅_𝓹𝓻𝓮𝓯𝓲𝔁
                instruction.𝗈𝗉𝖼𝗈𝖽𝖾,
                "𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞" // 𝓲𝓶𝓶𝓮𝓭𝓲𝓪𝓽𝓮_𝓸𝓹𝓬𝓸𝓭𝓮
            );

            let argument0_sql_operand = 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌[0].𝗌𝗊𝗅_𝗈𝗉𝖾𝗋𝖺𝗇𝖽.as_str();
            let argument0_sql_type = 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌[0].𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.as_str();
            let argument1_sql_operand = 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌[1].𝗌𝗊𝗅_𝗈𝗉𝖾𝗋𝖺𝗇𝖽.as_str();
            let argument1_sql_type = 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌[1].𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.as_str();
            let (instruction_emit, instruction_trait_for_emit) = match (argument0_sql_operand, argument1_sql_operand) {
                ("implicit", "immediate") => match (argument1_sql_type, rexw_prefix) {
                    ("imm8", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴) => (
                        format!("<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_and_opcodes(self,[parameter1 as u8])"),
                        format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>")
                    ),
                    ("imm16", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴) => (
                        format!("<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},2>>::emit_prefixes_and_opcodes(self,[parameter1 as u8,(parameter1>>8)as u8])"),
                        format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},2>")
                    ),
                    ("imm32", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴) => (
                        format!("<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>>::emit_prefixes_and_opcodes(self,[parameter1 as u8,(parameter1>>8)as u8,(parameter1>>16)as u8,(parameter1>>24)as u8])"),
                        format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>")
                    ),
                    ("imm32", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔯𝔢𝔵𝔴) => (
                        format!("<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>>::emit_prefixes_rex_and_opcodes(self,0b0100_1000,[parameter1 as u8,(parameter1>>8)as u8,(parameter1>>16)as u8,(parameter1>>24)as u8])"),
                        format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>")
                    ),
                    _ => panic!("Unsupported combination of instruction arguments and prefixes"),
                },
                ("reg", "rm") => {
                    let (instruction_trait, instruction_fn) = match rexw_prefix {
                        𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴 => {
                            if argument1_sql_type.starts_with("address_16bit") {
                                ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_8086_memory_instruction")
                            } else if argument1_sql_type.starts_with("address_32bit")
                                && assembler_kind != 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64
                                || argument1_sql_type.starts_with("norex_address_32bit") {
                                ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_80386_memory_instruction")
                            } else if argument1_sql_type.starts_with("address")
                                || argument1_sql_type.starts_with("norex_address") {
                                ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_ₓ86_64_memory_instruction")
                            } else {
                                ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction")
                            }
                        }
                        𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔯𝔢𝔵𝔴 => {
                            if argument1_sql_type.starts_with("address") || argument1_sql_type.starts_with("norex_address") {
                                (   "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                    "emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw")
                            } else {
                                ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction_with_rexw")
                            }
                        }
                    };
                    (   format!("<Self as {instruction_trait}<{instruction_type}>>::{instruction_fn}(self,parameter0,parameter1)"),
                        format!("{instruction_trait}<{instruction_type}>")
                    )
                }
                ("rm", "reg") => {
                    let (instruction_trait, instruction_fn) = match rexw_prefix {
                        𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴 => {
                            if argument0_sql_type.starts_with("address_16bit") {
                                ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_8086_memory_instruction")
                            } else if argument0_sql_type.starts_with("address_32bit")
                                && assembler_kind != 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64
                                || argument0_sql_type.starts_with("norex_address_32bit") {
                                ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_80386_memory_instruction")
                            } else if argument0_sql_type.starts_with("address")
                                || argument0_sql_type.starts_with("norex_address") {
                                ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_ₓ86_64_memory_instruction")
                            } else {
                                ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction")
                            }
                        }
                        𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔯𝔢𝔵𝔴 => {
                            if argument0_sql_type.starts_with("address") || argument0_sql_type.starts_with("norex_address") {
                                (   "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                    "emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw")
                            } else {
                                ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction_with_rexw")
                            }
                        }
                    };
                    (   format!("<Self as {instruction_trait}<{instruction_type}>>::{instruction_fn}(self,parameter1,parameter0)"),
                        format!("{instruction_trait}<{instruction_type}>"))
                }
                ("rm", "immediate") => {
                    let 𝗈𝗉𝖼𝗈𝖽𝖾_𝖾𝗑𝗍𝖾𝗇𝗌𝗂𝗈𝗇 = instruction
                        .𝗈𝗉𝖼𝗈𝖽𝖾_𝖾𝗑𝗍𝖾𝗇𝗌𝗂𝗈𝗇
                        .expect("Legacy instruction can not have rm operand without either reg operand or opcode extension");
                    let (instruction_trait, instruction_fn) = match rexw_prefix {
                        𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴 => {
                            if argument0_sql_type.starts_with("address_16bit")
                                || argument0_sql_type.starts_with("norex_address_16bit") {
                                (   "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                    "emit_legacy_reg_address_8086_memory_instruction_with_i")
                            } else if argument0_sql_type.starts_with("address_32bit")
                               && assembler_kind != 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64
                                || argument0_sql_type.starts_with("norex_address_32bit") {
                                (   "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                    "emit_legacy_reg_address_80386_memory_instruction_with_i")
                            } else if argument0_sql_type.starts_with("address")
                                || argument0_sql_type.starts_with("norex_address") {
                                (   "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                    "emit_legacy_reg_address_ₓ86_64_memory_instruction_with_i")
                            } else {
                                ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction_with_i")
                            }
                        }
                        𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔯𝔢𝔵𝔴 => {
                            if argument0_sql_type.starts_with("address") || argument0_sql_type.starts_with("norex_address") {
                                (    "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                     "emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_i")
                            } else {
                                ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction_with_rexw_and_i")
                            }
                        }
                    };
                    let immediate_size = &argument1_sql_type[3..];
                    (   format!("<Self as {instruction_trait}<{instruction_type}>>::{instruction_fn}{immediate_size}(self,{𝗈𝗉𝖼𝗈𝖽𝖾_𝖾𝗑𝗍𝖾𝗇𝗌𝗂𝗈𝗇},parameter0,parameter1)"),
                        format!("{instruction_trait}<{instruction_type}>"))
                }
                _ => panic!("Unsupported combination of instruction arguments"),
            };
            let extra_trait = if need_extra_trait {
                format!("+𝒍𝒆𝒈𝒂𝒄𝒚_𝒎𝒐𝒅𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓+{instruction_trait_for_emit}")
            } else {
                "".to_owned()
            };
            for &xiz_version in process_xiz_version {
                let arguments_type = if xiz_version { &arguments_type_xiz } else { &arguments_type };

                for &unsized_memory in process_unsized_memory {
                    let arguments_type_buffer;
                    let arguments_type = if unsized_memory {
                        let memory_size = memory_size.unwrap();
                        arguments_type_buffer = arguments_type.replace(format!(",{memory_size}>").as_str(), ",0>");
                        &arguments_type_buffer
                    } else {
                        arguments_type
                    };

                    if let 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶 = assembler_kind {
                        assembler_instructions.push(format!("impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮:𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓{extra_trait}>{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒍𝒆𝒈𝒂𝒄𝒚_𝒎𝒐𝒅𝒆_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<{arguments_type}>for 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮{{type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞=<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞=<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;fn {𝖿𝗇_𝗇𝖺𝗆𝖾}_legacy_mode(&mut self,({parameters_list}):{arguments_type})->Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>{{{instruction_emit}}}}}"));
                    } else {
                        assembler_instructions.push(format!("impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮:𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_ₓ86_64_𝒎𝒐𝒅𝒆_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<{arguments_type}>for 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮{{type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞=<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞=<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;fn {𝖿𝗇_𝗇𝖺𝗆𝖾}_x86_64_mode(&mut self,({parameters_list}):{arguments_type})->Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>{{{instruction_emit}}}}}"));
                    }
                }
            }
            for &unsized_memory in process_unsized_memory {
                let arguments_trait_type_buffer;
                let arguments_trait_type = if unsized_memory {
                    let memory_size = memory_size.unwrap();
                    arguments_trait_type_buffer = arguments_trait_type.replace(format!(",{memory_size}>").as_str(), ",0>");
                    &arguments_trait_type_buffer
                } else {
                    &arguments_trait_type
                };
                𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌[assembler_kind as usize].push(format!("{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<{arguments_trait_type}>"));
            }
        }
    }
    return 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
        𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇: assembler_instructions.concat(),
        𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈: [
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
                𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌: 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶 as usize].join(" + "),
                𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌: "".to_owned(),
            },
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
                𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌: 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64 as usize].join(" + "),
                𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌: "".to_owned(),
            },
        ],
    };

    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    #[repr(i8)]
    enum 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞 {
        𝔫𝔬𝔯𝔢𝔵𝔴 = 0,
        𝔯𝔢𝔵𝔴 = 1,
    }
}

async fn get_database_connection() -> sqlx::SqliteConnection {
    use sqlx::Connection;
    let root_path = std::env::current_dir().expect("Obtaining crate root path");
    let root_path = root_path.to_str().expect("Turning crate root path into unicode string");
    // Note: during regular build root_path points to the yace workspace root, but in doctests
    // we get nested crate root.  Try to access both paths.
    let database_url = format!("sqlite:{}/instructions.db", root_path);
    let database_url_fallback = format!("sqlite:{}/../instructions.db", root_path);
    let Ok(connection) = sqlx::SqliteConnection::connect(database_url.as_str()).await else {
        return sqlx::SqliteConnection::connect(database_url_fallback.as_str())
            .await
            .expect("Failed to connect to instructions.db database")
    };
    connection
}

fn get_insructions_info<'ᵉˣᵉᶜᵘᵗᵒʳ, 𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: sqlx::Database>(
    connection: impl sqlx::Executor<'ᵉˣᵉᶜᵘᵗᵒʳ, Database = 𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    assembler_kind: 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞,
) -> impl futures::stream::Stream<Item = Result<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞, sqlx::Error>>
       + 'ᵉˣᵉᶜᵘᵗᵒʳ
where
    &'ᵉˣᵉᶜᵘᵗᵒʳ str: sqlx::Type<𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>
        + sqlx::Encode<'ᵉˣᵉᶜᵘᵗᵒʳ, 𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>
        + sqlx::ColumnIndex<<𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as sqlx::Database>::Row>,
    for<'ˢᵗʳⁱⁿᵍ> String: sqlx::Type<𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>
        + sqlx::Encode<'ˢᵗʳⁱⁿᵍ, 𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>
        + sqlx::Decode<'ˢᵗʳⁱⁿᵍ, 𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    for<'ᵇʸᵗᵉ> u8: sqlx::Type<𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>
        + sqlx::Encode<'ᵇʸᵗᵉ, 𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>
        + sqlx::Decode<'ᵇʸᵗᵉ, 𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    for<'ᵉˣᵗʳᵃ> Option<u8>: sqlx::Type<𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>
        + sqlx::Encode<'ᵉˣᵗʳᵃ, 𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>
        + sqlx::Decode<'ᵉˣᵗʳᵃ, 𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    <𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as sqlx::database::HasArguments<'ᵉˣᵉᶜᵘᵗᵒʳ>>::Arguments: sqlx::IntoArguments<'ᵉˣᵉᶜᵘᵗᵒʳ, 𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
{
    let rust_types_map = assembler_kind.as_rust_types_map();
    let rust_types_map_xiz = assembler_kind.as_rust_types_map_xiz();
    sqlx::query(indoc! {"
            SELECT name0 AS name,
                   operand0.parameter_type AS type0,
                   trait0.name AS trait0,
                   operand0.operand_source AS operand0,
                   operand1.parameter_type AS type1,
                   trait1.name AS trait1,
                   operand1.operand_source AS operand1,
                   MAX(IFNULL(instruction.data_size_prefix, ''),
                       IFNULL(operand0.data_size_prefix, ''),
                       IFNULL(operand1.data_size_prefix, '')) AS data_size_prefix,
                   MAX(IFNULL(instruction.address_size_prefix, ''),
                       IFNULL(operand0.address_size_prefix, ''),
                       IFNULL(operand1.address_size_prefix, '')) AS address_size_prefix,
                   opcode,
                   opcode_extension
            FROM (
                SELECT *
                FROM (
                    SELECT instruction.name AS name0, traits_information.name AS trait0, priority
                    FROM instruction LEFT JOIN
                         operands ON operands = short_name LEFT JOIN
                         operand ON operand0 = operand.name LEFT JOIN
                         traits_information ON parameter_type = allowed_operand
                         LEFT JOIN traits_priority ON traits_information.name = traits_priority.name
                    WHERE instruction.assembler_kind IS NULL OR
                          instruction.assembler_kind == $1
                    GROUP BY instruction.name, operands
                    HAVING priority = MIN(priority)
                    ORDER BY instruction.name, operands, priority
                )
                GROUP BY name0
                HAVING priority = MAX(priority)
                ORDER BY name0
            ) LEFT JOIN (
                SELECT *
                FROM (
                    SELECT instruction.name AS name1, traits_information.name AS trait1, priority
                    FROM instruction LEFT JOIN
                         operands ON operands = short_name LEFT JOIN
                         operand ON operand1 = operand.name LEFT JOIN
                         traits_information ON parameter_type = allowed_operand LEFT JOIN
                         traits_priority ON traits_information.name = traits_priority.name
                    WHERE instruction.assembler_kind IS NULL OR
                          instruction.assembler_kind == $1
                    GROUP BY instruction.name, operands
                    HAVING priority = MIN(priority)
                    ORDER BY instruction.name, operands, priority
                )
                GROUP BY name1
                HAVING priority = MAX(priority)
                ORDER BY name1
            ) ON name0 = name1 LEFT JOIN
            instruction ON name0 = instruction.name LEFT JOIN
            operands ON operands = short_name LEFT JOIN
            operand AS operand0 ON operand0 = operand0.name LEFT JOIN
            operand AS operand1 ON operand1 = operand1.name,
            traits_information AS trait0 ON trait0 = trait0.name AND operand0.parameter_type = trait0.allowed_operand,
            traits_information AS trait1 ON trait1 = trait1.name AND operand1.parameter_type = trait1.allowed_operand
            WHERE (instruction.assembler_kind IS NULL OR
                   instruction.assembler_kind == $1) AND
                  (operand0.data_size_prefix = operand1.data_size_prefix OR
                   operand0.data_size_prefix IS NULL OR
                   operand1.data_size_prefix IS NULL) AND
                  (operand0.assembler_kind IS NULL OR operand0.assembler_kind = $1) AND
                  (operand1.assembler_kind IS NULL OR operand1.assembler_kind = $1)
            GROUP BY name0, type0, type1
            HAVING operands = MIN(operands)
            ORDER BY name0, type0, type1;"})
    .bind(assembler_kind.as_str())
    .fetch(connection)
    .map(|row| {
        use sqlx::Row;

        let row = row?;
        let instruction_name: String = row.try_get("name")?;
        let instruction_argument0: String = row.try_get("type0")?;
        let instruction_argument1: String = row.try_get("type1")?;
        let instruction_argument0_trait: String = row.try_get("trait0")?;
        let instruction_argument1_trait: String = row.try_get("trait1")?;
        let instruction_operand0: String = row.try_get("operand0")?;
        let instruction_operand1: String = row.try_get("operand1")?;

        let instruction_trait_name = 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾(instruction_name.as_str());
        let instruction_argument0_type = rust_types_map
            .get(instruction_argument0.as_str())
            .expect("Failed to convert sql type to rust type");
        let instruction_argument0_type_xiz = rust_types_map_xiz.get(instruction_argument0.as_str()).copied();
        let instruction_argument1_type = rust_types_map
            .get(instruction_argument1.as_str())
            .expect("Failed to convert sql type to rust type");
        let instruction_argument1_type_xiz = rust_types_map_xiz.get(instruction_argument1.as_str()).copied();
        let instruction_argument0_trait = 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱
            .get(instruction_argument0_trait.as_str())
            .expect("Failed to convert sql type to rust type");
        let instruction_argument0_trait_type = 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱
            .get(instruction_argument0.as_str())
            .expect("Failed to convert sql type to rust type");
        let instruction_argument1_trait = 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱
            .get(instruction_argument1_trait.as_str())
            .expect("Failed to convert sql type to rust type");
        let instruction_argument1_trait_type = 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱
            .get(instruction_argument1.as_str())
            .expect("Failed to convert sql type to rust type");

        Ok(𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
            𝖿𝗇_𝗇𝖺𝗆𝖾: instruction_name,
            𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾: instruction_trait_name,
            𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑: row.try_get("data_size_prefix")?,
            𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑: row.try_get("address_size_prefix")?,
            𝗈𝗉𝖼𝗈𝖽𝖾: row.try_get("opcode")?,
            𝗈𝗉𝖼𝗈𝖽𝖾_𝖾𝗑𝗍𝖾𝗇𝗌𝗂𝗈𝗇: row.try_get("opcode_extension")?,
            𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌: vec![
                𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭_𝐭𝐲𝐩𝐞 {
                    𝗌𝗊𝗅_𝗍𝗒𝗉𝖾: instruction_argument0,
                    𝗌𝗊𝗅_𝗈𝗉𝖾𝗋𝖺𝗇𝖽: instruction_operand0,
                    𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾: instruction_argument0_type,
                    𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾_ₓ𝔦𝔷: instruction_argument0_type_xiz,
                    𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍: instruction_argument0_trait,
                    𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍_𝗍𝗒𝗉𝖾: instruction_argument0_trait_type,
                },
                𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭_𝐭𝐲𝐩𝐞 {
                    𝗌𝗊𝗅_𝗍𝗒𝗉𝖾: instruction_argument1,
                    𝗌𝗊𝗅_𝗈𝗉𝖾𝗋𝖺𝗇𝖽: instruction_operand1,
                    𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾: instruction_argument1_type,
                    𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾_ₓ𝔦𝔷: instruction_argument1_type_xiz,
                    𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍: instruction_argument1_trait,
                    𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍_𝗍𝗒𝗉𝖾: instruction_argument1_trait_type,
                },
            ],
        })
    })
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(i8)]
enum 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞 {
    𝔩𝔢𝔤𝔞𝔠𝔶 = 0,
    𝔵86_64 = 1,
}

impl 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞 {
    fn as_str(self) -> &'static str {
        ["legacy", "x86_64"][self as usize]
    }
    fn as_rust_types_map(self) -> &'static HashMap<&'static str, &'static str> {
        [&*𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_𝔩𝔢𝔤𝔞𝔠𝔶, &*𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_ₓ86_64][self as usize]
    }
    fn as_rust_types_map_xiz(self) -> &'static HashMap<&'static str, &'static str> {
        [&*𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_𝔩𝔢𝔤𝔞𝔠𝔶_𝔴𝔦𝔱𝔥_𝔢𝔦𝔷, &*𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_ₓ86_64_𝔴𝔦𝔱𝔥_𝔯𝔦𝔷][self as usize]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
    𝖿𝗇_𝗇𝖺𝗆𝖾: String,
    𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾: String,
    𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑: String,
    𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑: String,
    𝗈𝗉𝖼𝗈𝖽𝖾: u8,
    𝗈𝗉𝖼𝗈𝖽𝖾_𝖾𝗑𝗍𝖾𝗇𝗌𝗂𝗈𝗇: Option<u8>,
    𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌: Vec<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭_𝐭𝐲𝐩𝐞>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭_𝐭𝐲𝐩𝐞 {
    𝗌𝗊𝗅_𝗍𝗒𝗉𝖾: String,
    𝗌𝗊𝗅_𝗈𝗉𝖾𝗋𝖺𝗇𝖽: String,
    𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾: &'static str,
    𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾_ₓ𝔦𝔷: Option<&'static str>,
    𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍: &'static str,
    𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍_𝗍𝗒𝗉𝖾: &'static str,
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

static 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬: Lazy<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞> = Lazy::new(get_instrution_info);
static 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    hashmap! {
        "accumulator_register_16bit" => "Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ",
        "accumulator_register_32bit" => "Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ",
        "accumulator_register_64bit" => "Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ",
        "accumulator_register_8bit" => "Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ",
        "address_16bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "i16,",
                                                              "2>"),
        "address_16bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "i16,",
                                                              "4>"),
        "address_16bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                             "i16,",
                                                             "1>"),
        "address_32bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                             "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "2>"),
        "address_32bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                             "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "4>"),
        "address_32bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                             "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "8>"),
        "address_32bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                            "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                            "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                            "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "1>"),
        "address_64bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "2>"),
        "address_64bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "4>"),
        "address_64bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "8>"),
        "address_64bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                            "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "1>"),
        "assembler_operand_of_8bit_instruction" => "𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
        "assembler_operand_separate_accumulator" => "𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓",
        "generic_assembler_operand" => "𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅",
        "gp_register_16bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ",
        "gp_register_32bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ",
        "gp_register_64bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ",
        "gp_register_8bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ",
        "imm16" => "i16",
        "imm32" => "i32",
        "imm64" => "i64",
        "imm8" => "i8",
        "low_register_8bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ",
        "norex_address_32bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "2>"),
        "norex_address_32bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "4>"),
        "norex_address_32bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "8>"),
        "norex_address_32bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                  "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "1>"),
        "norex_address_64bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "2>"),
        "norex_address_64bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "4>"),
        "norex_address_64bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "8>"),
        "norex_address_64bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                  "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "1>"),
        "norex_register_16bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ",
        "norex_register_32bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ",
        "norex_register_64bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ",
        "norex_register_8bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ",
        "rex_register_8bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ",
    }
});
static 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_𝔩𝔢𝔤𝔞𝔠𝔶: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    hashmap! {
        "accumulator_register_16bit" => "𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ",
        "accumulator_register_32bit" => "𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ",
        "accumulator_register_8bit" => "𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ",
        "address_16bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "i16,",
                                                              "2>"),
        "address_16bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "i16,",
                                                              "4>"),
        "address_16bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                             "i16,",
                                                             "1>"),
        "address_32bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "2>"),
        "address_32bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "4>"),
        "address_32bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "1>"),
        "gp_register_16bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086",
        "gp_register_32bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386",
        "gp_register_8bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086",
        "imm16" => "i16",
        "imm32" => "i32",
        "imm64" => "i64",
        "imm8" => "i8",
    }
});
static 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_𝔩𝔢𝔤𝔞𝔠𝔶_𝔴𝔦𝔱𝔥_𝔢𝔦𝔷: Lazy<
    HashMap<&'static str, &'static str>,
> = Lazy::new(|| {
    hashmap! {
        "address_32bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "2>"),
        "address_32bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "4>"),
        "address_32bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "1>"),
    }
});
static 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_ₓ86_64: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    hashmap! {
        "accumulator_register_16bit" => "𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ",
        "accumulator_register_32bit" => "𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ",
        "accumulator_register_64bit" => "𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ",
        "accumulator_register_8bit" => "𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ",
        "address_32bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "2>"),
        "address_32bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "4>"),
        "address_32bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "8>"),
        "address_32bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "1>"),
        "address_64bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "2>"),
        "address_64bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "4>"),
        "address_64bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "8>"),
        "address_64bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "1>"),
        "gp_register_16bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64",
        "gp_register_32bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64",
        "gp_register_64bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ",
        "imm16" => "i16",
        "imm32" => "i32",
        "imm64" => "i64",
        "imm8" => "i8",
        "low_register_8bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ",
        "norex_address_32bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "2>"),
        "norex_address_32bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "4>"),
        "norex_address_32bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "8>"),
        "norex_address_32bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                  "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                  "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                  "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "1>"),
        "norex_address_64bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "2>"),
        "norex_address_64bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "4>"),
        "norex_address_64bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "8>"),
        "norex_address_64bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                  "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "1>"),
        "norex_register_16bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086",
        "norex_register_32bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386",
        "norex_register_64bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ",
        "norex_register_8bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086",
        "rex_register_8bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64",
    }
});
static 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_ₓ86_64_𝔴𝔦𝔱𝔥_𝔯𝔦𝔷: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    hashmap! {
        "address_32bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "2>"),
        "address_32bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "4>"),
        "address_32bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "8>"),
        "address_32bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "1>"),
        "address_64bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "2>"),
        "address_64bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "4>"),
        "address_64bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "8>"),
        "address_64bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "1>"),
        "norex_address_32bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "2>"),
        "norex_address_32bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "4>"),
        "norex_address_32bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "8>"),
        "norex_address_32bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                  "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                  "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                  "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "1>"),
        "norex_address_64bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "2>"),
        "norex_address_64bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "4>"),
        "norex_address_64bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "8>"),
        "norex_address_64bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                  "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "1>"),
    }
});

struct 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
    𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇: String,
    // These are indexed by 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞 as usize.
    𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈: [𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞; 2],
}

struct 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
    𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌: String,
    𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌: String,
}
