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

use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

// Note: the use of that macro is a bit unusial. It works like this:
//     𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! {
//         𝓼𝓸𝓶𝓮_𝓸𝓽𝓱𝓮𝓻_𝓶𝓪𝓬𝓻𝓸! {
//             [𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 𝓭𝓪𝓽𝓪_𝓼𝓲𝔃𝓮 𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼…]
//             … 𝓭𝓪𝓽𝓪 𝓽𝓸 𝓯𝓲𝓵𝓽𝓮𝓻 …
//         }
//    }
// 𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! first moves filtered data to unfiltered one, then removes square brackets and expands 𝓼𝓸𝓶𝓮_𝓸𝓽𝓱𝓮𝓻_𝓶𝓪𝓬𝓻𝓸!
// We couldn't use “more obvious” alternative where 𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! would be called from the 𝓼𝓸𝓶𝓮_𝓸𝓽𝓱𝓮𝓻_𝓶𝓪𝓬𝓻𝓸! because in Rust
// macro can only be called in certain, limited, positions and sometimes (e.g. in 𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖙𝖗𝖆𝖎𝖙) we need to filter data in
// some positions where this call is not allowed.
//
// Supported markers:
//     Ξ𝔯𝔳32𝔢 — 𝗿𝘃𝟯𝟮𝗲 data
//     Χ𝔯𝔳32𝔢 — 𝗿𝘃𝟯𝟮𝗶/𝗿𝘃𝟲𝟰𝗶 data
//     Ξ𝔯𝔳32 — 𝗿𝘃𝟯𝟮𝗲/𝗿𝘃𝟯𝟮𝗶data
//     Ξ𝔯𝔳64 — 𝗿𝘃𝟲𝟰𝗶 data
//     Ξ𝔢𝔞𝔟𝔦 — 𝔢𝔞𝔟𝔦 data
//     Ξ𝔲𝔞𝔟𝔦 — 𝔲𝔞𝔟𝔦 (aka 𝔞𝔟𝔦) data
// Additional expandable markers:
//     𝔻𝕚𝕤𝕒𝕤𝕤𝕖𝕞𝕓𝕝𝕖𝕣𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟𝕤 — default implementation of disassembler instructions which call self.instruction function
//     𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟𝕤𝕀𝕟𝕥𝕖𝕣𝕗𝕒𝕔𝕖 — extra clauses for the list of instructions from SQL database
//     𝔽𝕠𝕣𝕨𝕒𝕣𝕕𝕀𝕞𝕡𝕝𝕖𝕞𝕖𝕟𝕥𝕋𝕣𝕒𝕚𝕥𝕤 — forwarding traits
#[proc_macro]
pub fn 𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘(items: TokenStream) -> TokenStream {
    let mut iter = items.into_iter();
    let Some(TokenTree::Ident(macro_name)) = iter.next() else {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't find the name of nested macro.\");"
            .parse()
            .unwrap();
    };

    let Some(TokenTree::Punct(exclamation_mark)) = iter.next() else {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't find ! after the name of nested macro.\");"
            .parse()
            .unwrap();
    };
    if exclamation_mark.as_char() != '!' {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't find ! after the name of nested macro.\");"
            .parse()
            .unwrap();
    }

    let Some(TokenTree::Group(main_group)) = iter.next() else {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't find main group to process.\");"
            .parse()
            .unwrap();
    };
    let Delimiter::Brace = main_group.delimiter() else {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — main group should use braces.\");"
            .parse()
            .unwrap();
    };
    if iter.next().is_some() {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — spurious tokens after main group.\");"
            .parse()
            .unwrap();
    }

    let mut main_group_iter = main_group.stream().into_iter();
    let mut attributes_iter = main_group_iter.clone();
    let Some(TokenTree::Group(attributes_group)) = attributes_iter.next() else {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't find attributes group to process.\");"
            .parse()
            .unwrap();
    };
    let Delimiter::Bracket = attributes_group.delimiter() else {
        return "compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — attributes_group group should use brackets.\");"
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
    let attributes = match 𝗿𝗶𝘀𝗰_𝘃::𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬::new(
        &mut attributes_iter,
        &𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌,
    ) {
        Ok(attributes) => attributes,
        Err(err) => return err.parse().unwrap(),
    };

    let mut 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌 = TokenStream::new();
    𝗿𝗶𝘀𝗰_𝘃::filter_riscv_markers_iterable(&mut 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌, &mut None, &mut main_group_iter, attributes);
    let mut result = TokenStream::new();
    result.extend([
        TokenTree::Ident(macro_name),
        TokenTree::Punct(exclamation_mark),
        Into::<TokenTree>::into(Group::new(Delimiter::Brace, 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌)),
    ]);
    result
}

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
//     𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟𝕤𝕀𝕟𝕥𝕖𝕣𝕗𝕒𝕔𝕖  — extra clauses for the list of instructions from SQL database
//     𝔽𝕠𝕣𝕨𝕒𝕣𝕕𝕀𝕞𝕡𝕝𝕖𝕞𝕖𝕟𝕥𝕋𝕣𝕒𝕚𝕥𝕤 — 𝐝𝐚𝐭𝐚_𝐩𝐫𝐞𝐟𝐢𝐱_{16,32}ᵇⁱᵗ and 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐩𝐫𝐞𝐟𝐢𝐱_{16,32}ᵇⁱᵗ, forwarding traits
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
    let attributes = match 𝘅𝟴𝟲::𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬::new(
        &mut attributes_iter,
        &𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌,
    ) {
        Ok(attributes) => attributes,
        Err(err) => return err.parse().unwrap(),
    };

    let mut 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌 = TokenStream::new();
    𝘅𝟴𝟲::filter_x86_markers_iterable(&mut 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌, &mut None, &mut main_group_iter, attributes);
    let mut result = TokenStream::new();
    result.extend([
        TokenTree::Ident(macro_name),
        TokenTree::Punct(exclamation_mark),
        Into::<TokenTree>::into(Group::new(Delimiter::Brace, 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌)),
    ]);
    result
}

// Defines instructions of three styles:
//     Universal ones with no arguments (for 𝘅𝟴𝟲 module).
//     Version for 16ᵇⁱᵗ/32ᵇⁱᵗ case with “𝗶𝗮𝟯𝟮” argument (for 𝗶𝗮𝟯𝟮 module).
//     Version for 64ᵇⁱᵗ case with “𝘅𝟴𝟲_𝟲𝟰” argument (for 𝘅𝟴𝟲_𝟲𝟰 module).
#[proc_macro]
pub fn 𝖉𝖊𝖋𝖎𝖓𝖊_𝖝𝟴𝟲_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓𝖘(
    items: TokenStream,
) -> TokenStream {
    let mut iter = items.into_iter();
    match iter.next() {
        Some(TokenTree::Ident(marker)) if matches!(marker.to_string().as_ref(), "𝗶𝗮𝟯𝟮") => {
            𝘅𝟴𝟲::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈
                [𝘅𝟴𝟲::𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶 as usize]
                .𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇
                .as_ref()
        }
        Some(TokenTree::Ident(marker)) if matches!(marker.to_string().as_ref(), "𝘅𝟴𝟲_𝟲𝟰") => {
            𝘅𝟴𝟲::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈[𝘅𝟴𝟲::𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64 as usize]
                .𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇
                .as_ref()
        }
        None => 𝘅𝟴𝟲::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇.as_ref(),
        _ => "compile_error!(\"𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓𝖘! — arguments are not supported.\");",
    }
    .parse()
    .unwrap()
}

// Defines instructions of three styles for assembler and disassembler:
//     Universal ones with no arguments (for 𝗿𝗶𝘀𝗰_𝘃 module).
//     Version for 𝗿𝘃𝟯𝟮𝗲 case with “𝗿𝘃𝟯𝟮𝗲ᵃˢˢᵉᵐᵇˡᵉʳ” or “𝗿𝘃𝟯𝟮𝗲ᵈⁱˢᵃˢˢᵉᵐᵇˡᵉʳ” arguments.
//     Version for 𝗿𝘃𝟯𝟮𝗶 case with “𝗿𝘃𝟯𝟮𝗶ᵃˢˢᵉᵐᵇˡᵉʳ” or “𝗿𝘃𝟯𝟮𝗶ᵈⁱˢᵃˢˢᵉᵐᵇˡᵉʳ” arguments.
//     Version for 𝗿𝘃𝟲𝟰𝗶 case with “𝗿𝘃𝟲𝟰𝗶ᵃˢˢᵉᵐᵇˡᵉʳ” or “𝗿𝘃𝟲𝟰𝗶ᵈⁱˢᵃˢˢᵉᵐᵇˡᵉʳ”arguments.
#[proc_macro]
pub fn 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖎𝖘𝖈𝖛_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓𝖘(
    items: TokenStream,
) -> TokenStream {
    let mut iter = items.into_iter();
    match iter.next() {
        Some(TokenTree::Ident(marker)) if matches!(marker.to_string().as_ref(), "𝗿𝘃𝟯𝟮𝗲ᵃˢˢᵉᵐᵇˡᵉʳ") => {
            𝗿𝗶𝘀𝗰_𝘃::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈
                [𝗿𝗶𝘀𝗰_𝘃::𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔢 as usize]
                .𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇
                .as_ref()
        }
        Some(TokenTree::Ident(marker)) if matches!(marker.to_string().as_ref(), "𝗿𝘃𝟯𝟮𝗶ᵃˢˢᵉᵐᵇˡᵉʳ") => {
            𝗿𝗶𝘀𝗰_𝘃::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈
                [𝗿𝗶𝘀𝗰_𝘃::𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔦 as usize]
                .𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇
                .as_ref()
        }
        Some(TokenTree::Ident(marker)) if matches!(marker.to_string().as_ref(), "𝗿𝘃𝟲𝟰𝗶ᵃˢˢᵉᵐᵇˡᵉʳ") => {
            𝗿𝗶𝘀𝗰_𝘃::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈
                [𝗿𝗶𝘀𝗰_𝘃::𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳64𝔦 as usize]
                .𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇
                .as_ref()
        }
        Some(TokenTree::Ident(marker)) if matches!(marker.to_string().as_ref(), "𝗿𝘃𝟯𝟮𝗲ᵈⁱˢᵃˢˢᵉᵐᵇˡᵉʳ") => {
            𝗿𝗶𝘀𝗰_𝘃::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈
                [𝗿𝗶𝘀𝗰_𝘃::𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔢 as usize]
                .𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝖾𝗇𝗎𝗆𝗌
                .as_ref()
        }
        Some(TokenTree::Ident(marker)) if matches!(marker.to_string().as_ref(), "𝗿𝘃𝟯𝟮𝗶ᵈⁱˢᵃˢˢᵉᵐᵇˡᵉʳ") => {
            𝗿𝗶𝘀𝗰_𝘃::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈
                [𝗿𝗶𝘀𝗰_𝘃::𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔦 as usize]
                .𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝖾𝗇𝗎𝗆𝗌
                .as_ref()
        }
        Some(TokenTree::Ident(marker)) if matches!(marker.to_string().as_ref(), "𝗿𝘃𝟲𝟰𝗶ᵈⁱˢᵃˢˢᵉᵐᵇˡᵉʳ") => {
            𝗿𝗶𝘀𝗰_𝘃::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈
                [𝗿𝗶𝘀𝗰_𝘃::𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳64𝔦 as usize]
                .𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝖾𝗇𝗎𝗆𝗌
                .as_ref()
        }
        None => 𝗿𝗶𝘀𝗰_𝘃::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇.as_ref(),
        _ => "compile_error!(\"𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓𝖘! — arguments are not supported.\");",
    }
    .parse()
    .unwrap()
}

#[derive(Clone, Default, Debug)]
pub(crate) struct 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐞𝐱𝐭𝐫𝐚_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬 {
    𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾: Option<TokenTree>,
    𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾: Option<TokenTree>,
    𝗍𝗋𝖺𝗂𝗍_𝗌𝗎𝖿𝖿𝗂𝗑: Option<String>,
    𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇: Option<TokenStream>,
    𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼: Option<TokenStream>,
}

impl 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐞𝐱𝐭𝐫𝐚_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬 {
    // Note: it's not an error to have unparseable data after initial, mandatory, group.
    // We just don't get extra info in that case.
    pub(crate) fn new(input: &mut impl Iterator<Item = TokenTree>) -> Result<Self, &'static str> {
        let (restrictions_stream, mut next_item) = match input.next() {
            Some(TokenTree::Ident(pub_ident)) if pub_ident.to_string() == "pub" => match input.next() {
                Some(TokenTree::Ident(trait_ident)) if trait_ident.to_string() == "trait" => (None, input.next()),

                _ => return Ok(Default::default()),
            },
            Some(TokenTree::Ident(impl_ident)) if impl_ident.to_string() == "impl" => match input.next() {
                Some(TokenTree::Group(restrictions_group)) if matches!(restrictions_group.delimiter(), Delimiter::Bracket) => {
                    (Some(restrictions_group.stream()), input.next())
                }
                next_item => (None, next_item),
            },
            _ => return Ok(Default::default()),
        };
        let 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 = match next_item {
            Some(TokenTree::Ident(_)) => next_item,
            Some(TokenTree::Group(trait_group)) if matches!(trait_group.delimiter(), Delimiter::None) => {
                let mut trait_iter = trait_group.stream().into_iter();
                let 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 = trait_iter.next();
                let Some(TokenTree::Ident(_)) = 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 else {
                    return Ok(Default::default());
                };
                if trait_iter.next().is_some() {
                    return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — couldn't parse optional data.\");");
                }
                𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾
            }
            _ => return Ok(Default::default()),
        };
        let 𝗍𝗋𝖺𝗂𝗍_𝗌𝗎𝖿𝖿𝗂𝗑 = {
            let Some(TokenTree::Ident(ref 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾)) = 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 else {
                return Ok(Default::default());
            };
            let 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 = 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.to_string();
            let 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 = 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.as_bytes();
            let mut index = 0;
            while index < 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.len() {
                // SAFETY: guaranteed by while check.
                if unsafe { *𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.get_unchecked(index) } == b'_' {
                    break;
                }
                index += 1;
            }
            // SAFETY: guaranteed by UTF-8.
            Some(unsafe {
                String::from_utf8_unchecked(𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾[index..𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.len()].to_vec())
            })
        };
        match input.next() {
            Some(TokenTree::Ident(for_ident)) if for_ident.to_string() == "for" => (),
            _ => {
                return Ok(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐞𝐱𝐭𝐫𝐚_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬 {
                    𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾: None,
                    𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾,
                    𝗍𝗋𝖺𝗂𝗍_𝗌𝗎𝖿𝖿𝗂𝗑,
                    𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇: restrictions_stream,
                    𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼: None,
                })
            }
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
                if struct_iter.next().is_some() {
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
            𝗍𝗋𝖺𝗂𝗍_𝗌𝗎𝖿𝖿𝗂𝗑,
            𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇: restrictions_stream,
            𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼: params_stream,
        })
    }
}

fn 𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾(name: &str) -> String {
    let mut 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 = Vec::new();
    for c in name.as_bytes() {
        if *c >= 128 {
            𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.push(*c)
        } else {
            let c = if core::ptr::eq(c, &name.as_bytes()[0]) && *c >= b'a' && *c <= b'z' {
                (*c - (b'a' - b'A')) as usize
            } else {
                *c as usize
            };
            if 𝔱𝔯𝔞𝔦𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰[c * 4] > 128 {
                𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.extend_from_slice(&𝔱𝔯𝔞𝔦𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰[c * 4..c * 4 + 4])
            } else {
                𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.push(𝔱𝔯𝔞𝔦𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰[c * 4])
            }
        }
    }
    unsafe { String::from_utf8_unchecked(𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾) }
}

fn 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾(name: &str) -> String {
    let mut 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 = Vec::new();
    for c in name.as_bytes() {
        if *c >= 128 {
            𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.push(*c)
        } else {
            let c = *c as usize;
            if 𝔱𝔯𝔞𝔦𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰[c * 4] > 128 {
                𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.extend_from_slice(&𝔱𝔯𝔞𝔦𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰[c * 4..c * 4 + 4])
            } else {
                𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.push(𝔱𝔯𝔞𝔦𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰[c * 4])
            }
        }
    }
    unsafe { String::from_utf8_unchecked(𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾) }
}

#[allow(non_upper_case_globals)]
const 𝔱𝔯𝔞𝔦𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰: [u8; 512] = [
    0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9, 0, 0, 0, 10, 0,
    0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0, 16, 0, 0, 0, 17, 0, 0, 0, 18, 0, 0, 0, 19, 0, 0, 0, 20,
    0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0, 0, 0, 28, 0, 0, 0, 29, 0, 0, 0,
    30, 0, 0, 0, 31, 0, 0, 0, 32, 0, 0, 0, 33, 0, 0, 0, 34, 0, 0, 0, 35, 0, 0, 0, 36, 0, 0, 0, 37, 0, 0, 0, 38, 0, 0, 0, 39, 0, 0,
    0, 40, 0, 0, 0, 41, 0, 0, 0, 42, 0, 0, 0, 43, 0, 0, 0, 44, 0, 0, 0, 45, 0, 0, 0, 95, 0, 0, 0, 47, 0, 0, 0, 48, 0, 0, 0, 49, 0,
    0, 0, 50, 0, 0, 0, 52, 0, 0, 0, 52, 0, 0, 0, 53, 0, 0, 0, 54, 0, 0, 0, 55, 0, 0, 0, 56, 0, 0, 0, 57, 0, 0, 0, 58, 0, 0, 0, 59,
    0, 0, 0, 60, 0, 0, 0, 61, 0, 0, 0, 62, 0, 0, 0, 63, 0, 0, 0, 64, 0, 0, 0, 240, 157, 145, 168, 240, 157, 145, 169, 240, 157,
    145, 170, 240, 157, 145, 171, 240, 157, 145, 172, 240, 157, 145, 173, 240, 157, 145, 174, 240, 157, 145, 175, 240, 157, 145,
    176, 240, 157, 145, 177, 240, 157, 145, 178, 240, 157, 145, 179, 240, 157, 145, 180, 240, 157, 145, 181, 240, 157, 145, 182,
    240, 157, 145, 183, 240, 157, 145, 184, 240, 157, 145, 185, 240, 157, 145, 186, 240, 157, 145, 187, 240, 157, 145, 188, 240,
    157, 145, 189, 240, 157, 145, 190, 240, 157, 145, 191, 240, 157, 146, 128, 240, 157, 146, 129, 91, 0, 0, 0, 92, 0, 0, 0, 93, 0,
    0, 0, 94, 0, 0, 0, 95, 0, 0, 0, 96, 0, 0, 0, 240, 157, 146, 130, 240, 157, 146, 131, 240, 157, 146, 132, 240, 157, 146, 133,
    240, 157, 146, 134, 240, 157, 146, 135, 240, 157, 146, 136, 240, 157, 146, 137, 240, 157, 146, 138, 240, 157, 146, 139, 240,
    157, 146, 140, 240, 157, 146, 141, 240, 157, 146, 142, 240, 157, 146, 143, 240, 157, 146, 144, 240, 157, 146, 145, 240, 157,
    146, 146, 240, 157, 146, 147, 240, 157, 146, 148, 240, 157, 146, 149, 240, 157, 146, 150, 240, 157, 146, 151, 240, 157, 146,
    152, 240, 157, 146, 153, 240, 157, 146, 154, 240, 157, 146, 155, 123, 0, 0, 0, 124, 0, 0, 0, 125, 0, 0, 0, 126, 0, 0, 0, 127,
    0, 0, 0,
];

fn 𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍_𝗇𝖺𝗆𝖾(name: &str) -> String {
    let mut 𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍_𝗇𝖺𝗆𝖾 = Vec::new();
    for c in name.as_bytes() {
        if *c >= 128 {
            𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍_𝗇𝖺𝗆𝖾.push(*c)
        } else {
            let c = *c as usize;
            if 𝔢𝔫𝔲𝔪_𝔳𝔞𝔯𝔦𝔞𝔫𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰[c * 4] >= 240 {
                𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍_𝗇𝖺𝗆𝖾.extend_from_slice(&𝔢𝔫𝔲𝔪_𝔳𝔞𝔯𝔦𝔞𝔫𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰[c * 4..c * 4 + 4])
            } else if 𝔢𝔫𝔲𝔪_𝔳𝔞𝔯𝔦𝔞𝔫𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰[c * 4] >= 128 {
                𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍_𝗇𝖺𝗆𝖾.extend_from_slice(&𝔢𝔫𝔲𝔪_𝔳𝔞𝔯𝔦𝔞𝔫𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰[c * 4..c * 4 + 3])
            } else {
                𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍_𝗇𝖺𝗆𝖾.push(𝔢𝔫𝔲𝔪_𝔳𝔞𝔯𝔦𝔞𝔫𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰[c * 4])
            }
        }
    }
    unsafe { String::from_utf8_unchecked(𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍_𝗇𝖺𝗆𝖾) }
}

#[allow(non_upper_case_globals)]
const 𝔢𝔫𝔲𝔪_𝔳𝔞𝔯𝔦𝔞𝔫𝔱_𝔠𝔥𝔞𝔯𝔞𝔠𝔱𝔢𝔯𝔰: [u8; 512] = [
    0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9, 0, 0, 0, 10, 0,
    0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0, 16, 0, 0, 0, 17, 0, 0, 0, 18, 0, 0, 0, 19, 0, 0, 0, 20,
    0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0, 0, 0, 28, 0, 0, 0, 29, 0, 0, 0,
    30, 0, 0, 0, 31, 0, 0, 0, 32, 0, 0, 0, 33, 0, 0, 0, 34, 0, 0, 0, 35, 0, 0, 0, 36, 0, 0, 0, 37, 0, 0, 0, 38, 0, 0, 0, 39, 0, 0,
    0, 40, 0, 0, 0, 41, 0, 0, 0, 42, 0, 0, 0, 43, 0, 0, 0, 44, 0, 0, 0, 45, 0, 0, 0, 95, 0, 0, 0, 47, 0, 0, 0, 48, 0, 0, 0, 49, 0,
    0, 0, 50, 0, 0, 0, 52, 0, 0, 0, 52, 0, 0, 0, 53, 0, 0, 0, 54, 0, 0, 0, 55, 0, 0, 0, 56, 0, 0, 0, 57, 0, 0, 0, 58, 0, 0, 0, 59,
    0, 0, 0, 60, 0, 0, 0, 61, 0, 0, 0, 62, 0, 0, 0, 63, 0, 0, 0, 64, 0, 0, 0, 240, 157, 148, 132, 240, 157, 148, 133, 226, 132,
    173, 0, 240, 157, 148, 135, 240, 157, 148, 136, 240, 157, 148, 137, 240, 157, 148, 138, 226, 132, 140, 0, 226, 132, 145, 0,
    240, 157, 148, 141, 240, 157, 148, 142, 240, 157, 148, 143, 240, 157, 148, 144, 240, 157, 148, 145, 240, 157, 148, 146, 240,
    157, 148, 147, 240, 157, 148, 148, 226, 132, 156, 0, 240, 157, 148, 150, 240, 157, 148, 151, 240, 157, 148, 152, 240, 157, 148,
    153, 240, 157, 148, 154, 240, 157, 148, 155, 240, 157, 148, 156, 226, 132, 168, 0, 91, 0, 0, 0, 92, 0, 0, 0, 93, 0, 0, 0, 94,
    0, 0, 0, 95, 0, 0, 0, 96, 0, 0, 0, 240, 157, 148, 158, 240, 157, 148, 159, 240, 157, 148, 160, 240, 157, 148, 161, 240, 157,
    148, 162, 240, 157, 148, 163, 240, 157, 148, 164, 240, 157, 148, 165, 240, 157, 148, 166, 240, 157, 148, 167, 240, 157, 148,
    168, 240, 157, 148, 169, 240, 157, 148, 170, 240, 157, 148, 171, 240, 157, 148, 172, 240, 157, 148, 173, 240, 157, 148, 174,
    240, 157, 148, 175, 240, 157, 148, 176, 240, 157, 148, 177, 240, 157, 148, 178, 240, 157, 148, 179, 240, 157, 148, 180, 240,
    157, 148, 181, 240, 157, 148, 182, 240, 157, 148, 183, 123, 0, 0, 0, 124, 0, 0, 0, 125, 0, 0, 0, 126, 0, 0, 0, 127, 0, 0, 0,
];

#[path = "x86.rs"]
mod 𝘅𝟴𝟲;

#[path = "risc-v.rs"]
mod 𝗿𝗶𝘀𝗰_𝘃;
