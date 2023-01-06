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

extern crate proc_macro;

use {
    futures::{StreamExt, TryStreamExt},
    indoc::{formatdoc, indoc},
    maplit::hashmap,
    once_cell::sync::Lazy,
    proc_macro::{Delimiter, Group, Ident, TokenStream, TokenTree},
    std::collections::{BTreeMap, HashMap, HashSet},
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬<'ᵉˣᵗʳᵃ> {
    𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾: Option<core::num::NonZeroI8>,
    𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾: Option<core::num::NonZeroI8>,
    ₓ𝗂𝗓: Option<i8>,
    𝖺𝗏𝗑𝟧𝟣𝟤: Option<bool>,
    𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌:
        &'ᵉˣᵗʳᵃ super::𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐞𝐱𝐭𝐫𝐚_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬,
}

impl<'ᵉˣᵗʳᵃ> 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬<'ᵉˣᵗʳᵃ> {
    pub(crate) fn new(
        input: &mut impl Iterator<Item = TokenTree>,
        𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌: &'ᵉˣᵗʳᵃ super::𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐞𝐱𝐭𝐫𝐚_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬,
    ) -> Result<Self, &'static str> {
        let mut result: Self = 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬 {
            𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾: None,
            𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾: None,
            ₓ𝗂𝗓: None,
            𝖺𝗏𝗑𝟧𝟣𝟤: None,
            𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌,
        };
        let mut process = |identifier: &Ident| -> Result<(), &'static str> {
            match identifier.to_string().as_ref() {
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

impl From<Option<core::num::NonZeroI8>> for 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞 {
    fn from(size: Option<core::num::NonZeroI8>) -> 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞 {
        match size {
            value if value == core::num::NonZeroI8::new(64) => 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64,
            _ => 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶,
        }
    }
}

pub(crate) fn filter_x86_markers_iterable(
    output: &mut impl Extend<TokenTree>,
    output_extra: &mut Option<TokenStream>,
    input: &mut impl Iterator<Item = TokenTree>,
    attributes: 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬,
) {
    fn emit_tokens(
        output: &mut impl Extend<TokenTree>,
        output_extra: &mut Option<TokenStream>,
        tokens: impl IntoIterator<Item = TokenTree> + Clone,
    ) {
        if let Some(output) = output_extra.as_mut() {
            output.extend(tokens.clone());
        }
        output.extend(tokens)
    }

    fn emit_or_expand_token(
        output: &mut impl Extend<TokenTree>,
        output_extra: &mut Option<TokenStream>,
        token: TokenTree,
        attributes: 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬,
    ) {
        let TokenTree::Ident(ref ident) = token else {
            return emit_tokens(output, output_extra, [token]);
        };

        match ident.to_string().as_ref() {
            "𝔽𝕠𝕣𝕨𝕒𝕣𝕕𝕀𝕞𝕡𝕝𝕖𝕞𝕖𝕟𝕥𝕋𝕣𝕒𝕚𝕥𝕤" => {
                if attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 != core::num::NonZeroI8::new(64) {
                    let token_stream: TokenStream = format!(
                        concat!(
                            "impl Æ 𝒍𝒆𝒈𝒂𝒄𝒚_𝒎𝒐𝒅𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓 for æ",
                            "{{type 𝐝𝐚𝐭𝐚_𝐩𝐫𝐞𝐟𝐢𝐱_16ᵇⁱᵗ={};type 𝐝𝐚𝐭𝐚_𝐩𝐫𝐞𝐟𝐢𝐱_32ᵇⁱᵗ={};",
                            "type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐩𝐫𝐞𝐟𝐢𝐱_16ᵇⁱᵗ={};type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐩𝐫𝐞𝐟𝐢𝐱_32ᵇⁱᵗ={};}}"
                        ),
                        if attributes.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾 == core::num::NonZeroI8::new(16) {
                            "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱"
                        } else {
                            "𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x66>"
                        },
                        if attributes.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾 == core::num::NonZeroI8::new(16) {
                            "𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x66>"
                        } else {
                            "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱"
                        },
                        if attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 == core::num::NonZeroI8::new(16) {
                            "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱"
                        } else {
                            "𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x67>"
                        },
                        if attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾 == core::num::NonZeroI8::new(16) {
                            "𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x67>"
                        } else {
                            "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱"
                        }
                    )
                    .parse()
                    .unwrap();
                    for token in token_stream.into_iter() {
                        match token {
                            TokenTree::Ident(ref ident) if ident.to_string() == "Æ" => {
                                if let Some(ref 𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇) = attributes.𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌.𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇
                                {
                                    emit_tokens(output, output_extra, 𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇.clone().into_iter());
                                }
                            }
                            TokenTree::Ident(ref ident) if ident.to_string() == "æ" => {
                                if let Some(ref 𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾) = attributes.𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌.𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾
                                {
                                    emit_tokens(output, output_extra, [𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾.clone()]);
                                }
                                if let Some(ref 𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼) = attributes.𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌.𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼
                                {
                                    emit_tokens(output, output_extra, 𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼.clone().into_iter());
                                }
                            }
                            _ => {
                                emit_tokens(output, output_extra, [token]);
                            }
                        }
                    }
                }
                let token_stream: TokenStream = 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈
                    [Into::<𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞>::into(attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾) as usize]
                    .𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌
                    .parse()
                    .unwrap();
                for token in token_stream.into_iter() {
                    match token {
                        TokenTree::Ident(ref ident) if ident.to_string() == "Æ" => {
                            let token_stream: TokenStream = if let Some(ref 𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇) =
                                attributes.𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌.𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇
                            {
                                let mut previous_token = None;
                                for token in 𝗍𝗒𝗉𝖾_𝗋𝖾𝗌𝗍𝗋𝗂𝖼𝗍𝗂𝗈𝗇.clone().into_iter() {
                                    if let Some(previous_token) = previous_token.replace(token) {
                                        emit_tokens(output, output_extra, [previous_token]);
                                    }
                                }
                                ","
                            } else {
                                "<"
                            }
                            .parse()
                            .unwrap();
                            emit_tokens(output, output_extra, token_stream);
                        }
                        TokenTree::Ident(ref ident) if ident.to_string() == "æ" => {
                            if let Some(ref 𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾) = attributes.𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌.𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾
                            {
                                emit_tokens(output, output_extra, [𝗌𝗍𝗋𝗎𝖼𝗍_𝗇𝖺𝗆𝖾.clone()]);
                            }
                            if let Some(ref 𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼) = attributes.𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌.𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼
                            {
                                emit_tokens(output, output_extra, 𝗍𝗒𝗉𝖾_𝗀𝖾𝗇𝖾𝗋𝗂𝖼.clone().into_iter());
                            }
                        }
                        _ => emit_tokens(output, output_extra, [token]),
                    }
                }
            }
            _ => emit_tokens(output, output_extra, [token]),
        }
    }

    let mut instructions_interface: Option<TokenStream> = None;
    let mut last_token: Option<TokenTree> = None;
    for token in input {
        if let Some(unwrapped_token) = last_token.take() {
            match token {
                TokenTree::Group(mut data_group_to_process) if matches!(data_group_to_process.delimiter(), Delimiter::Bracket) => {
                    let unwrapped_token_str = unwrapped_token.to_string();
                    let unwrapped_token_ref = unwrapped_token_str.as_ref();
                    match marker_is_compatible(unwrapped_token_ref, attributes) {
                        (Some(true), attributes) => filter_x86_markers_iterable(
                            output,
                            output_extra,
                            &mut data_group_to_process.stream().into_iter(),
                            attributes,
                        ),
                        (Some(false), _) => (),
                        (None, _) if unwrapped_token_ref == "𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟𝕤𝕀𝕟𝕥𝕖𝕣𝕗𝕒𝕔𝕖" =>
                        {
                            if instructions_interface.is_some() {
                                panic!("Two 𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟𝕤𝕀𝕟𝕥𝕖𝕣𝕗𝕒𝕔𝕖 markers detected!");
                            }
                            instructions_interface.replace(TokenStream::new());
                            filter_x86_markers_iterable(
                                output,
                                &mut instructions_interface,
                                &mut data_group_to_process.stream().into_iter(),
                                attributes,
                            );
                            output_extra.as_mut().map({
                                let instructions_interface = instructions_interface.clone();
                                |output| output.extend(instructions_interface.unwrap())
                            });
                        }
                        (None, _) => {
                            emit_or_expand_token(output, output_extra, unwrapped_token, attributes);
                            let filered_content = [filter_x86_markers_group(&mut data_group_to_process, attributes)];
                            emit_tokens(output, output_extra, filered_content);
                        }
                    }
                }
                TokenTree::Group(mut data_group_to_process) => {
                    emit_or_expand_token(output, output_extra, unwrapped_token, attributes);
                    let filered_content = [filter_x86_markers_group(&mut data_group_to_process, attributes)];
                    emit_tokens(output, output_extra, filered_content);
                }
                TokenTree::Ident(_) => {
                    emit_or_expand_token(output, output_extra, unwrapped_token, attributes);
                    last_token = Some(token)
                }
                _ => {
                    emit_or_expand_token(output, output_extra, unwrapped_token, attributes);
                    emit_tokens(output, output_extra, [token]);
                }
            }
        } else if let TokenTree::Ident(_) = token {
            last_token = Some(token)
        } else if let TokenTree::Group(mut data_group_to_process) = token {
            let filered_content = [filter_x86_markers_group(&mut data_group_to_process, attributes)];
            emit_tokens(output, output_extra, filered_content)
        } else {
            emit_tokens(output, output_extra, [token])
        }
    }
    if let Some(unwrapped_token) = last_token.take() {
        emit_or_expand_token(output, output_extra, unwrapped_token, attributes);
    }
    if let Some(instructions_interface) = instructions_interface {
        let token_stream: TokenStream = (𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈
            [Into::<𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞>::into(attributes.𝖺𝖽𝖽𝗋_𝗌𝗂𝗓𝖾) as usize]
            .𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌
            .replace(
                '⋇',
                attributes
                    .𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌
                    .𝗍𝗋𝖺𝗂𝗍_𝗌𝗎𝖿𝖿𝗂𝗑
                    .as_ref()
                    .expect("Trait must be accessible when 𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟𝕤𝕀𝕟𝕥𝕖𝕣𝕗𝕒𝕔𝕖 used"),
            ))
        .parse()
        .unwrap();
        for token in token_stream.into_iter() {
            match token {
                TokenTree::Ident(ref ident) if ident.to_string() == "Æ" => {
                    let 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 = attributes
                        .𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌
                        .𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾
                        .as_ref()
                        .expect("Trait must be accessible when 𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟𝕤𝕀𝕟𝕥𝕖𝕣𝕗𝕒𝕔𝕖 used");
                    emit_tokens(output, output_extra, [𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.clone()])
                }
                TokenTree::Ident(ref ident) if ident.to_string() == "æ" => {
                    emit_tokens(output, output_extra, instructions_interface.clone())
                }
                _ => emit_tokens(output, output_extra, [token]),
            }
        }
    }
}

fn filter_x86_markers_group(
    input: &mut Group, attributes: 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬
) -> TokenTree {
    let mut content = TokenStream::new();
    filter_x86_markers_iterable(&mut content, &mut None, &mut input.stream().into_iter(), attributes);
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
        "Χ𝔦𝔷" => (Some(attributes.ₓ𝗂𝗓.is_none()), attributes),
        "Ξ𝔷𝔷" => (Some(attributes.𝖺𝗏𝗑𝟧𝟣𝟤 == Some(true)), attributes),
        "Χ𝔷𝔷" => (Some(attributes.𝖺𝗏𝗑𝟧𝟣𝟤 != Some(true)), attributes),
        _ => (None, attributes),
    }
}

#[tokio::main]
async fn get_instrution_info() -> 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
    let mut instruction_trait = HashSet::new();
    let mut instruction_traits = HashSet::new();
    let mut kind_specific_traits = [HashSet::new(), HashSet::new()];

    let mut x86_assembler_instructions = Vec::new();
    let mut assembler_instructions = [Vec::new(), Vec::new()];
    let mut leaf_assembler_instructions = [BTreeMap::new(), BTreeMap::new()];
    let mut 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌 = [BTreeMap::new(), BTreeMap::new()];
    let mut 𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌 = [Vec::new(), Vec::new()];

    let mut connection = get_database_connection().await;

    // We need that trick because of SQLx design: https://github.com/launchbadge/sqlx/issues/1594#issuecomment-1100763779
    // Keep query string alive in our function — that way it wouldn't become stale while we are processing instructions list.
    let mut query = String::new();
    for assembler_kind in [𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶, 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64]
    {
        for arguments_count in 0..=5 {
            let mut instructions_stream = get_insructions_info(&mut connection, arguments_count, assembler_kind, &mut query);
            while let Some(instruction) = instructions_stream.try_next().await.expect("Connection aborted") {
                let 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌 = &instruction.𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌;

                let arguments_sql_types = 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌
                    .iter()
                    .map(|argument| argument.𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.as_str())
                    .collect::<Vec<_>>();
                let arguments_comma = if 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌.len() == 1 {
                    ","
                } else {
                    ""
                };

                let 𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑 = instruction.𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑.as_str();

                // Only instructions with two or more operands can have rex/norew operand mixup.
                // And only instructions with one or two operands can accept 8ᵇⁱᵗ arguments.
                // This means we need to only care about 2-operand instructions here.
                if arguments_count == 2
                    && (arguments_sql_types[0] == "norex_register_8bit"
                        && (arguments_sql_types[1] == "rex_register_8bit"
                            || arguments_sql_types[1].starts_with("address")
                            || arguments_sql_types[1].starts_with("gp_register")
                            || 𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑 == "data_size_prefix_64bit")
                        || (arguments_sql_types[0] == "rex_register_8bit"
                            || arguments_sql_types[0].starts_with("address")
                            || arguments_sql_types[0].starts_with("gp_register")
                            || 𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑 == "data_size_prefix_64bit")
                            && arguments_sql_types[1] == "norex_register_8bit")
                {
                    continue;
                }

                let 𝖿𝗇_𝗇𝖺𝗆𝖾 = instruction.𝖿𝗇_𝗇𝖺𝗆𝖾.as_str();
                let fn_name_adjusted = match 𝖿𝗇_𝗇𝖺𝗆𝖾 {
                    "in" => "r#in",
                    "loop" => "r#loop",
                    _ => 𝖿𝗇_𝗇𝖺𝗆𝖾,
                };

                let legacy_push_segment = 𝖿𝗇_𝗇𝖺𝗆𝖾.starts_with("push")
                    && arguments_sql_types.len() == 1
                    && arguments_sql_types[0] == "legacy_segment_register_no_cs";

                let x87_instruction_wait_prefix = 𝖿𝗇_𝗇𝖺𝗆𝖾.starts_with("fn") && 𝖿𝗇_𝗇𝖺𝗆𝖾 != "fnop";

                let 𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 = instruction.𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.as_str();
                let 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 = instruction.𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.as_str();

                if instruction_trait.insert(𝖿𝗇_𝗇𝖺𝗆𝖾.to_owned()) {
                    let instructions_trait = format!("pub trait {𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>{{type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;fn {fn_name_adjusted}(&mut self,arguments:𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮)->Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;}}");
                    let instruction_trait = format!("pub trait {𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>{{type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;fn {𝖿𝗇_𝗇𝖺𝗆𝖾}_implementation(&mut self,arguments:𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮)->Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;}}");
                    if x87_instruction_wait_prefix {
                        x86_assembler_instructions.push(x86_fnₓ_instruction_to_fₓ_instruction(&instructions_trait));
                        x86_assembler_instructions.push(x86_fnₓ_instruction_to_fₓ_instruction(&instruction_trait));
                    }
                    x86_assembler_instructions.push(instructions_trait);
                    x86_assembler_instructions.push(instruction_trait);
                }

                if instruction_traits.insert((𝖿𝗇_𝗇𝖺𝗆𝖾.to_owned(), arguments_count)) {
                    let mut parameter_types_list = Vec::new();
                    let mut argument_types = Vec::new();
                    let mut parameters_type_list = Vec::new();
                    let mut parameters_list = Vec::new();
                    let mut parameters_convert_into = Vec::new();
                    for (i, argument) in 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌.iter().enumerate() {
                        let argument_trait = argument.𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍;
                        parameter_types_list.push(format!("𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻{i}_𝓽𝔂𝓹𝓮:{argument_trait}<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,"));
                        argument_types.push(format!("<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻{i}_𝓽𝔂𝓹𝓮 as {argument_trait}<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭"));
                        parameters_type_list.push(format!("𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻{i}_𝓽𝔂𝓹𝓮"));
                        parameters_list.push(format!("parameter{i}"));
                        parameters_convert_into.push(format!(
                            "core::convert::Into::<<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻{i}_𝓽𝔂𝓹𝓮 as {argument_trait}<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>::into(parameter{i})"
                        ));
                    }
                    let parameter_types_list = parameter_types_list.concat();
                    let argument_types = argument_types.join(",");
                    let parameters_type_list = parameters_type_list.join(",");
                    let parameters_list = parameters_list.join(",");
                    let parameters_convert_into = parameters_convert_into.join(",");
                    let impl_instruction = format!("impl<{parameter_types_list}𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮:{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<({argument_types}{arguments_comma})>>{𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}<({parameters_type_list}{arguments_comma})>for 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮{{#[allow(clippy::type_complexity)]type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞=<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as {𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<({argument_types}{arguments_comma})>>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;#[allow(clippy::type_complexity)]type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞=<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as {𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<({argument_types}{arguments_comma})>>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;#[inline(always)]fn {fn_name_adjusted}(&mut self,({parameters_list}{arguments_comma}):({parameters_type_list}{arguments_comma}))->Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>{{self.{𝖿𝗇_𝗇𝖺𝗆𝖾}_implementation(({parameters_convert_into}{arguments_comma}))}}}}");
                    if x87_instruction_wait_prefix {
                        x86_assembler_instructions.push(x86_fnₓ_instruction_to_fₓ_instruction(&impl_instruction));
                    }
                    x86_assembler_instructions.push(impl_instruction);
                }

                if kind_specific_traits[assembler_kind as usize].insert(instruction.𝖿𝗇_𝗇𝖺𝗆𝖾.to_owned()) {
                    let instruction_trait = format!("pub trait {𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>{{type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;fn {𝖿𝗇_𝗇𝖺𝗆𝖾}_with(&mut self,parameters:𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮)->Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;}}");
                    let instruction_impl = format!("impl Æ 𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>𝘅𝟴𝟲::{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮> for æ where Self:{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>{{type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞=<Self as {𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞=<Self as {𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;#[inline(always)]fn {𝖿𝗇_𝗇𝖺𝗆𝖾}_implementation(&mut self,arguments:𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮)->Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>{{self.{𝖿𝗇_𝗇𝖺𝗆𝖾}_with(arguments)}}}}");
                    if x87_instruction_wait_prefix {
                        assembler_instructions[assembler_kind as usize]
                            .push(x86_fnₓ_instruction_to_fₓ_instruction(&instruction_trait));
                        𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌[assembler_kind as usize]
                            .push(x86_fnₓ_instruction_to_fₓ_instruction(&instruction_impl));
                    }
                    assembler_instructions[assembler_kind as usize].push(instruction_trait);
                    𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌[assembler_kind as usize].push(instruction_impl);
                }

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
                for (index, argument) in 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌.iter().enumerate() {
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
                    // Note that if operand is of unknown size (e.g. 𝔭𝔯𝔢𝔣𝔢𝔱𝔠𝔥 then we shouldn't say that operand_size is Some(0).
                    // This is important because if we do that we would try to turn _0bit into _0bit and instruction with clash
                    // with itself and would be forbidden!).
                    let operand_size = if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_8bit") {
                        Some(1)
                    } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_16bit") {
                        Some(2)
                    } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_32bit") {
                        Some(4)
                    } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_48bit") {
                        Some(6)
                    } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_64bit") {
                        Some(8)
                    } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_80bit") {
                        Some(10)
                    } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_112bit") {
                        Some(14)
                    } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_128bit") {
                        Some(16)
                    } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_224bit") {
                        Some(24)
                    } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_256bit") {
                        Some(32)
                    } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_512bit") {
                        Some(64)
                    } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_752bit") {
                        Some(94)
                    } else if 𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.ends_with("_864bit") {
                        Some(108)
                    } else {
                        None
                    };

                    let operand_size_target = if 𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾.ends_with('>') {
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
                    if 𝗌𝗊𝗅_𝗈𝗉𝖾𝗋𝖺𝗇𝖽 == "implicit"
                        && !𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.starts_with("string_instruction_source_address")
                        && !𝗌𝗊𝗅_𝗍𝗒𝗉𝖾.starts_with("xlat_address")
                    {
                        parameters_list.push(format!("_parameter{index}"));
                    } else {
                        parameters_list.push(format!("parameter{index}"));
                    }
                }
                let arguments_type = format!("({}{arguments_comma})", arguments_type.join(","));
                let arguments_type_xiz = format!("({}{arguments_comma})", arguments_type_xiz.join(","));
                let arguments_trait_type = format!("({}{arguments_comma})", arguments_trait_type.join(","));
                let parameters_list = format!("({}{arguments_comma})", parameters_list.join(","));
                let process_unsized_memory = if memory_size.is_some() {
                    &[false, true][..]
                } else {
                    &[false][..]
                };

                let mut need_extra_trait = false;
                let 𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑 = instruction.𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑.as_str();
                let 𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑 = match (assembler_kind, 𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑)
                {
                    (_, "") => "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱",
                    (𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶, "address_size_prefix_16bit") =>
                    {
                        need_extra_trait = true;
                        "<Self as 𝒍𝒆𝒈𝒂𝒄𝒚_𝒎𝒐𝒅𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓>::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐩𝐫𝐞𝐟𝐢𝐱_16ᵇⁱᵗ"
                    }
                    (𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶, "address_size_prefix_32bit") =>
                    {
                        need_extra_trait = true;
                        "<Self as 𝒍𝒆𝒈𝒂𝒄𝒚_𝒎𝒐𝒅𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓>::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐩𝐫𝐞𝐟𝐢𝐱_32ᵇⁱᵗ"
                    }
                    (𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64, "address_size_prefix_32bit") => {
                        "𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x67>"
                    }
                    (𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64, "address_size_prefix_64bit") => "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱",
                    _ => panic!("Usupported config of address prefixes {𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑}"),
                };

                let (𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑, rexw_prefix) = match (assembler_kind, 𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑)
                {
                    (_, "") => ("𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴),
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
                        ("𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴)
                    }
                    (𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64, "data_size_prefix_64bit") => {
                        ("𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔯𝔢𝔵𝔴)
                    }
                    _ => panic!("Usupported config of data prefixes {𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑}"),
                };

                let 𝗋𝖾𝗉ₓ_𝗉𝗋𝖾𝖿𝗂𝗑 = instruction.𝗋𝖾𝗉ₓ_𝗉𝗋𝖾𝖿𝗂𝗑.as_deref();
                let 𝗋𝖾𝗉ₓ_𝗉𝗋𝖾𝖿𝗂𝗑 = match 𝗋𝖾𝗉ₓ_𝗉𝗋𝖾𝖿𝗂𝗑 {
                    None | Some("not_allowed") => "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱",
                    Some("0xf2") => "𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0xf2>",
                    Some("0xf3") => "𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0xf3>",
                    _ => panic!("Usupported config of repx prefixes"),
                };

                let 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗉 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗉.as_str();
                let (x0f_prefix, x3x_prefix) = match 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗉 {
                    "primary" => ("𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱", "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱"),
                    "secondary" => ("𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x0f>", "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱"),
                    "0x0f_0x38" => ("𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x0f>", "𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x38>"),
                    "0x0f_0x3a" => ("𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x0f>", "𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x3a>"),
                    _ => panic!("Usupported config of opcode_map"),
                };

                let arguments_sql_operands = 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌
                    .iter()
                    .map(|argument| argument.𝗌𝗊𝗅_𝗈𝗉𝖾𝗋𝖺𝗇𝖽.as_str())
                    .collect::<Vec<_>>();
                let 𝗈𝗉𝖼𝗈𝖽𝖾 = if let ["opcode", ..] = arguments_sql_operands[..] {
                    "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱".to_owned()
                } else {
                    format!("𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x{:02x}>", instruction.𝗈𝗉𝖼𝗈𝖽𝖾)
                };

                let immediate_opcode = match (&arguments_sql_operands[..], instruction.𝗈𝗉𝖼𝗈𝖽𝖾_𝖾𝗑𝗍𝖾𝗇𝗌𝗂𝗈𝗇)
                {
                    (["rm"] | ["rm", "immediate"], _) => "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱".to_owned(),
                    (_, Some(𝗈𝗉𝖼𝗈𝖽𝖾_𝖾𝗑𝗍𝖾𝗇𝗌𝗂𝗈𝗇)) => {
                        format!("𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x{𝗈𝗉𝖼𝗈𝖽𝖾_𝖾𝗑𝗍𝖾𝗇𝗌𝗂𝗈𝗇:02x}>")
                    }
                    (_, None) => "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱".to_owned(),
                };

                #[rustfmt::skip]
                let instruction_type = if x87_instruction_wait_prefix {
                    format!(
                        "𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧<{},{},{},{},{},{},{},{},{}>",
                        "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱",  // 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓹𝓻𝓮𝓯𝓲𝔁
                        "𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x9b>", // 𝔀𝓪𝓲𝓽_𝓹𝓻𝓮𝓯𝓲𝔁
                        𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑,         // 𝓭𝓪𝓽𝓪_𝓼𝓲𝔃𝓮_𝓹𝓻𝓮𝓯𝓲𝔁
                        𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑,      // 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮_𝓹𝓻𝓮𝓯𝓲𝔁
                        𝗋𝖾𝗉ₓ_𝗉𝗋𝖾𝖿𝗂𝗑,              // 𝓻𝓮𝓹ₓ_𝓹𝓻𝓮𝓯𝓲𝔁
                        x0f_prefix,               // 𝔁𝟬𝗙_𝓹𝓻𝓮𝓯𝓲𝔁
                        x3x_prefix,               // 𝔁𝟯𝘅_𝓹𝓻𝓮𝓯𝓲𝔁
                        𝗈𝗉𝖼𝗈𝖽𝖾,                   // 𝓸𝓹𝓬𝓸𝓭𝓮
                        immediate_opcode          // 𝓲𝓶𝓶𝓮𝓭𝓲𝓪𝓽𝓮_𝓸𝓹𝓬𝓸𝓭𝓮
                    )
                } else {
                    format!(
                        "𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧<{},{},{},{},{},{},{},{},{}>",
                        "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱", // 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓹𝓻𝓮𝓯𝓲𝔁
                        𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑,        // 𝓭𝓪𝓽𝓪_𝓼𝓲𝔃𝓮_𝓹𝓻𝓮𝓯𝓲𝔁
                        𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑,     // 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮_𝓹𝓻𝓮𝓯𝓲𝔁
                        "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱", // 𝓵𝓸𝓬𝓴_𝓹𝓻𝓮𝓯𝓲𝔁
                        𝗋𝖾𝗉ₓ_𝗉𝗋𝖾𝖿𝗂𝗑,             // 𝓻𝓮𝓹ₓ_𝓹𝓻𝓮𝓯𝓲𝔁
                        x0f_prefix,              // 𝔁𝟬𝗙_𝓹𝓻𝓮𝓯𝓲𝔁
                        x3x_prefix,              // 𝔁𝟯𝘅_𝓹𝓻𝓮𝓯𝓲𝔁
                        𝗈𝗉𝖼𝗈𝖽𝖾,                  // 𝓸𝓹𝓬𝓸𝓭𝓮
                        immediate_opcode         // 𝓲𝓶𝓶𝓮𝓭𝓲𝓪𝓽𝓮_𝓸𝓹𝓬𝓸𝓭𝓮
                    )
                };

                let (instruction_emit, instruction_trait_for_emit) = match arguments_sql_operands[..] {
                    [] | ["implicit"] | ["implicit", "implicit"] => {
                        if !arguments_sql_operands.is_empty()
                            && (arguments_sql_types[0].starts_with("string_instruction_source_address")
                                || arguments_sql_types[0].starts_with("xlat_address")) {
                            (
                                format!("if let Some(𝗌𝖾𝗀𝗆𝖾𝗇𝗍)=parameter0.𝗌𝖾𝗀𝗆𝖾𝗇𝗍{{let 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: u8 = 𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into();<Self as 𝒆𝒎𝒊𝒕_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},0>>::emit_segment_prefixes_and_opcodes(self,𝗌𝖾𝗀𝗆𝖾𝗇𝗍,[])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},0>>::emit_prefixes_and_opcodes(self,[])}}"),
                                format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},0>+𝒆𝒎𝒊𝒕_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},0>"),
                            )
                        } else if arguments_sql_operands.len() > 1
                            && (arguments_sql_types[1].starts_with("string_instruction_source_address")
                                || arguments_sql_types[1].starts_with("xlat_address")) {
                            (
                                format!("if let Some(𝗌𝖾𝗀𝗆𝖾𝗇𝗍)=parameter1.𝗌𝖾𝗀𝗆𝖾𝗇𝗍{{let 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: u8 = 𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into();<Self as 𝒆𝒎𝒊𝒕_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},0>>::emit_segment_prefixes_and_opcodes(self,𝗌𝖾𝗀𝗆𝖾𝗇𝗍,[])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},0>>::emit_prefixes_and_opcodes(self,[])}}"),
                                format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},0>+𝒆𝒎𝒊𝒕_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},0>"),
                            )
                        } else {
                            (
                                format!("<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},0>>::emit_prefixes_and_opcodes(self,[])"),
                                format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},0>"),
                            )
                        }
                    },
                    ["immediate"] | ["immediate", "implicit"] | ["implicit", "immediate"] => {
                        let imm_argument: usize = (arguments_sql_operands[0] != "immediate").into();
                        match (arguments_sql_types[imm_argument], rexw_prefix) {
                            ("absolute_address_16bit_memory_8bit" |
                             "absolute_address_16bit_memory_16bit" |
                             "absolute_address_16bit_memory_32bit", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴) => (
                                format!("if let Some(𝗌𝖾𝗀𝗆𝖾𝗇𝗍)=parameter{imm_argument}.𝗌𝖾𝗀𝗆𝖾𝗇𝗍{{let 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: u8 = 𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into();<Self as 𝒆𝒎𝒊𝒕_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},2>>::emit_segment_prefixes_and_opcodes(self,𝗌𝖾𝗀𝗆𝖾𝗇𝗍,[parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍 as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>8)as u8])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},2>>::emit_prefixes_and_opcodes(self,[parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍 as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>8)as u8])}}"),
                                format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},2>+𝒆𝒎𝒊𝒕_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},2>"),
                            ),
                            ("absolute_address_32bit_memory_8bit" |
                             "absolute_address_32bit_memory_16bit" |
                             "absolute_address_32bit_memory_32bit", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴) => (
                                format!("if let Some(𝗌𝖾𝗀𝗆𝖾𝗇𝗍)=parameter{imm_argument}.𝗌𝖾𝗀𝗆𝖾𝗇𝗍{{let 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: u8 = 𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into();<Self as 𝒆𝒎𝒊𝒕_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>>::emit_segment_prefixes_and_opcodes(self,𝗌𝖾𝗀𝗆𝖾𝗇𝗍,[parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍 as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>8)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>16)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>24) as u8])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>>::emit_prefixes_and_opcodes(self,[parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍 as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>8)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>16)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>24)as u8])}}"),
                                format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>+𝒆𝒎𝒊𝒕_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>"),
                            ),
                            ("absolute_address_32bit_memory_64bit", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔯𝔢𝔵𝔴) => (
                                format!("if let Some(𝗌𝖾𝗀𝗆𝖾𝗇𝗍)=parameter{imm_argument}.𝗌𝖾𝗀𝗆𝖾𝗇𝗍{{let 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: u8 = 𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into();<Self as 𝒆𝒎𝒊𝒕_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>>::emit_segment_prefixes_rex_and_opcodes(self,𝗌𝖾𝗀𝗆𝖾𝗇𝗍,0x48,[parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍 as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>8)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>16)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>24) as u8])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>>::emit_prefixes_rex_and_opcodes(self,0x48,[parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍 as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>8)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>16)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>24)as u8])}}"),
                                format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>+𝒆𝒎𝒊𝒕_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>"),
                            ),
                            ("absolute_address_64bit_memory_8bit" |
                             "absolute_address_64bit_memory_16bit" |
                             "absolute_address_64bit_memory_32bit", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴) => (
                                format!("if let Some(𝗌𝖾𝗀𝗆𝖾𝗇𝗍)=parameter{imm_argument}.𝗌𝖾𝗀𝗆𝖾𝗇𝗍{{let 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: u8 = 𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into();<Self as 𝒆𝒎𝒊𝒕_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},8>>::emit_segment_prefixes_and_opcodes(self,𝗌𝖾𝗀𝗆𝖾𝗇𝗍,[parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍 as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>8)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>16)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>24) as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>32)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>40)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>48)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>56)as u8])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},8>>::emit_prefixes_and_opcodes(self,[parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍 as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>8)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>16)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>24)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>32)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>40)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>48)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>56)as u8])}}"),
                                format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},8>+𝒆𝒎𝒊𝒕_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},8>"),
                            ),
                            ("absolute_address_64bit_memory_64bit", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔯𝔢𝔵𝔴) => (
                                format!("if let Some(𝗌𝖾𝗀𝗆𝖾𝗇𝗍)=parameter{imm_argument}.𝗌𝖾𝗀𝗆𝖾𝗇𝗍{{let 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: u8 = 𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into();<Self as 𝒆𝒎𝒊𝒕_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},8>>::emit_segment_prefixes_rex_and_opcodes(self,𝗌𝖾𝗀𝗆𝖾𝗇𝗍,0x48,[parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍 as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>8)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>16)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>24) as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>32)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>40)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>48)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>56)as u8])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},8>>::emit_prefixes_rex_and_opcodes(self,0x48,[parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍 as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>8)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>16)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>24)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>32)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>40)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>48)as u8,(parameter{imm_argument}.𝗈𝖿𝖿𝗌𝖾𝗍>>56)as u8])}}"),
                                format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},8>+𝒆𝒎𝒊𝒕_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},8>"),
                            ),
                            ("imm8", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴) => (
                                format!("<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_and_opcodes(self,[parameter{imm_argument} as u8])"),
                                format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>"),
                            ),
                            ("imm16", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴) => (
                                format!("<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},2>>::emit_prefixes_and_opcodes(self,[parameter{imm_argument} as u8,(parameter{imm_argument}>>8)as u8])"),
                                format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},2>"),
                            ),
                            ("imm32", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴) => (
                                format!("<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>>::emit_prefixes_and_opcodes(self,[parameter{imm_argument} as u8,(parameter{imm_argument}>>8)as u8,(parameter{imm_argument}>>16)as u8,(parameter{imm_argument}>>24)as u8])"),
                                format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>"),
                            ),
                            ("imm32", 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔯𝔢𝔵𝔴) => (
                                format!("<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>>::emit_prefixes_rex_and_opcodes(self,0b0100_1000,[parameter{imm_argument} as u8,(parameter{imm_argument}>>8)as u8,(parameter{imm_argument}>>16)as u8,(parameter{imm_argument}>>24)as u8])"),
                                format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>"),
                            ),
                            _ => panic!("Unsupported combination of instruction arguments and prefixes"),
                        }
                    }
                    ["immediate", "immediate"] => {
                        match (&arguments_sql_types[..], rexw_prefix) {
                        (["imm16", "imm8"], 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴) => (
                            format!("<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},3>>::emit_prefixes_and_opcodes(self,[parameter0 as u8,(parameter0>>8)as u8,parameter1 as u8])"),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},3>"),
                        ),
                        (["imm16", "imm16"], 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴) => (
                            format!("<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>>::emit_prefixes_and_opcodes(self,[parameter0 as u8,(parameter0>>8)as u8,parameter1 as u8,(parameter1>>8)as u8])"),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},4>"),
                        ),
                        (["imm16", "imm32"], 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴) => (
                            format!("<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},6>>::emit_prefixes_and_opcodes(self,[parameter0 as u8,(parameter0>>8)as u8,parameter1 as u8,(parameter1>>8)as u8,(parameter1>>16)as u8,(parameter1>>24)as u8])"),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},6>"),
                        ),
                        _ => panic!("Unsupported combination of instruction arguments and prefixes"),
                        }
                    }
                    // These six instructions are filling ³⁄₁₆ of the opcode table.
                    // There probably would never be any others, but better to verify.
                    ["opcode"] => match (𝖿𝗇_𝗇𝖺𝗆𝖾, &arguments_sql_types[..], assembler_kind) {
                        ("bswap" | "dec" | "inc" | "pop" | "push", ["gp_register_16bit"] | ["gp_register_32bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|𝗋𝖾𝗀])", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>"),
                        ),
                        ("bswap", ["gp_register_64bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();if(𝗋𝖾𝗀&8)!=0{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_rex_and_opcodes(self,0x49,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7)])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_rex_and_opcodes(self,0x48,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7)])}}", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>"),
                        ),
                        ("bswap", ["gp_register_16bit"] | ["gp_register_32bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) |
                        ("pop" | "push", ["gp_register_16bit" | "gp_register_64bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();if(𝗋𝖾𝗀&8)!=0{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_rex_and_opcodes(self,0x41,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7)])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7)])}}", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>"),
                        ),
                        ("pop" | "popd" | "popq" | "popw" | "push" | "pushd" | "pushq" | "pushw",
                         ["legacy_segment_register_no_cs" | "segment_register_no_cs"],
                         𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶 | 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => {
                            let 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾;
                            let instruction_no_fs_gs = instruction_type.replace("𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x0f>", "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱");
                            let opcode_no_fs_gs = 𝗈𝗉𝖼𝗈𝖽𝖾 - 0x60;
                            (   format!("let 𝗋𝖾𝗀:u8=parameter0.into();if 𝗋𝖾𝗀>=0x40{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}^(𝗋𝖾𝗀<<3)])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_no_fs_gs},1>>::emit_prefixes_and_opcodes(self,[0x{opcode_no_fs_gs:02x}^𝗋𝖾𝗀])}}"),
                                format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>+𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_no_fs_gs},1>"),
                            )
                        },
                        _ => panic!("Unsupported combination of instruction arguments and prefixes"),
                    }
                    ["implicit", "opcode"] => match (𝖿𝗇_𝗇𝖺𝗆𝖾, &arguments_sql_types[..], assembler_kind) {
                        ("xchg", ["accumulator_register_16bit", "accumulator_register_16bit"] | ["accumulator_register_32bit", "accumulator_register_32bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶) |
                        ("xchg", ["accumulator_register_16bit", "accumulator_register_16bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}])", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>"),
                        ),
                        ("xchg", ["accumulator_register_16bit", "gp_register_16bit"] | ["accumulator_register_32bit", "gp_register_32bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶) |
                        ("xchg", ["accumulator_register_16bit", "norex_register_16bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter1.into();<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|𝗋𝖾𝗀])", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>"),
                        ),
                        ("xchg", ["accumulator_register_16bit", "gp_register_16bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter1.into();if(𝗋𝖾𝗀&8)!=0{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_rex_and_opcodes(self,0x41,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|𝗋𝖾𝗀])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|𝗋𝖾𝗀])}}", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>"),
                        ),
                        ("xchg", ["accumulator_register_32bit", "accumulator_register_32bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            "<Self as 𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<2>>::emit_array(self,[0x87,0xc0])".to_owned(),
                            "𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<2>".to_owned(),
                        ),
                        ("xchg", ["accumulator_register_32bit", "gp_register_32bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter1.into();if(𝗋𝖾𝗀==0){{<Self as 𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<2>>::emit_array(self,[0x87,0xc0])}}else if(𝗋𝖾𝗀&8)!=0{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_rex_and_opcodes(self,0x41,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|𝗋𝖾𝗀])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|𝗋𝖾𝗀])}}", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>+𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<2>"),
                        ),
                        ("xchg", ["accumulator_register_32bit", "norex_register_32bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter1.into();if(𝗋𝖾𝗀==0){{<Self as 𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<2>>::emit_array(self,[0x87,0xc0])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|𝗋𝖾𝗀])}}", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>+𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<2>"),
                        ),
                        ("xchg", ["accumulator_register_64bit", "accumulator_register_64bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            "<Self as 𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<1>>::emit_array(self,[0x90])".to_owned(),
                            "𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<1>".to_owned(),
                        ),
                        ("xchg", ["accumulator_register_64bit", "gp_register_64bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter1.into();if(𝗋𝖾𝗀==0){{<Self as 𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<1>>::emit_array(self,[0x90])}}else if(𝗋𝖾𝗀&8)!=0{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_rex_and_opcodes(self,0x49,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7)])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_rex_and_opcodes(self,0x41,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7)])}}", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>+𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<1>"),
                        ),
                        ("xchg", ["accumulator_register_64bit", "norex_register_64bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter1.into();if(𝗋𝖾𝗀==0){{<Self as 𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<1>>::emit_array(self,[0x90])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_rex_and_opcodes(self,0x41,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7)])}}", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>+𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<1>"),
                        ),
                        _ => panic!("Unsupported combination of instruction arguments and prefixes"),
                    }
                    ["opcode", "implicit"] => match (𝖿𝗇_𝗇𝖺𝗆𝖾, &arguments_sql_types[..], assembler_kind) {
                        ("xchg", ["gp_register_16bit", "accumulator_register_16bit"] | ["gp_register_32bit", "accumulator_register_32bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶) |
                        ("xchg", ["norex_register_16bit", "accumulator_register_16bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|𝗋𝖾𝗀])", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>"),
                        ),
                        ("xchg", ["gp_register_16bit", "accumulator_register_16bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();if(𝗋𝖾𝗀&8)!=0{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_rex_and_opcodes(self,0x41,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|𝗋𝖾𝗀])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|𝗋𝖾𝗀])}}", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>"),
                        ),
                        ("xchg", ["gp_register_32bit", "accumulator_register_32bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();if(𝗋𝖾𝗀==0){{<Self as 𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<2>>::emit_array(self,[0x87,0xc0])}}else if(𝗋𝖾𝗀&8)!=0{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_rex_and_opcodes(self,0x41,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|𝗋𝖾𝗀])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|𝗋𝖾𝗀])}}", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>+𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<2>"),
                        ),
                        ("xchg", ["norex_register_32bit", "accumulator_register_32bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();if(𝗋𝖾𝗀==0){{<Self as 𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<2>>::emit_array(self,[0x87,0xc0])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|𝗋𝖾𝗀])}}", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>+𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<2>"),
                        ),
                        ("xchg", ["gp_register_64bit", "accumulator_register_64bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();if(𝗋𝖾𝗀==0){{<Self as 𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<1>>::emit_array(self,[0x90])}}else if(𝗋𝖾𝗀&8)!=0{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_rex_and_opcodes(self,0x49,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7)])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_rex_and_opcodes(self,0x41,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7)])}}", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>+𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<1>"),
                        ),
                        ("xchg", ["norex_register_64bit", "accumulator_register_64bit"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();if(𝗋𝖾𝗀==0){{<Self as 𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<1>>::emit_array(self,[0x90])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>>::emit_prefixes_rex_and_opcodes(self,0x41,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7)])}}", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},1>+𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<1>"),
                        ),
                        _ => panic!("Unsupported combination of instruction arguments and prefixes"),
                    }
                    ["opcode", "immediate"] => match (𝖿𝗇_𝗇𝖺𝗆𝖾, &arguments_sql_types[..], assembler_kind) {
                        ("mov", ["accumulator_register_8bit" | "gp_register_8bit" | "low_register_8bit" | "norex_register_8bit", "imm8"], _) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},2>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|𝗋𝖾𝗀,parameter1 as u8])", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},2>"),
                        ),
                        ("mov", ["rex_register_8bit", "imm8"], _) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();if(𝗋𝖾𝗀&8)!=0{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},2>>::emit_prefixes_rex_and_opcodes(self,0x41,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7),parameter1 as u8])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},2>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7),parameter1 as u8])}}", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},2>"),
                        ),
                        ("mov", ["accumulator_register_16bit" | "gp_register_16bit", "imm16"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶) |
                        ("mov", ["accumulator_register_16bit" | "norex_register_16bit", "imm16"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64)=> (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},3>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|𝗋𝖾𝗀,parameter1 as u8,(parameter1>>8)as u8])", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},3>"),
                        ),
                        ("mov", ["gp_register_16bit", "imm16"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();if(𝗋𝖾𝗀&8)!=0{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},3>>::emit_prefixes_rex_and_opcodes(self,0x41,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7),parameter1 as u8,(parameter1>>8)as u8])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},3>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7),parameter1 as u8,(parameter1>>8)as u8])}}", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},3>"),
                        ),
                        ("mov", ["accumulator_register_32bit" | "gp_register_32bit", "imm32"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶) |
                        ("mov", ["accumulator_register_32bit" | "norex_register_32bit", "imm32"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},5>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|𝗋𝖾𝗀,parameter1 as u8,(parameter1>>8)as u8,(parameter1>>16)as u8,(parameter1>>24)as u8])", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},5>"),
                        ),
                        ("mov", ["gp_register_32bit", "imm32"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();if(𝗋𝖾𝗀&8)!=0{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},5>>::emit_prefixes_rex_and_opcodes(self,0x41,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7),parameter1 as u8,(parameter1>>8)as u8,(parameter1>>16)as u8,(parameter1>>24)as u8])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},5>>::emit_prefixes_and_opcodes(self,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7),parameter1 as u8,(parameter1>>8)as u8,(parameter1>>16)as u8,(parameter1>>24)as u8])}}", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},5>"),
                        ),
                        ("mov", ["gp_register_64bit", "imm64"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();if(𝗋𝖾𝗀&8)!=0{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},9>>::emit_prefixes_rex_and_opcodes(self,0x49,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7),parameter1 as u8,(parameter1>>8)as u8,(parameter1>>16)as u8,(parameter1>>24)as u8,(parameter1>>32)as u8,(parameter1>>40)as u8,(parameter1>>48)as u8,(parameter1>>56)as u8])}}else{{<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},9>>::emit_prefixes_rex_and_opcodes(self,0x48,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7),parameter1 as u8,(parameter1>>8)as u8,(parameter1>>16)as u8,(parameter1>>24)as u8,(parameter1>>32)as u8,(parameter1>>40)as u8,(parameter1>>48)as u8,(parameter1>>56)as u8])}}", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},9>"),
                        ),
                        ("mov", ["accumulator_register_64bit" | "norex_register_64bit", "imm64"], 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => (
                            format!("let 𝗋𝖾𝗀:u8=parameter0.into();<Self as 𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},9>>::emit_prefixes_rex_and_opcodes(self,0x48,[0x{𝗈𝗉𝖼𝗈𝖽𝖾:02x}|(𝗋𝖾𝗀&0x7),parameter1 as u8,(parameter1>>8)as u8,(parameter1>>16)as u8,(parameter1>>24)as u8,(parameter1>>32)as u8,(parameter1>>40)as u8,(parameter1>>48)as u8,(parameter1>>56)as u8])", 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾),
                            format!("𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<{instruction_type},9>"),
                        ),
                        _ => panic!("Unsupported combination of instruction arguments and prefixes"),
                    }
                    ["reg", "rm"] | ["reg", "rm", "implicit", "implicit"] => {
                        let (instruction_trait, instruction_fn) = match rexw_prefix {
                            𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴 => {
                                if arguments_sql_types[1].starts_with("address_16bit") {
                                    ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_8086_memory_instruction")
                                } else if arguments_sql_types[1].starts_with("address_32bit")
                                    && assembler_kind != 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64
                                    || arguments_sql_types[1].starts_with("norex_address_32bit")
                                {
                                    ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_80386_memory_instruction")
                                } else if arguments_sql_types[1].starts_with("address")
                                    || arguments_sql_types[1].starts_with("norex_address")
                                {
                                    ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_ₓ86_64_memory_instruction")
                                } else {
                                    ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction")
                                }
                            }
                            𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔯𝔢𝔵𝔴 => {
                                if arguments_sql_types[1].starts_with("address")
                                    || arguments_sql_types[1].starts_with("norex_address")
                                {
                                    (
                                        "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                        "emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw",
                                    )
                                } else {
                                    ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction_with_rexw")
                                }
                            }
                        };
                        let adjust_reg = match (arguments_sql_types[0], assembler_kind) {
                            ("segment_register", 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶) => "let parameter0:𝐬𝐞𝐠𝐦𝐞𝐧𝐭_8086_𝐫𝐞𝐠=parameter0.into();",
                            ("segment_register", 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => "let parameter0=(parameter0 as u8) & 0x7;",
                            _ => "",
                        };
                        (
                            format!(
                                "{adjust_reg}<Self as {instruction_trait}<{instruction_type}>>::{instruction_fn}(self,parameter0,parameter1)"
                            ),
                            format!("{instruction_trait}<{instruction_type}>"),
                        )
                    }
                    ["reg", "rm", "immediate"] => {
                        let (instruction_trait, instruction_fn) = match rexw_prefix {
                            𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴 => {
                                if arguments_sql_types[1].starts_with("address_16bit") {
                                    ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_8086_memory_instruction_with_i")
                                } else if arguments_sql_types[1].starts_with("address_32bit")
                                    && assembler_kind != 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64
                                    || arguments_sql_types[1].starts_with("norex_address_32bit")
                                {
                                    ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_80386_memory_instruction_with_i")
                                } else if arguments_sql_types[1].starts_with("address")
                                    || arguments_sql_types[1].starts_with("norex_address")
                                {
                                    ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_ₓ86_64_memory_instruction_with_i")
                                } else {
                                    ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction_with_i")
                                }
                            }
                            𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔯𝔢𝔵𝔴 => {
                                if arguments_sql_types[1].starts_with("address")
                                    || arguments_sql_types[1].starts_with("norex_address")
                                {
                                    (
                                        "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                        "emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_i",
                                    )
                                } else {
                                    ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction_with_rexw_and_i")
                                }
                            }
                        };
                        let immediate_size = &arguments_sql_types[2][3..];
                        (
                            format!(
                                "<Self as {instruction_trait}<{instruction_type}>>::{instruction_fn}{immediate_size}(self,parameter0,parameter1,parameter2)"
                            ),
                            format!("{instruction_trait}<{instruction_type}>"),
                        )
                    }
                    ["rm", "reg"] | ["rm", "reg", "implicit"] | ["rm", "implicit", "implicit", "reg"] => {
                        let reg_argument = 1 + 2 * ((arguments_sql_operands[1] != "reg") as usize);
                        let (instruction_trait, instruction_fn) = match rexw_prefix {
                            𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴 => {
                                if arguments_sql_types[0].starts_with("address_16bit") {
                                    ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_8086_memory_instruction")
                                } else if arguments_sql_types[0].starts_with("address_32bit")
                                    && assembler_kind != 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64
                                    || arguments_sql_types[0].starts_with("norex_address_32bit")
                                {
                                    ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_80386_memory_instruction")
                                } else if arguments_sql_types[0].starts_with("address")
                                    || arguments_sql_types[0].starts_with("norex_address")
                                {
                                    ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_ₓ86_64_memory_instruction")
                                } else {
                                    ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction")
                                }
                            }
                            𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔯𝔢𝔵𝔴 => {
                                if arguments_sql_types[0].starts_with("address")
                                    || arguments_sql_types[0].starts_with("norex_address")
                                {
                                    (
                                        "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                        "emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw",
                                    )
                                } else {
                                    ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction_with_rexw")
                                }
                            }
                        };
                        let adjust_reg = match (arguments_sql_types[reg_argument], assembler_kind) {
                            ("segment_register", 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶) => format!("let parameter{reg_argument}:𝐬𝐞𝐠𝐦𝐞𝐧𝐭_8086_𝐫𝐞𝐠=parameter{reg_argument}.into();"),
                            ("segment_register", 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64) => format!("let parameter{reg_argument}=(parameter{reg_argument} as u8) & 0x7;"),
                            _ => "".to_owned(),
                        };
                        (
                            format!(
                                "{adjust_reg}<Self as {instruction_trait}<{instruction_type}>>::{instruction_fn}(self,parameter{reg_argument},parameter0)"
                            ),
                            format!("{instruction_trait}<{instruction_type}>"),
                        )
                    }
                    ["rm", "reg", "immediate"] => {
                        let (instruction_trait, instruction_fn) = match rexw_prefix {
                            𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴 => {
                                if arguments_sql_types[0].starts_with("address_16bit") {
                                    ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_8086_memory_instruction_with_i")
                                } else if arguments_sql_types[0].starts_with("address_32bit")
                                    && assembler_kind != 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64
                                    || arguments_sql_types[0].starts_with("norex_address_32bit")
                                {
                                    ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_80386_memory_instruction_with_i")
                                } else if arguments_sql_types[0].starts_with("address")
                                    || arguments_sql_types[0].starts_with("norex_address")
                                {
                                    ("𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_address_ₓ86_64_memory_instruction_with_i")
                                } else {
                                    ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction_with_i")
                                }
                            }
                            𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔯𝔢𝔵𝔴 => {
                                if arguments_sql_types[0].starts_with("address")
                                    || arguments_sql_types[0].starts_with("norex_address")
                                {
                                    (
                                        "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                        "emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_i",
                                    )
                                } else {
                                    ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction_with_rexw_and_i")
                                }
                            }
                        };
                        let immediate_size = &arguments_sql_types[2][3..];
                        (
                            format!(
                                "<Self as {instruction_trait}<{instruction_type}>>::{instruction_fn}{immediate_size}(self,parameter1,parameter0,parameter2)"
                            ),
                            format!("{instruction_trait}<{instruction_type}>"),
                        )
                    }
                    ["implicit", "rm"] | ["rm"] | ["rm", "implicit"] => {
                        let rm_argument: usize = (arguments_sql_operands[0] != "rm").into();
                        let 𝗈𝗉𝖼𝗈𝖽𝖾_𝖾𝗑𝗍𝖾𝗇𝗌𝗂𝗈𝗇 = instruction
                            .𝗈𝗉𝖼𝗈𝖽𝖾_𝖾𝗑𝗍𝖾𝗇𝗌𝗂𝗈𝗇
                            .expect("Legacy instruction can not have rm operand without either reg operand or opcode extension");
                        let (instruction_trait, instruction_fn) = match rexw_prefix {
                            𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴 => {
                                if arguments_sql_types[0].starts_with("address_16bit")
                                    || arguments_sql_types[0].starts_with("norex_address_16bit")
                                {
                                    (
                                        "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                        "emit_legacy_reg_address_8086_memory_instruction",
                                    )
                                } else if arguments_sql_types[0].starts_with("address_32bit")
                                    && assembler_kind != 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64
                                    || arguments_sql_types[0].starts_with("norex_address_32bit")
                                {
                                    (
                                        "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                        "emit_legacy_reg_address_80386_memory_instruction",
                                    )
                                } else if arguments_sql_types[0].starts_with("address")
                                    || arguments_sql_types[0].starts_with("norex_address")
                                {
                                    (
                                        "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                        "emit_legacy_reg_address_ₓ86_64_memory_instruction",
                                    )
                                } else {
                                    ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction")
                                }
                            }
                            𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔯𝔢𝔵𝔴 => {
                                if arguments_sql_types[0].starts_with("address")
                                    || arguments_sql_types[0].starts_with("norex_address")
                                {
                                    (
                                        "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                        "emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw",
                                    )
                                } else {
                                    ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction_with_rexw")
                                }
                            }
                        };
                        (   format!("<Self as {instruction_trait}<{instruction_type}>>::{instruction_fn}(self,{𝗈𝗉𝖼𝗈𝖽𝖾_𝖾𝗑𝗍𝖾𝗇𝗌𝗂𝗈𝗇},parameter{rm_argument})"),
                            format!("{instruction_trait}<{instruction_type}>"))
                    }
                    ["rm", "immediate"] => {
                        let 𝗈𝗉𝖼𝗈𝖽𝖾_𝖾𝗑𝗍𝖾𝗇𝗌𝗂𝗈𝗇 = instruction
                            .𝗈𝗉𝖼𝗈𝖽𝖾_𝖾𝗑𝗍𝖾𝗇𝗌𝗂𝗈𝗇
                            .expect("Legacy instruction can not have rm operand without either reg operand or opcode extension");
                        let (instruction_trait, instruction_fn) = match rexw_prefix {
                            𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔫𝔬𝔯𝔢𝔵𝔴 => {
                                if arguments_sql_types[0].starts_with("address_16bit")
                                    || arguments_sql_types[0].starts_with("norex_address_16bit")
                                {
                                    (
                                        "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                        "emit_legacy_reg_address_8086_memory_instruction_with_i",
                                    )
                                } else if arguments_sql_types[0].starts_with("address_32bit")
                                    && assembler_kind != 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64
                                    || arguments_sql_types[0].starts_with("norex_address_32bit")
                                {
                                    (
                                        "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                        "emit_legacy_reg_address_80386_memory_instruction_with_i",
                                    )
                                } else if arguments_sql_types[0].starts_with("address")
                                    || arguments_sql_types[0].starts_with("norex_address")
                                {
                                    (
                                        "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                        "emit_legacy_reg_address_ₓ86_64_memory_instruction_with_i",
                                    )
                                } else {
                                    ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction_with_i")
                                }
                            }
                            𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞::𝔯𝔢𝔵𝔴 => {
                                if arguments_sql_types[0].starts_with("address")
                                    || arguments_sql_types[0].starts_with("norex_address")
                                {
                                    (
                                        "𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
                                        "emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_i",
                                    )
                                } else {
                                    ("𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏", "emit_legacy_reg_rm_instruction_with_rexw_and_i")
                                }
                            }
                        };
                        let immediate_size = &arguments_sql_types[1][3..];
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
                            let far_memory_size = format!("{{-{}isize as usize}}", memory_size + 2);
                            arguments_type_buffer = arguments_type
                                .replace(format!(",{memory_size}>").as_str(), ",0>")
                                .replace(format!(",{far_memory_size}>").as_str(), ",0>");
                            &arguments_type_buffer
                        } else {
                            arguments_type
                        };

                        let instruction_info =
                            format!("impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮:𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓{extra_trait}>{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<{arguments_type}>for 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮{{type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞=<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞=<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;#[inline(always)]fn {𝖿𝗇_𝗇𝖺𝗆𝖾}_with(&mut self,{parameters_list}:{arguments_type})->Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>{{{instruction_emit}}}}}");
                        if legacy_push_segment {
                            let assembler_instructions = leaf_assembler_instructions[assembler_kind as usize]
                                .entry((
                                    𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.to_owned(),
                                    arguments_type.replace("𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐨_𝐜𝐬", "𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086"),
                                ))
                                .or_insert_with(Vec::new);
                            assembler_instructions.push(
                                instruction_info
                                    .clone()
                                    .replace("𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐨_𝐜𝐬", "𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086"),
                            );
                        }

                        if x87_instruction_wait_prefix {
                            let assembler_instructions = leaf_assembler_instructions[assembler_kind as usize]
                                .entry((𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.replace("𝒇𝒏", "𝒇"), arguments_type.to_owned()))
                                .or_insert_with(Vec::new);
                            assembler_instructions.push(x86_fnₓ_instruction_to_fₓ_instruction(&instruction_info));
                            let assembler_instructions = leaf_assembler_instructions[assembler_kind as usize]
                                .entry((𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.to_owned(), arguments_type.to_owned()))
                                .or_insert_with(Vec::new);
                            assembler_instructions.push(instruction_info.replace("𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<0x9b>", "𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱"));
                        } else {
                            let assembler_instructions = leaf_assembler_instructions[assembler_kind as usize]
                                .entry((𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.to_owned(), arguments_type.to_owned()))
                                .or_insert_with(Vec::new);
                            assembler_instructions.push(instruction_info);
                        }
                    }
                }
                for &unsized_memory in process_unsized_memory {
                    let arguments_trait_type_buffer;
                    let arguments_trait_type = if unsized_memory {
                        let memory_size = memory_size.unwrap();
                        let far_memory_size = format!("{{-{}isize as usize}}", memory_size + 2);
                        arguments_trait_type_buffer = arguments_trait_type
                            .replace(format!(",{memory_size}>").as_str(), ",0>")
                            .replace(format!(",{far_memory_size}>").as_str(), ",0>");
                        &arguments_trait_type_buffer
                    } else {
                        &arguments_trait_type
                    };

                    let instruction_info: String = format!("𝘅𝟴𝟲::{𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}<{arguments_trait_type}>");
                    if x87_instruction_wait_prefix {
                        let 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌 = 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌[assembler_kind as usize]
                            .entry((𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.replace("𝑭𝒏", "𝑭"), fn_name_adjusted.replace("fn", "f")))
                            .or_insert_with(BTreeMap::new);
                        let 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌 = 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌.entry(arguments_trait_type.to_owned()).or_insert_with(Vec::new);
                        𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌.push(instruction_info.replace("𝘅𝟴𝟲::𝑭𝒏", "𝘅𝟴𝟲::𝑭"));
                    }
                    let 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌 = 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌[assembler_kind as usize]
                        .entry((𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.to_owned(), fn_name_adjusted.to_owned()))
                        .or_insert_with(BTreeMap::new);
                    if legacy_push_segment {
                        let 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌 = 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌
                            .entry(arguments_trait_type.replace("𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐨_𝐜𝐬", "𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫"))
                            .or_insert_with(Vec::new);
                        𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌.push(instruction_info.replace("𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐨_𝐜𝐬", "𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫"));
                    }
                    let 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌 = 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌.entry(arguments_trait_type.to_owned()).or_insert_with(Vec::new);
                    𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌.push(instruction_info);
                }
            }
        }

        assembler_instructions[assembler_kind as usize].extend(
            leaf_assembler_instructions[assembler_kind as usize]
                .values()
                .filter(|assembler_instruction| assembler_instruction.len() == 1)
                .map(|assembler_instruction| assembler_instruction[0].clone()),
        );
    }

    let 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌 = 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌.map(|traits_info| {
        traits_info
            .iter()
            .map(|((𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾, 𝖿𝗇_𝗇𝖺𝗆𝖾), trait_info)| {
                let trait_info = trait_info
                    .values()
                    .filter(|trait_info| trait_info.len() == 1)
                    .map(|trait_info| trait_info[0].clone())
                    .collect::<Vec<_>>()
                    .join("+");
                format!("pub trait {𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}⋇:Æ+{trait_info}æ{{#[inline(always)]fn {𝖿𝗇_𝗇𝖺𝗆𝖾}<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>(&mut self,arguments:𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮)->Result<<Self as 𝘅𝟴𝟲::{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,<Self as 𝘅𝟴𝟲::{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>where Self:𝘅𝟴𝟲::{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>{{𝘅𝟴𝟲::{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}::<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>::{𝖿𝗇_𝗇𝖺𝗆𝖾}(self,arguments)}}}}impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮:Æ+{trait_info}>{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}⋇ for 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 æ{{}}")
            })
            .collect::<Vec<_>>()
    });

    return 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
        𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇: x86_assembler_instructions.concat(),
        𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈: [
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
                𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌: 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶 as usize].concat(),
                𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌: 𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶 as usize].concat(),
                𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇: assembler_instructions[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶 as usize].concat(),
            },
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
                𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌: 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64 as usize].concat(),
                𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌: 𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64 as usize].concat(),
                𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇: assembler_instructions[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔵86_64 as usize].concat(),
            },
        ],
    };

    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    #[repr(i8)]
    enum 𝐫𝐞𝐱𝐰_𝐭𝐲𝐩𝐞 {
        𝔫𝔬𝔯𝔢𝔵𝔴 = 0,
        𝔯𝔢𝔵𝔴 = 1,
    }

    fn x86_fnₓ_instruction_to_fₓ_instruction(text: &str) -> String {
        text.to_owned()
            .replace("fn fn", "fn f")
            .replace(".fn", ".f")
            .replace("𝑭𝒏", "𝑭")
            .replace("𝒇𝒏", "𝒇")
    }
}

async fn get_database_connection() -> sqlx::SqliteConnection {
    use sqlx::Connection;
    let root_path = std::env::current_dir().expect("Obtaining crate root path");
    let root_path = root_path.to_str().expect("Turning crate root path into unicode string");
    // Note: during regular build root_path points to the yace workspace root, but in doctests
    // we get nested crate root.  Try to access both paths.
    let database_url = format!("sqlite:{root_path}/x86-instructions.db?immutable=1");
    let database_url_fallback = format!("sqlite:{root_path}/../x86-instructions.db?immutable=1");
    let Ok(connection) = sqlx::SqliteConnection::connect(database_url.as_str()).await else {
        return sqlx::SqliteConnection::connect(database_url_fallback.as_str())
            .await
            .expect("Failed to connect to instructions.db database")
    };
    connection
}

fn get_insructions_info<'ᵉˣᵉᶜᵘᵗᵒʳ, 𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: sqlx::Database>(
    connection: impl sqlx::Executor<'ᵉˣᵉᶜᵘᵗᵒʳ, Database = 𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    operands_count: usize,
    assembler_kind: 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞,
    query: &'ᵉˣᵉᶜᵘᵗᵒʳ mut String,
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

    if operands_count == 0 {
        *query = indoc! {"
            SELECT instruction.name AS name,
                   IFNULL(instruction.data_size_prefix, '') AS data_size_prefix,
                   IFNULL(instruction.address_size_prefix, '') AS address_size_prefix,
                   repx_prefix,
                   opcode_map,
                   opcode,
                   opcode_extension
            FROM instruction LEFT JOIN
            operands ON operands = short_name
            WHERE (instruction.assembler_kind IS NULL OR instruction.assembler_kind == $1) AND
                  operands.operand0 IS NULL
            GROUP BY instruction.name
            ORDER BY instruction.name;"}
        .to_owned();
    } else {
        let mut operand_requests = Vec::new();
        let mut data_prefixes_selection = Vec::new();
        let mut address_prefixes_selection = Vec::new();
        let mut select_traits = Vec::new();
        let mut operand_information = Vec::new();
        let mut trait_information = Vec::new();
        let mut combine_prefixes = Vec::new();
        let mut assembler_kind_check = Vec::new();
        let mut type_list = Vec::new();
        let operand_count_check = if operands_count == 0 {
            "AND operands.operand0 IS NULL".to_owned()
        } else if operands_count == 5 {
            "AND operands.operand4 IS NOT NULL".to_owned()
        } else {
            format!(
                "AND operands.operand{} IS NOT NULL AND operands.operand{} IS NULL",
                operands_count - 1,
                operands_count
            )
        };
        for i in 0..operands_count {
            operand_requests.push(format!(
                "operand{i}.parameter_type AS type{i},trait{i}.name AS trait{i},operand{i}.operand_source AS operand{i},"
            ));
            data_prefixes_selection.push(format!(",IFNULL(operand{i}.data_size_prefix, '')"));
            address_prefixes_selection.push(format!(",IFNULL(operand{i}.address_size_prefix, '')"));
            let (prefix, suffix) = if i == 0 {
                ("", "".to_owned())
            } else {
                (" LEFT JOIN", format!("ON name0 = name{i}"))
            };
            select_traits.push(formatdoc! {"
                {prefix}(
                    SELECT name{i}, trait{i}
                    FROM (
                        SELECT instruction.name AS name{i}, traits_information.name AS trait{i}, priority
                        FROM instruction LEFT JOIN
                             operands ON operands = short_name {operand_count_check} LEFT JOIN
                             operand ON operand{i} = operand.name LEFT JOIN
                             traits_information ON parameter_type = allowed_operand
                             LEFT JOIN traits_priority ON traits_information.name = traits_priority.name
                        WHERE instruction.assembler_kind IS NULL OR
                              instruction.assembler_kind == $1
                        GROUP BY instruction.name, operands
                        HAVING priority = MIN(priority)
                        ORDER BY instruction.name, operands, priority
                    )
                    GROUP BY name{i}
                    HAVING priority = MAX(priority)
                    ORDER BY name{i}
                ){suffix}"});
            operand_information.push(format!(" LEFT JOIN operand AS operand{i} ON operand{i} = operand{i}.name"));
            trait_information.push(formatdoc! {"
                ,traits_information AS trait{i} ON trait{i} = trait{i}.name
                AND operand{i}.parameter_type = trait{i}.allowed_operand"});
            for j in 0..i {
                combine_prefixes.push(formatdoc! {"
                    AND (operand{i}.data_size_prefix = operand{j}.data_size_prefix OR
                         operand{i}.data_size_prefix IS NULL OR
                         operand{j}.data_size_prefix IS NULL)
                    AND (operand{i}.address_size_prefix = operand{j}.address_size_prefix OR
                         operand{i}.address_size_prefix IS NULL OR
                         operand{j}.address_size_prefix IS NULL)"});
            }
            assembler_kind_check.push(format!(
                "AND (operand{i}.assembler_kind IS NULL OR operand{i}.assembler_kind = $1)"
            ));
            type_list.push(format!(", type{i}"));
        }
        let operand_requests = operand_requests.concat();
        let data_prefixes_selection = data_prefixes_selection.concat();
        let address_prefixes_selection = address_prefixes_selection.concat();
        let select_traits = select_traits.concat();
        let operand_information = operand_information.concat();
        let trait_information = trait_information.concat();
        let combine_prefixes = combine_prefixes.concat();
        let assembler_kind_check = assembler_kind_check.concat();
        let type_list = type_list.concat();
        *query = formatdoc! {"
            SELECT name0 AS name,
                   {operand_requests}
                   MAX(IFNULL(instruction.data_size_prefix, '') {data_prefixes_selection}) AS data_size_prefix,
                   MAX(IFNULL(instruction.address_size_prefix, '') {address_prefixes_selection}) AS address_size_prefix,
                   repx_prefix,
                   opcode_map,
                   opcode,
                   opcode_extension
            FROM {select_traits} LEFT JOIN
            instruction ON name0 = instruction.name LEFT JOIN
            operands ON operands = short_name
            {operand_information}
            {trait_information}
            WHERE (instruction.assembler_kind IS NULL OR instruction.assembler_kind == $1)
                  {combine_prefixes}
                  {assembler_kind_check}
                  {operand_count_check}
            GROUP BY instruction.name {type_list}
            HAVING operands = MIN(operands)
            ORDER BY instruction.name {type_list};"};
    }
    sqlx::query(query.as_str())
        .bind(assembler_kind.as_str())
        .fetch(connection)
        .map(move |row| {
            use sqlx::Row;

            let row = row?;
            let instruction_name: String = row.try_get("name")?;
            let instruction_trait_name = super::𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾(instruction_name.as_str());
            let instruction_auto_trait_name =
                super::𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾(instruction_name.as_str());

            let mut 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌 = Vec::new();
            for i in 0..operands_count {
                const TYPE: [&str; 5] = ["type0", "type1", "type2", "type3", "type4"];
                let mut argument: String = row.try_get(TYPE[i])?;
                const TRAIT: [&str; 5] = ["trait0", "trait1", "trait2", "trait3", "trait4"];
                let argument_trait: String = row.try_get(TRAIT[i])?;
                const OPERAND: [&str; 5] = ["operand0", "operand1", "operand2", "operand3", "operand4"];
                let operand: String = row.try_get(OPERAND[i])?;

                let argument_type = *rust_types_map
                    .get(argument.as_str())
                    .expect("Failed to convert sql type to rust type");
                let argument_type_xiz = rust_types_map_xiz.get(argument.as_str()).copied();
                let argument_trait = 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱
                    .get(argument_trait.as_str())
                    .expect("Failed to convert sql type to rust type");
                if assembler_kind == 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔩𝔢𝔤𝔞𝔠𝔶
                    && (argument.starts_with("string_instruction_destination") || argument == "segment_register_no_cs")
                {
                    argument = format!("legacy_{argument}");
                }
                let argument_trait_type = 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱
                    .get(argument.as_str())
                    .expect("Failed to convert sql type to rust type");
                𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌.push(𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭_𝐭𝐲𝐩𝐞 {
                    𝗌𝗊𝗅_𝗍𝗒𝗉𝖾: argument,
                    𝗌𝗊𝗅_𝗈𝗉𝖾𝗋𝖺𝗇𝖽: operand,
                    𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾: argument_type,
                    𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾_ₓ𝔦𝔷: argument_type_xiz,
                    𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍: argument_trait,
                    𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍_𝗍𝗒𝗉𝖾: argument_trait_type,
                });
            }

            Ok(𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
                𝖿𝗇_𝗇𝖺𝗆𝖾: instruction_name,
                𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾: instruction_auto_trait_name,
                𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾: instruction_trait_name,
                𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑: row.try_get("data_size_prefix")?,
                𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑: row.try_get("address_size_prefix")?,
                𝗋𝖾𝗉ₓ_𝗉𝗋𝖾𝖿𝗂𝗑: row.try_get("repx_prefix")?,
                𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗉: row.try_get("opcode_map")?,
                𝗈𝗉𝖼𝗈𝖽𝖾: row.try_get("opcode")?,
                𝗈𝗉𝖼𝗈𝖽𝖾_𝖾𝗑𝗍𝖾𝗇𝗌𝗂𝗈𝗇: row.try_get("opcode_extension")?,
                𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌,
            })
        })
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(i8)]
pub(crate) enum 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞 {
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
    𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾: String,
    𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾: String,
    𝖽𝖺𝗍𝖺_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑: String,
    𝖺𝖽𝖽𝗋𝖾𝗌𝗌_𝗌𝗂𝗓𝖾_𝗉𝗋𝖾𝖿𝗂𝗑: String,
    𝗋𝖾𝗉ₓ_𝗉𝗋𝖾𝖿𝗂𝗑: Option<String>,
    𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗉: String,
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

pub(crate) static 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬: Lazy<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞> = Lazy::new(get_instrution_info);
static 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    hashmap! {
        "absolute_address_16bit_memory_16bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,i16,2>",
        "absolute_address_16bit_memory_32bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,i16,4>",
        "absolute_address_16bit_memory_8bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,i16,1>",
        "absolute_address_32bit_memory_16bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,i32,2>",
        "absolute_address_32bit_memory_32bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,i32,4>",
        "absolute_address_32bit_memory_64bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,i32,8>",
        "absolute_address_32bit_memory_8bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,i32,1>",
        "absolute_address_64bit_memory_16bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,i64,2>",
        "absolute_address_64bit_memory_32bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,i64,4>",
        "absolute_address_64bit_memory_64bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,i64,8>",
        "absolute_address_64bit_memory_8bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,i64,1>",
        "accumulator_register_16bit" => "Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ",
        "accumulator_register_32bit" => "Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ",
        "accumulator_register_64bit" => "Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ",
        "accumulator_register_8bit" => "Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ",
        "address_16bit_memory_0bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                             "i16,",
                                                             "0>"),
        "address_16bit_memory_112bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                               "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "i16,",
                                                               "14>"),
        "address_16bit_memory_128bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                               "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "i16,",
                                                               "16>"),
        "address_16bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "i16,",
                                                              "2>"),
        "address_16bit_memory_224bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                               "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "i16,",
                                                               "28>"),
        "address_16bit_memory_256bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                               "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "i16,",
                                                               "32>"),
        "address_16bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "i16,",
                                                              "4>"),
        "address_16bit_memory_48bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "i16,",
                                                              "6>"),
        "address_16bit_memory_512bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                               "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "i16,",
                                                               "64>"),
        "address_16bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "i16,",
                                                              "8>"),
        "address_16bit_memory_752bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                               "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "i16,",
                                                               "94>"),
        "address_16bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                             "i16,",
                                                             "1>"),
        "address_16bit_memory_80bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "i16,",
                                                              "10>"),
        "address_16bit_memory_864bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                               "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "i16,",
                                                               "108>"),
        "address_16bit_memory_far_ptr_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                      "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                                      "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                                      "i16,",
                                                                      "{-4isize as usize}>"),
        "address_16bit_memory_far_ptr_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                      "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                                      "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                                      "i16,",
                                                                      "{-6isize as usize}>"),
        "address_32bit_memory_0bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                            "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                            "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                            "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "0>"),
        "address_32bit_memory_112bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                              "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "14>"),
        "address_32bit_memory_128bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                              "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "16>"),
        "address_32bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                             "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "2>"),
        "address_32bit_memory_224bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                              "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "28>"),
        "address_32bit_memory_256bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                              "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "32>"),
        "address_32bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                             "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "4>"),
        "address_32bit_memory_48bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                             "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "6>"),
        "address_32bit_memory_512bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                              "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "64>"),
        "address_32bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                             "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "8>"),
        "address_32bit_memory_752bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                              "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "94>"),
        "address_32bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                            "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                            "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                            "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "1>"),
        "address_32bit_memory_80bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                             "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "10>"),
        "address_32bit_memory_864bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                              "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "108>"),
        "address_32bit_memory_far_ptr_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                     "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                                     "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                                     "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-4isize as usize}>"),
        "address_32bit_memory_far_ptr_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                     "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                                     "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                                     "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-6isize as usize}>"),
        "address_32bit_memory_far_ptr_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                     "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                                     "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,",
                                                                     "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-10isize as usize}>"),
        "address_64bit_memory_0bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                            "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "0>"),
        "address_64bit_memory_112bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "14>"),
        "address_64bit_memory_128bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "16>"),
        "address_64bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "2>"),
        "address_64bit_memory_224bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "28>"),
        "address_64bit_memory_256bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "32>"),
        "address_64bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "4>"),
        "address_64bit_memory_48bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "6>"),
        "address_64bit_memory_512bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "64>"),
        "address_64bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "8>"),
        "address_64bit_memory_752bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "94>"),
        "address_64bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                            "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "1>"),
        "address_64bit_memory_80bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                             "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "10>"),
        "address_64bit_memory_864bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                              "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "108>"),
        "address_64bit_memory_far_ptr_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                     "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-4isize as usize}>"),
        "address_64bit_memory_far_ptr_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                     "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-6isize as usize}>"),
        "address_64bit_memory_far_ptr_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                     "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-10isize as usize}>"),
        "assembler_operand_of_8bit_instruction" => "𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏",
        "assembler_operand_separate_accumulator" => "𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓",
        "control_register" => "Self::𝐜𝐨𝐧𝐭𝐫𝐨𝐥_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫",
        "counter_assembler_operand" => "𝒄𝒐𝒖𝒏𝒕𝒆𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅",
        "counter_register_8bit" => "Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ",
        "data_register_16bit" => "Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ",
        "debug_register" => "Self::𝐝𝐞𝐛𝐮𝐠_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫",
        "destination_string_operand" => "𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒔𝒕𝒓𝒊𝒏𝒈_𝒐𝒑𝒆𝒓𝒂𝒏𝒅",
        "generic_assembler_operand" => "𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅",
        "gp_register_16bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ",
        "gp_register_32bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ",
        "gp_register_64bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ",
        "gp_register_8bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ",
        "imm16" => "i16",
        "imm32" => "i32",
        "imm64" => "i64",
        "imm8" => "i8",
        "io_operand" => "𝒊𝒐_𝒐𝒑𝒆𝒓𝒂𝒏𝒅",
        "legacy_segment_register_no_cs" => "Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐨_𝐜𝐬",
        "legacy_string_instruction_destination_address_16bit_memory_16bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝔢𝔰_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,2>",
        "legacy_string_instruction_destination_address_16bit_memory_32bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝔢𝔰_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,4>",
        "legacy_string_instruction_destination_address_16bit_memory_8bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝔢𝔰_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,1>",
        "legacy_string_instruction_destination_address_32bit_memory_16bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝔢𝔰_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,2>",
        "legacy_string_instruction_destination_address_32bit_memory_32bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝔢𝔰_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,4>",
        "legacy_string_instruction_destination_address_32bit_memory_8bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝔢𝔰_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,1>",
        "low_register_8bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ",
        "norex_address_32bit_memory_0bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                  "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "0>"),
        "norex_address_32bit_memory_112bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                    "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "14>"),
        "norex_address_32bit_memory_128bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                    "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "16>"),
        "norex_address_32bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "2>"),
        "norex_address_32bit_memory_224bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                    "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "28>"),
        "norex_address_32bit_memory_256bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                    "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "32>"),
        "norex_address_32bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "4>"),
        "norex_address_32bit_memory_48bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "6>"),
        "norex_address_32bit_memory_512bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                    "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "64>"),
        "norex_address_32bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "8>"),
        "norex_address_32bit_memory_752bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                    "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "94>"),
        "norex_address_32bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                  "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "1>"),
        "norex_address_32bit_memory_80bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "10>"),
        "norex_address_32bit_memory_864bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                    "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "108>"),
        "norex_address_32bit_memory_far_ptr_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                           "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-4isize as usize}>"),
        "norex_address_32bit_memory_far_ptr_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                           "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-6isize as usize}>"),
        "norex_address_32bit_memory_far_ptr_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                           "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-10isize as usize}>"),
        "norex_address_64bit_memory_0bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                  "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "0>"),
        "norex_address_64bit_memory_112bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                    "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "14>"),
        "norex_address_64bit_memory_128bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                    "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "16>"),
        "norex_address_64bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "2>"),
        "norex_address_64bit_memory_224bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                    "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "28>"),
        "norex_address_64bit_memory_256bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                    "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "32>"),
        "norex_address_64bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "4>"),
        "norex_address_64bit_memory_48bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "6>"),
        "norex_address_64bit_memory_512bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                    "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "64>"),
        "norex_address_64bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "8>"),
        "norex_address_64bit_memory_752bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                    "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "94>"),
        "norex_address_64bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                  "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "1>"),
        "norex_address_64bit_memory_80bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                   "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "10>"),
        "norex_address_64bit_memory_864bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                    "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "108>"),
        "norex_address_64bit_memory_far_ptr_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                           "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-4isize as usize}>"),
        "norex_address_64bit_memory_far_ptr_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                           "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-6isize as usize}>"),
        "norex_address_64bit_memory_far_ptr_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,",
                                                                           "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-10isize as usize}>"),
        "norex_register_16bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ",
        "norex_register_32bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ",
        "norex_register_64bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ",
        "norex_register_8bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ",
        "rex_register_8bit" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ",
        "segment_register" => "Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫",
        "segment_register_no_cs" => "Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫",
        "source_string_operand" => "𝒔𝒐𝒖𝒓𝒄𝒆_𝒔𝒕𝒓𝒊𝒏𝒈_𝒐𝒑𝒆𝒓𝒂𝒏𝒅",
        "st_register" => "Self::𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫",
        "string_instruction_destination_address_16bit_memory_16bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,2>",
        "string_instruction_destination_address_16bit_memory_32bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,4>",
        "string_instruction_destination_address_16bit_memory_8bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,1>",
        "string_instruction_destination_address_32bit_memory_16bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,2>",
        "string_instruction_destination_address_32bit_memory_32bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,4>",
        "string_instruction_destination_address_32bit_memory_64bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,8>",
        "string_instruction_destination_address_32bit_memory_8bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,1>",
        "string_instruction_destination_address_64bit_memory_16bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,2>",
        "string_instruction_destination_address_64bit_memory_32bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,4>",
        "string_instruction_destination_address_64bit_memory_64bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,8>",
        "string_instruction_destination_address_64bit_memory_8bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,1>",
        "string_instruction_source_address_16bit_memory_16bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,2>",
        "string_instruction_source_address_16bit_memory_32bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,4>",
        "string_instruction_source_address_16bit_memory_8bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,1>",
        "string_instruction_source_address_32bit_memory_16bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,2>",
        "string_instruction_source_address_32bit_memory_32bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,4>",
        "string_instruction_source_address_32bit_memory_64bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,8>",
        "string_instruction_source_address_32bit_memory_8bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,1>",
        "string_instruction_source_address_64bit_memory_16bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,2>",
        "string_instruction_source_address_64bit_memory_32bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,4>",
        "string_instruction_source_address_64bit_memory_64bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,8>",
        "string_instruction_source_address_64bit_memory_8bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,1>",
        "test_register" => "Self::𝐭𝐞𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫",
        "x87_register" => "Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫",
        "xlat_operand" => "𝒙𝒍𝒂𝒕_𝒐𝒑𝒆𝒓𝒂𝒏𝒅",
        "xlat_address_16bit_memory_8bit" => "𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,1>",
        "xlat_address_32bit_memory_8bit" => "𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,1>",
        "xlat_address_64bit_memory_8bit" => "𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,1>",
    }
});
static 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_𝔩𝔢𝔤𝔞𝔠𝔶: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    hashmap! {
        "absolute_address_16bit_memory_16bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,i16,2>",
        "absolute_address_16bit_memory_32bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,i16,4>",
        "absolute_address_16bit_memory_8bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,i16,1>",
        "absolute_address_32bit_memory_16bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,i32,2>",
        "absolute_address_32bit_memory_32bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,i32,4>",
        "absolute_address_32bit_memory_64bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,i32,8>",
        "absolute_address_32bit_memory_8bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,i32,1>",
        "absolute_address_64bit_memory_16bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,i64,2>",
        "absolute_address_64bit_memory_32bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,i64,4>",
        "absolute_address_64bit_memory_64bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,i64,8>",
        "absolute_address_64bit_memory_8bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,i64,1>",
        "accumulator_register_16bit" => "𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ",
        "accumulator_register_32bit" => "𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ",
        "accumulator_register_8bit" => "𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ",
        "address_16bit_memory_0bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                             "i16,",
                                                             "0>"),
        "address_16bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "i16,",
                                                              "2>"),
        "address_16bit_memory_112bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                               "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "i16,",
                                                               "14>"),
        "address_16bit_memory_128bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                               "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "i16,",
                                                               "16>"),
        "address_16bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "i16,",
                                                              "4>"),
        "address_16bit_memory_224bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                               "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "i16,",
                                                               "28>"),
        "address_16bit_memory_256bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                               "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "i16,",
                                                               "32>"),
        "address_16bit_memory_48bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "i16,",
                                                              "6>"),
        "address_16bit_memory_512bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                               "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "i16,",
                                                               "64>"),
        "address_16bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "i16,",
                                                              "8>"),
        "address_16bit_memory_752bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                               "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "i16,",
                                                               "94>"),
        "address_16bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                             "i16,",
                                                             "1>"),
        "address_16bit_memory_80bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                              "i16,",
                                                              "10>"),
        "address_16bit_memory_864bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                               "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                               "i16,",
                                                               "108>"),
        "address_16bit_memory_far_ptr_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                                      "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                                      "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                                      "i16,",
                                                                      "{-4isize as usize}>"),
        "address_16bit_memory_far_ptr_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                                      "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                                      "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,",
                                                                      "i16,",
                                                                      "{-6isize as usize}>"),
        "address_32bit_memory_0bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "0>"),
        "address_32bit_memory_112bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "14>"),
        "address_32bit_memory_128bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "16>"),
        "address_32bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "2>"),
        "address_32bit_memory_224bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "28>"),
        "address_32bit_memory_256bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "32>"),
        "address_32bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "4>"),
        "address_32bit_memory_48bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "6>"),
        "address_32bit_memory_512bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "64>"),
        "address_32bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "8>"),
        "address_32bit_memory_752bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "94>"),
        "address_32bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "1>"),
        "address_32bit_memory_80bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "10>"),
        "address_32bit_memory_864bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "108>"),
        "address_32bit_memory_far_ptr_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                                     "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                     "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                     "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-4isize as usize}>"),
        "address_32bit_memory_far_ptr_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                                     "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                     "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                     "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-6isize as usize}>"),
        "control_register" => "𝐜𝐨𝐧𝐭𝐫𝐨𝐥_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_80386",
        "counter_register_8bit" => "𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ",
        "data_register_16bit" => "𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ",
        "debug_register" => "𝐝𝐞𝐛𝐮𝐠_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫",
        "gp_register_16bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086",
        "gp_register_32bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386",
        "gp_register_8bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086",
        "imm16" => "i16",
        "imm32" => "i32",
        "imm64" => "i64",
        "imm8" => "i8",
        "segment_register" => "𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086",
        "segment_register_no_cs" => "𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐨_𝐜𝐬",
        "st_register" => "𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫",
        "string_instruction_destination_address_16bit_memory_16bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝔢𝔰_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,2>",
        "string_instruction_destination_address_16bit_memory_32bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝔢𝔰_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,4>",
        "string_instruction_destination_address_16bit_memory_8bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝔢𝔰_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,1>",
        "string_instruction_destination_address_32bit_memory_16bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝔢𝔰_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,2>",
        "string_instruction_destination_address_32bit_memory_32bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝔢𝔰_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,4>",
        "string_instruction_destination_address_32bit_memory_8bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝔢𝔰_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,1>",
        "string_instruction_source_address_16bit_memory_16bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,2>",
        "string_instruction_source_address_16bit_memory_32bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,4>",
        "string_instruction_source_address_16bit_memory_8bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,1>",
        "string_instruction_source_address_32bit_memory_16bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,2>",
        "string_instruction_source_address_32bit_memory_32bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,4>",
        "string_instruction_source_address_32bit_memory_8bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,1>",
        "test_register" => "𝐭𝐞𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫",
        "x87_register" => "𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫",
        "xlat_address_16bit_memory_8bit" => "𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,1>",
        "xlat_address_32bit_memory_8bit" => "𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,1>",
    }
});
static 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_𝔩𝔢𝔤𝔞𝔠𝔶_𝔴𝔦𝔱𝔥_𝔢𝔦𝔷: Lazy<
    HashMap<&'static str, &'static str>,
> = Lazy::new(|| {
    hashmap! {
        "address_32bit_memory_0bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "0>"),
        "address_32bit_memory_112bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "14>"),
        "address_32bit_memory_128bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "16>"),
        "address_32bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "2>"),
        "address_32bit_memory_224bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "28>"),
        "address_32bit_memory_256bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "32>"),
        "address_32bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "4>"),
        "address_32bit_memory_48bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "6>"),
        "address_32bit_memory_512bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "64>"),
        "address_32bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "8>"),
        "address_32bit_memory_752bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "94>"),
        "address_32bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "1>"),
        "address_32bit_memory_80bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "10>"),
        "address_32bit_memory_864bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "108>"),
        "address_32bit_memory_far_ptr_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                                     "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                     "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                     "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-4isize as usize}>"),
        "address_32bit_memory_far_ptr_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,",
                                                                     "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                     "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                     "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-6isize as usize}>"),
    }
});
static 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_ₓ86_64: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    hashmap! {
        "absolute_address_16bit_memory_16bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,i16,2>",
        "absolute_address_16bit_memory_32bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,i16,4>",
        "absolute_address_16bit_memory_8bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,i16,1>",
        "absolute_address_32bit_memory_16bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,i32,2>",
        "absolute_address_32bit_memory_32bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,i32,4>",
        "absolute_address_32bit_memory_64bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,i32,8>",
        "absolute_address_32bit_memory_8bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,i32,1>",
        "absolute_address_64bit_memory_16bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,i64,2>",
        "absolute_address_64bit_memory_32bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,i64,4>",
        "absolute_address_64bit_memory_64bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,i64,8>",
        "absolute_address_64bit_memory_8bit" => "𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,i64,1>",
        "accumulator_register_16bit" => "𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ",
        "accumulator_register_32bit" => "𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ",
        "accumulator_register_64bit" => "𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ",
        "accumulator_register_8bit" => "𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ",
        "address_32bit_memory_0bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "0>"),
        "address_32bit_memory_112bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "14>"),
        "address_32bit_memory_128bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "16>"),
        "address_32bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "2>"),
        "address_32bit_memory_224bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "28>"),
        "address_32bit_memory_256bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "32>"),
        "address_32bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "4>"),
        "address_32bit_memory_48bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "6>"),
        "address_32bit_memory_512bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "64>"),
        "address_32bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "8>"),
        "address_32bit_memory_752bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "94>"),
        "address_32bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "1>"),
        "address_32bit_memory_80bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "10>"),
        "address_32bit_memory_864bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "108>"),
        "address_32bit_memory_far_ptr_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                     "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                                     "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                                     "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-4isize as usize}>"),
        "address_32bit_memory_far_ptr_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                     "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                                     "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                                     "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-6isize as usize}>"),
        "address_32bit_memory_far_ptr_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                     "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                                     "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                                     "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-10isize as usize}>"),
        "address_64bit_memory_0bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "0>"),
        "address_64bit_memory_112bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "14>"),
        "address_64bit_memory_128bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "16>"),
        "address_64bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "2>"),
        "address_64bit_memory_224bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "28>"),
        "address_64bit_memory_256bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "32>"),
        "address_64bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "4>"),
        "address_64bit_memory_48bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "6>"),
        "address_64bit_memory_512bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "64>"),
        "address_64bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "8>"),
        "address_64bit_memory_752bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "94>"),
        "address_64bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "1>"),
        "address_64bit_memory_80bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "10>"),
        "address_64bit_memory_864bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "108>"),
        "address_64bit_memory_far_ptr_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                     "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-4isize as usize}>"),
        "address_64bit_memory_far_ptr_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                     "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-6isize as usize}>"),
        "address_64bit_memory_far_ptr_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                     "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-10isize as usize}>"),
        "control_register" => "𝐜𝐨𝐧𝐭𝐫𝐨𝐥_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64",
        "counter_register_8bit" => "𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ",
        "data_register_16bit" => "𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ",
        "debug_register" => "𝐝𝐞𝐛𝐮𝐠_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫",
        "gp_register_16bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64",
        "gp_register_32bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64",
        "gp_register_64bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ",
        "imm16" => "i16",
        "imm32" => "i32",
        "imm64" => "i64",
        "imm8" => "i8",
        "low_register_8bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ",
        "norex_address_32bit_memory_0bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                  "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                  "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                  "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "0>"),
        "norex_address_32bit_memory_112bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "14>"),
        "norex_address_32bit_memory_128bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "16>"),
        "norex_address_32bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "2>"),
        "norex_address_32bit_memory_224bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "28>"),
        "norex_address_32bit_memory_256bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "32>"),
        "norex_address_32bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "4>"),
        "norex_address_32bit_memory_48bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "6>"),
        "norex_address_32bit_memory_512bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "64>"),
        "norex_address_32bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "8>"),
        "norex_address_32bit_memory_752bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "94>"),
        "norex_address_32bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                  "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                  "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                  "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "1>"),
        "norex_address_32bit_memory_80bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "10>"),
        "norex_address_32bit_memory_864bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "108>"),
        "norex_address_32bit_memory_far_ptr_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                           "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                           "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                           "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-4isize as usize}>"),
        "norex_address_32bit_memory_far_ptr_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                           "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                           "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                           "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-6isize as usize}>"),
        "norex_address_32bit_memory_far_ptr_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                           "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                           "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                           "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-10isize as usize}>"),
        "norex_address_64bit_memory_0bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                  "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "0>"),
        "norex_address_64bit_memory_112bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "14>"),
        "norex_address_64bit_memory_128bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "16>"),
        "norex_address_64bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "2>"),
        "norex_address_64bit_memory_224bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "28>"),
        "norex_address_64bit_memory_256bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "32>"),
        "norex_address_64bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "4>"),
        "norex_address_64bit_memory_48bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "6>"),
        "norex_address_64bit_memory_512bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "64>"),
        "norex_address_64bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "8>"),
        "norex_address_64bit_memory_752bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "94>"),
        "norex_address_64bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                  "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "1>"),
        "norex_address_64bit_memory_80bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "10>"),
        "norex_address_64bit_memory_864bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "108>"),
        "norex_address_64bit_memory_far_ptr_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                           "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-4isize as usize}>"),
        "norex_address_64bit_memory_far_ptr_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                           "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-6isize as usize}>"),
        "norex_address_64bit_memory_far_ptr_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                           "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-10isize as usize}>"),
        "norex_register_16bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086",
        "norex_register_32bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386",
        "norex_register_64bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ",
        "norex_register_8bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086",
        "rex_register_8bit" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64",
        "segment_register" => "𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64",
        "segment_register_no_cs" => "𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64",
        "st_register" => "𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫",
        "string_instruction_destination_address_32bit_memory_16bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,2>",
        "string_instruction_destination_address_32bit_memory_32bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,4>",
        "string_instruction_destination_address_32bit_memory_64bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,8>",
        "string_instruction_destination_address_32bit_memory_8bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,1>",
        "string_instruction_destination_address_64bit_memory_16bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,2>",
        "string_instruction_destination_address_64bit_memory_32bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,4>",
        "string_instruction_destination_address_64bit_memory_64bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,8>",
        "string_instruction_destination_address_64bit_memory_8bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐧𝐨_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,1>",
        "string_instruction_source_address_32bit_memory_16bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,2>",
        "string_instruction_source_address_32bit_memory_32bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,4>",
        "string_instruction_source_address_32bit_memory_64bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,8>",
        "string_instruction_source_address_32bit_memory_8bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,1>",
        "string_instruction_source_address_64bit_memory_16bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,2>",
        "string_instruction_source_address_64bit_memory_32bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,4>",
        "string_instruction_source_address_64bit_memory_64bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,8>",
        "string_instruction_source_address_64bit_memory_8bit" =>
            "𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,1>",
        "test_register" => "𝐭𝐞𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫",
        "x87_register" => "𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫",
        "xlat_address_32bit_memory_8bit" => "𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,1>",
        "xlat_address_64bit_memory_8bit" => "𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,1>",
    }
});
static 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_ₓ86_64_𝔴𝔦𝔱𝔥_𝔯𝔦𝔷: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    hashmap! {
        "address_32bit_memory_0bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "0>"),
        "address_32bit_memory_112bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "14>"),
        "address_32bit_memory_128bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "16>"),
        "address_32bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "2>"),
        "address_32bit_memory_224bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "28>"),
        "address_32bit_memory_256bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "32>"),
        "address_32bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "4>"),
        "address_32bit_memory_48bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "6>"),
        "address_32bit_memory_512bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "64>"),
        "address_32bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "8>"),
        "address_32bit_memory_752bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "94>"),
        "address_32bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "1>"),
        "address_32bit_memory_80bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "10>"),
        "address_32bit_memory_864bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "108>"),
        "address_32bit_memory_far_ptr_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                     "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                                     "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                                     "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-4isize as usize}>"),
        "address_32bit_memory_far_ptr_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                     "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                                     "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                                     "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-6isize as usize}>"),
        "address_32bit_memory_far_ptr_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                     "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                                     "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,",
                                                                     "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-10isize as usize}>"),
        "address_64bit_memory_0bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "0>"),
        "address_64bit_memory_112bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "14>"),
        "address_64bit_memory_128bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "16>"),
        "address_64bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "2>"),
        "address_64bit_memory_224bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "28>"),
        "address_64bit_memory_256bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "32>"),
        "address_64bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "4>"),
        "address_64bit_memory_48bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "6>"),
        "address_64bit_memory_512bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "64>"),
        "address_64bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "8>"),
        "address_64bit_memory_752bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "94>"),
        "address_64bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                            "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                            "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                            "i32,",
                                                            "1>"),
        "address_64bit_memory_80bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                             "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                             "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                             "i32,",
                                                             "10>"),
        "address_64bit_memory_864bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                              "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                              "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                              "i32,",
                                                              "108>"),
        "address_64bit_memory_far_ptr_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                     "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-4isize as usize}>"),
        "address_64bit_memory_far_ptr_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                     "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-6isize as usize}>"),
        "address_64bit_memory_far_ptr_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                     "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,",
                                                                     "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                     "i32,",
                                                                     "{-10isize as usize}>"),
        "norex_address_32bit_memory_0bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                  "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                  "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                  "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "0>"),
        "norex_address_32bit_memory_112bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "14>"),
        "norex_address_32bit_memory_128bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "16>"),
        "norex_address_32bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "2>"),
        "norex_address_32bit_memory_224bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "28>"),
        "norex_address_32bit_memory_256bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "32>"),
        "norex_address_32bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "4>"),
        "norex_address_32bit_memory_48bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "6>"),
        "norex_address_32bit_memory_512bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "64>"),
        "norex_address_32bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "8>"),
        "norex_address_32bit_memory_752bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "94>"),
        "norex_address_32bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                  "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                  "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                  "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "1>"),
        "norex_address_32bit_memory_80bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "10>"),
        "norex_address_32bit_memory_864bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "108>"),
        "norex_address_32bit_memory_far_ptr_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                           "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                           "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                           "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-4isize as usize}>"),
        "norex_address_32bit_memory_far_ptr_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                           "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                           "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                           "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-6isize as usize}>"),
        "norex_address_32bit_memory_far_ptr_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                           "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                           "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,",
                                                                           "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-10isize as usize}>"),
        "norex_address_64bit_memory_0bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                  "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "0>"),
        "norex_address_64bit_memory_112bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "14>"),
        "norex_address_64bit_memory_128bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "16>"),
        "norex_address_64bit_memory_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "2>"),
        "norex_address_64bit_memory_224bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "28>"),
        "norex_address_64bit_memory_256bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "32>"),
        "norex_address_64bit_memory_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "4>"),
        "norex_address_64bit_memory_48bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "6>"),
        "norex_address_64bit_memory_512bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "64>"),
        "norex_address_64bit_memory_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "8>"),
        "norex_address_64bit_memory_752bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "94>"),
        "norex_address_64bit_memory_8bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                  "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                  "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                  "i32,",
                                                                  "1>"),
        "norex_address_64bit_memory_80bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                   "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                   "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                   "i32,",
                                                                   "10>"),
        "norex_address_64bit_memory_864bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                    "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                    "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                    "i32,",
                                                                    "108>"),
        "norex_address_64bit_memory_far_ptr_16bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                           "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-4isize as usize}>"),
        "norex_address_64bit_memory_far_ptr_32bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                           "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-6isize as usize}>"),
        "norex_address_64bit_memory_far_ptr_64bit" => concat! ("𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,",
                                                                           "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,",
                                                                           "𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,",
                                                                           "i32,",
                                                                           "{-10isize as usize}>"),
    }
});

pub(crate) struct 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
    pub(crate) 𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇: String,
    // These are indexed by 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞 as usize.
    pub(crate) 𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈: [𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞; 2],
}

pub(crate) struct 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
    𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌: String,
    𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌: String,
    pub(crate) 𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇: String,
}
