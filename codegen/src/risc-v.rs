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
    𝗋𝗏_𝗆𝗈𝖽𝖾: Option<𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞>,
    𝗋𝗏_𝖺𝖻𝗂: Option<𝐫𝐢𝐬𝐜_𝐯_𝐚𝐛𝐢>,
    𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌:
        &'ᵉˣᵗʳᵃ super::𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐞𝐱𝐭𝐫𝐚_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum 𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞 {
    𝔯𝔳32𝔢 = 0,
    𝔯𝔳32𝔦 = 1,
    𝔯𝔳64𝔦 = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum 𝐫𝐢𝐬𝐜_𝐯_𝐚𝐛𝐢 {
    𝔢𝔞𝔟𝔦 = 0,
    𝔲𝔞𝔟𝔦 = 1,
}

impl<'ᵉˣᵗʳᵃ> 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬<'ᵉˣᵗʳᵃ> {
    pub(crate) fn new(
        input: &mut impl Iterator<Item = TokenTree>,
        𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌: &'ᵉˣᵗʳᵃ super::𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐞𝐱𝐭𝐫𝐚_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬,
    ) -> Result<Self, &'static str> {
        let mut result: Self = 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬 {
            𝗋𝗏_𝗆𝗈𝖽𝖾: None,
            𝗋𝗏_𝖺𝖻𝗂: None,
            𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌,
        };
        let mut process = |identifier: &Ident| -> Result<(), &'static str> {
            match identifier.to_string().as_ref() {
                "𝔯𝔳32𝔢" => {
                    if result.𝗋𝗏_𝗆𝗈𝖽𝖾.is_some() {
                        return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated address size.\");");
                    } else {
                        result.𝗋𝗏_𝗆𝗈𝖽𝖾 = Some(𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞::𝔯𝔳32𝔢)
                    }
                }
                "𝔯𝔳32𝔦" => {
                    if result.𝗋𝗏_𝗆𝗈𝖽𝖾.is_some() {
                        return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated address size.\");");
                    } else {
                        result.𝗋𝗏_𝗆𝗈𝖽𝖾 = Some(𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞::𝔯𝔳32𝔦)
                    }
                }
                "𝔯𝔳64𝔦" => {
                    if result.𝗋𝗏_𝗆𝗈𝖽𝖾.is_some() {
                        return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated address size.\");");
                    } else {
                        result.𝗋𝗏_𝗆𝗈𝖽𝖾 = Some(𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞::𝔯𝔳64𝔦)
                    }
                }
                "𝔢𝔞𝔟𝔦" => {
                    if result.𝗋𝗏_𝖺𝖻𝗂.is_some() {
                        return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated address size.\");");
                    } else {
                        result.𝗋𝗏_𝖺𝖻𝗂 = Some(𝐫𝐢𝐬𝐜_𝐯_𝐚𝐛𝐢::𝔢𝔞𝔟𝔦)
                    }
                }
                "𝔲𝔞𝔟𝔦" => {
                    if result.𝗋𝗏_𝖺𝖻𝗂.is_some() {
                        return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — duplicated address size.\");");
                    } else {
                        result.𝗋𝗏_𝖺𝖻𝗂 = Some(𝐫𝐢𝐬𝐜_𝐯_𝐚𝐛𝐢::𝔲𝔞𝔟𝔦)
                    }
                }
                _ => return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — unknown token.\");"),
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
                            return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — attributes group includes unknown item.\");")
			};
                        process(identifier)?;
                    }
                }
                _ => return Err("compile_error!(\"𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! — attributes group includes unknown item.\");"),
            }
        }
        if let Some(ref 𝗋𝗏_𝗆𝗈𝖽𝖾) = result.𝗋𝗏_𝗆𝗈𝖽𝖾 {
            if result.𝗋𝗏_𝖺𝖻𝗂.is_none() {
                result.𝗋𝗏_𝖺𝖻𝗂 = Some(match 𝗋𝗏_𝗆𝗈𝖽𝖾 {
                    𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞::𝔯𝔳32𝔢 => 𝐫𝐢𝐬𝐜_𝐯_𝐚𝐛𝐢::𝔢𝔞𝔟𝔦,
                    _ => 𝐫𝐢𝐬𝐜_𝐯_𝐚𝐛𝐢::𝔲𝔞𝔟𝔦,
                });
            }
        }
        Ok(result)
    }
}

impl From<𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞> for 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞 {
    fn from(size: 𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞) -> 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞 {
        match size {
            𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞::𝔯𝔳32𝔢 => {
                𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔢
            }
            𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞::𝔯𝔳32𝔦 => {
                𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔦
            }
            𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞::𝔯𝔳64𝔦 => {
                𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳64𝔦
            }
        }
    }
}

pub(crate) fn filter_riscv_markers_iterable(
    output: &mut impl Extend<TokenTree>,
    output_extra: &mut Option<TokenStream>,
    input: &mut impl Iterator<Item = TokenTree>,
    attributes: 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬,
) {
    fn filter_riscv_markers_group(
        input: &mut Group, attributes: 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬
    ) -> TokenTree {
        let mut content = TokenStream::new();
        filter_riscv_markers_iterable(&mut content, &mut None, &mut input.stream().into_iter(), attributes);
        Group::new(input.delimiter(), content).into()
    }

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
            "𝔻𝕚𝕤𝕒𝕤𝕤𝕖𝕞𝕓𝕝𝕖𝕣𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟𝕤" => {
                let token_stream: TokenStream = 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈
                    [Into::<𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞>::into(attributes.𝗋𝗏_𝗆𝗈𝖽𝖾.unwrap()) as usize]
                    .𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖼𝗈𝗇𝗌𝗎𝗆𝖾_𝗋𝖾𝖽𝗂𝗋𝖾𝖼𝗍𝗈𝗋𝗌
                    .parse()
                    .unwrap();
                emit_tokens(output, output_extra, token_stream);
            }
            "𝔽𝕠𝕣𝕨𝕒𝕣𝕕𝕀𝕞𝕡𝕝𝕖𝕞𝕖𝕟𝕥𝕋𝕣𝕒𝕚𝕥𝕤" => {
                if attributes.𝗋𝗏_𝖺𝖻𝗂.unwrap() == 𝐫𝐢𝐬𝐜_𝐯_𝐚𝐛𝐢::𝔢𝔞𝔟𝔦 {
                    let token_stream: TokenStream = 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈
                        [Into::<𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞>::into(attributes.𝗋𝗏_𝗆𝗈𝖽𝖾.unwrap()) as usize]
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
                        (Some(true), attributes) => filter_riscv_markers_iterable(
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
                            filter_riscv_markers_iterable(
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
                            let filered_content = [filter_riscv_markers_group(&mut data_group_to_process, attributes)];
                            emit_tokens(output, output_extra, filered_content);
                        }
                    }
                }
                TokenTree::Group(mut data_group_to_process)
                    if matches!(data_group_to_process.delimiter(), Delimiter::Parenthesis) =>
                {
                    let unwrapped_token_str = unwrapped_token.to_string();
                    if unwrapped_token_str == "𝔻𝕖𝕔𝕠𝕕𝕖𝟛𝟚𝕓𝕚𝕥𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟"
                    {
                        let mut group_iter = data_group_to_process.stream().into_iter();
                        let mut get_chunk = move || {
                            let mut chunk = TokenStream::new();
                            loop {
                                match group_iter.next() {
                                    Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => break,
                                    None => break,
                                    Some(token) => chunk.extend([token]),
                                }
                            }
                            chunk
                        };
                        let token_stream: TokenStream;
                        let ref chunks @ (
                            ref _opcode,
                            ref _rm,
                            ref _rd_bits,
                            ref _compressed_instruction_step,
                            ref _instruction_bits,
                        ) = (
                            get_chunk(),
                            {
                                let Some(TokenTree::Literal(branch)) = get_chunk().into_iter().next() else { panic!("Couldn't find branch value") };
                                token_stream = 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬.𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈
                                    [Into::<𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞>::into(
                                        attributes.𝗋𝗏_𝗆𝗈𝖽𝖾.unwrap(),
                                    ) as usize]
                                    .𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌
                                    [(branch.to_string().as_bytes()[0] - b'0') as usize]
                                    .parse()
                                    .unwrap();
                                get_chunk()
                            },
                            get_chunk(),
                            get_chunk(),
                            get_chunk(),
                        );
                        fn copy_tokens(
                            output: &mut impl Extend<TokenTree>,
                            output_extra: &mut Option<TokenStream>,
                            token_stream: TokenStream,
                            chunks @ (𝗈𝗉𝖼𝗈𝖽𝖾, rm, rd_bits, compressed_instruction_step, instruction_bits): &(
                                TokenStream,
                                TokenStream,
                                TokenStream,
                                TokenStream,
                                TokenStream,
                            ),
                        ) {
                            for token in token_stream {
                                match token {
                                    TokenTree::Ident(ident) if matches!(ident.to_string().as_ref(), "opcode") => {
                                        emit_tokens(output, output_extra, 𝗈𝗉𝖼𝗈𝖽𝖾.clone());
                                    }
                                    TokenTree::Ident(ident) if matches!(ident.to_string().as_ref(), "rm") => {
                                        emit_tokens(output, output_extra, rm.clone());
                                    }
                                    TokenTree::Ident(ident) if matches!(ident.to_string().as_ref(), "rd_bits") => {
                                        emit_tokens(output, output_extra, rd_bits.clone());
                                    }
                                    TokenTree::Ident(ident)
                                        if matches!(ident.to_string().as_ref(), "compressed_instruction_step") =>
                                    {
                                        emit_tokens(output, output_extra, compressed_instruction_step.clone());
                                    }
                                    TokenTree::Ident(ident) if matches!(ident.to_string().as_ref(), "instruction_bits") => {
                                        emit_tokens(output, output_extra, instruction_bits.clone());
                                    }
                                    TokenTree::Group(input) => {
                                        let mut content = TokenStream::new();
                                        copy_tokens(&mut content, &mut None, input.stream(), chunks);
                                        emit_tokens(output, output_extra, [Group::new(input.delimiter(), content).into()]);
                                    }
                                    _ => {
                                        emit_tokens(output, output_extra, [token]);
                                    }
                                }
                            }
                        }
                        copy_tokens(output, output_extra, token_stream, chunks);
                    } else {
                        emit_or_expand_token(output, output_extra, unwrapped_token, attributes);
                        let filered_content = [filter_riscv_markers_group(&mut data_group_to_process, attributes)];
                        emit_tokens(output, output_extra, filered_content);
                    }
                }
                TokenTree::Group(mut data_group_to_process) => {
                    emit_or_expand_token(output, output_extra, unwrapped_token, attributes);
                    let filered_content = [filter_riscv_markers_group(&mut data_group_to_process, attributes)];
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
            let filered_content = [filter_riscv_markers_group(&mut data_group_to_process, attributes)];
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
            [Into::<𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞>::into(attributes.𝗋𝗏_𝗆𝗈𝖽𝖾.unwrap()) as usize]
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

fn marker_is_compatible<'ᵉˣᵗʳᵃ>(
    marker: &str,
    attributes: 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬<'ᵉˣᵗʳᵃ>,
) -> (Option<bool>, 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬<'ᵉˣᵗʳᵃ>) {
    match marker {
        "Ξ𝔯𝔳32𝔢" => (Some(attributes.𝗋𝗏_𝗆𝗈𝖽𝖾.unwrap() == 𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞::𝔯𝔳32𝔢), attributes),
        "Χ𝔯𝔳32𝔢" => (Some(attributes.𝗋𝗏_𝗆𝗈𝖽𝖾.unwrap() != 𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞::𝔯𝔳32𝔢), attributes),
        "Ξ𝔯𝔳32" => (Some(attributes.𝗋𝗏_𝗆𝗈𝖽𝖾.unwrap() != 𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞::𝔯𝔳64𝔦), attributes),
        "Ξ𝔯𝔳64" => (Some(attributes.𝗋𝗏_𝗆𝗈𝖽𝖾.unwrap() == 𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞::𝔯𝔳64𝔦), attributes),
        "Ξ𝔢𝔞𝔟𝔦" => (Some(attributes.𝗋𝗏_𝖺𝖻𝗂.unwrap() == 𝐫𝐢𝐬𝐜_𝐯_𝐚𝐛𝐢::𝔢𝔞𝔟𝔦), attributes),
        "Ξ𝔲𝔞𝔟𝔦" => (Some(attributes.𝗋𝗏_𝖺𝖻𝗂.unwrap() == 𝐫𝐢𝐬𝐜_𝐯_𝐚𝐛𝐢::𝔲𝔞𝔟𝔦), attributes),
        _ => (None, attributes),
    }
}

#[tokio::main]
async fn get_instrution_info() -> 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
    let mut instruction_trait = HashSet::new();
    let mut instruction_traits = HashSet::new();
    let mut kind_specific_traits = [HashSet::new(), HashSet::new(), HashSet::new()];

    let mut riscv_assembler_instructions = Vec::new();
    let mut assembler_instructions = [Vec::new(), Vec::new(), Vec::new()];
    const 𝔫𝔬𝔱_𝔦𝔪𝔭𝔩𝔢𝔪𝔢𝔫𝔱𝔢𝔡_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫: BTreeMap<
        (u32, u32),
        𝐝𝐞𝐜𝐨𝐝𝐞𝐝_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨,
    > = BTreeMap::new();
    const 𝔫𝔬𝔱_𝔦𝔪𝔭𝔩𝔢𝔪𝔢𝔫𝔱𝔢𝔡_𝔪𝔞𝔧𝔬𝔯_𝔬𝔭𝔠𝔬𝔡𝔢: [BTreeMap<
        (u32, u32),
        𝐝𝐞𝐜𝐨𝐝𝐞𝐝_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨,
    >; 32] = [𝔫𝔬𝔱_𝔦𝔪𝔭𝔩𝔢𝔪𝔢𝔫𝔱𝔢𝔡_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫; 32];
    const 𝔫𝔬𝔱_𝔦𝔪𝔭𝔩𝔢𝔪𝔢𝔫𝔱𝔢𝔡_𝔣𝔲𝔫𝔠3_𝔬𝔭𝔠𝔬𝔡𝔢: [[BTreeMap<
        (u32, u32),
        𝐝𝐞𝐜𝐨𝐝𝐞𝐝_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨,
    >; 32]; 8] = [𝔫𝔬𝔱_𝔦𝔪𝔭𝔩𝔢𝔪𝔢𝔫𝔱𝔢𝔡_𝔪𝔞𝔧𝔬𝔯_𝔬𝔭𝔠𝔬𝔡𝔢; 8];
    #[allow(clippy::type_complexity)]
    const 𝔫𝔬𝔱_𝔦𝔪𝔭𝔩𝔢𝔪𝔢𝔫𝔱𝔢𝔡_𝔪𝔞𝔭: [[[BTreeMap<
        (u32, u32),
        𝐝𝐞𝐜𝐨𝐝𝐞𝐝_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨,
    >; 32]; 8]; 3] = [𝔫𝔬𝔱_𝔦𝔪𝔭𝔩𝔢𝔪𝔢𝔫𝔱𝔢𝔡_𝔣𝔲𝔫𝔠3_𝔬𝔭𝔠𝔬𝔡𝔢; 3];
    let mut disassembler_instructions_map = 𝔫𝔬𝔱_𝔦𝔪𝔭𝔩𝔢𝔪𝔢𝔫𝔱𝔢𝔡_𝔪𝔞𝔭;
    const 𝔢𝔪𝔭𝔱𝔶_𝔡𝔢𝔠𝔬𝔡𝔢_32𝔟𝔦𝔱_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰: Vec<
        String,
    > = Vec::new();
    const 𝔢𝔪𝔭𝔱𝔶_𝔡𝔢𝔠𝔬𝔡𝔢_32𝔟𝔦𝔱_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔳𝔢𝔠𝔱𝔬𝔯𝔰: [Vec<String>; 8] = [𝔢𝔪𝔭𝔱𝔶_𝔡𝔢𝔠𝔬𝔡𝔢_32𝔟𝔦𝔱_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰; 8];
    let mut 𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌 = [𝔢𝔪𝔭𝔱𝔶_𝔡𝔢𝔠𝔬𝔡𝔢_32𝔟𝔦𝔱_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔳𝔢𝔠𝔱𝔬𝔯𝔰; 3];
    let mut disassembler_instructions = [Vec::new(), Vec::new(), Vec::new()];
    let mut instructions_enum_declararion = [String::new(), String::new(), String::new()];
    let mut leaf_assembler_instructions = [BTreeMap::new(), BTreeMap::new(), BTreeMap::new()];
    let mut leaf_disassembler_instructions = [BTreeMap::new(), BTreeMap::new(), BTreeMap::new()];
    let mut 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌 = [BTreeMap::new(), BTreeMap::new(), BTreeMap::new()];
    let mut 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝖾𝗇𝗎𝗆𝗌 = [BTreeMap::new(), BTreeMap::new(), BTreeMap::new()];
    let mut 𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌 = [Vec::new(), Vec::new(), Vec::new()];

    let mut connection = get_database_connection().await;

    // We need that trick because of SQLx design: https://github.com/launchbadge/sqlx/issues/1594#issuecomment-1100763779
    // Keep query string alive in our function — that way it wouldn't become stale while we are processing instructions list.
    let mut query = String::new();
    for assembler_kind in [𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔢, 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔦, 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳64𝔦]
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

                let 𝖿𝗇_𝗇𝖺𝗆𝖾 = instruction.𝖿𝗇_𝗇𝖺𝗆𝖾.replace('.', "_");
                let 𝖿𝗇_𝗇𝖺𝗆𝖾 = 𝖿𝗇_𝗇𝖺𝗆𝖾.as_str();

                let rv64_long_shift = arguments_count == 3 && arguments_sql_types[2] == "<:imm";

                let 𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 = instruction.𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.as_str();
                let 𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾 = instruction.𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.as_str();
                let 𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍_𝗇𝖺𝗆𝖾 = instruction.𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍_𝗇𝖺𝗆𝖾.as_str();

                if instruction_trait.insert(𝖿𝗇_𝗇𝖺𝗆𝖾.to_owned()) {
                    let instructions_trait = format!("pub trait {𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>{{type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;fn {𝖿𝗇_𝗇𝖺𝗆𝖾}(&mut self,arguments:𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮)->Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;}}");
                    let instruction_trait = format!("pub trait {𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>{{type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;fn {𝖿𝗇_𝗇𝖺𝗆𝖾}_implementation(&mut self,arguments:𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮)->Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;}}");
                    riscv_assembler_instructions.push(instructions_trait);
                    riscv_assembler_instructions.push(instruction_trait);
                }

                if instruction_traits.insert((𝖿𝗇_𝗇𝖺𝗆𝖾.to_owned(), arguments_count)) {
                    let mut parameter_types_list = Vec::new();
                    let mut argument_types = Vec::new();
                    let mut parameters_type_list = Vec::new();
                    let mut parameters_list = Vec::new();
                    let mut parameters_convert_into = Vec::new();
                    for (index, argument) in 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌.iter().enumerate() {
                        let argument_trait = argument.𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍;
                        parameter_types_list.push(format!("𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻{index}_𝓽𝔂𝓹𝓮:{argument_trait}<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,"));
                        argument_types.push(format!("<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻{index}_𝓽𝔂𝓹𝓮 as {argument_trait}<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭"));
                        parameters_type_list.push(format!("𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻{index}_𝓽𝔂𝓹𝓮"));
                        parameters_list.push(format!("parameter{index}"));
                        parameters_convert_into.push(format!(
                            "core::convert::Into::<<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻{index}_𝓽𝔂𝓹𝓮 as {argument_trait}<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>::into(parameter{index})"
                        ));
                    }
                    let parameter_types_list = parameter_types_list.concat();
                    let argument_types = argument_types.join(",");
                    let parameters_type_list = parameters_type_list.join(",");
                    let parameters_list = parameters_list.join(",");
                    let parameters_convert_into = parameters_convert_into.join(",");
                    let impl_instruction = format!("impl<{parameter_types_list}𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮:{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<({argument_types}{arguments_comma})>>{𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}<({parameters_type_list}{arguments_comma})>for 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮{{#[allow(clippy::type_complexity)]type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞=<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as {𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<({argument_types}{arguments_comma})>>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;#[allow(clippy::type_complexity)]type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞=<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as {𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<({argument_types}{arguments_comma})>>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;#[inline(always)]fn {𝖿𝗇_𝗇𝖺𝗆𝖾}(&mut self,({parameters_list}{arguments_comma}):({parameters_type_list}{arguments_comma}))->Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>{{self.{𝖿𝗇_𝗇𝖺𝗆𝖾}_implementation(({parameters_convert_into}{arguments_comma}))}}}}");
                    riscv_assembler_instructions.push(impl_instruction);
                }

                if kind_specific_traits[assembler_kind as usize].insert(instruction.𝖿𝗇_𝗇𝖺𝗆𝖾.to_owned()) {
                    let instruction_trait = format!("pub trait {𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>{{type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;fn {𝖿𝗇_𝗇𝖺𝗆𝖾}_with(&mut self,parameters:𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮)->Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;}}");
                    let instruction_impl = format!("impl Æ 𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>𝗿𝗶𝘀𝗰_𝘃::{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮> for æ where Self:{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>{{type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞=<Self as {𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞=<Self as {𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;#[inline(always)]fn {𝖿𝗇_𝗇𝖺𝗆𝖾}_implementation(&mut self,arguments:𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮)->Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>{{self.{𝖿𝗇_𝗇𝖺𝗆𝖾}_with(arguments)}}}}");
                    assembler_instructions[assembler_kind as usize].push(instruction_trait);
                    𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌[assembler_kind as usize].push(instruction_impl);
                }

                let mut arguments_type = Vec::new();
                let mut arguments_trait_type = Vec::new();
                let mut parameters_list = Vec::new();
                let mut enums_vector_list = Vec::new();
                for (index, argument) in 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌.iter().enumerate() {
                    arguments_type.push(argument.𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾);
                    arguments_trait_type.push(argument.𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍_𝗍𝗒𝗉𝖾);
                    parameters_list.push(format!("parameter{index}"));
                    enums_vector_list.push(format!(
                        "𝐨𝐩𝐞𝐫𝐚𝐧𝐝::<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::{}(parameter{})",
                        argument.𝗋𝗎𝗌𝗍_𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍, index
                    ));
                }
                let arguments_type = format!("({}{arguments_comma})", arguments_type.join(","));
                let arguments_trait_type = format!("({}{arguments_comma})", arguments_trait_type.join(","));
                let parameters_list = format!("({}{arguments_comma})", parameters_list.join(","));
                let enums_vector_list = format!("[{}].as_slice()", enums_vector_list.join(","));

                let 𝗈𝗉𝖼𝗈𝖽𝖾 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾;
                let 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄 = instruction.𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄;

                let arguments_sql_sources = 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌
                    .iter()
                    .map(|argument| argument.𝗌𝗊𝗅_𝗌𝗈𝗎𝗋𝖼𝖾.as_str())
                    .collect::<Vec<_>>();
                let instruction_emit = match arguments_sql_sources[..] {
                    [] => format!("self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x})"),
                    ["fencep", "fences"] => format!("let parameter0:u32=parameter0.into();let parameter1:u32=parameter1.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|parameter0<<24|parameter1<<20)"),
                    ["p:imm(rs1)"] => format!("let base:u32=parameter0.𝖻𝖺𝗌𝖾.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|base<<15|parameter0.𝖽𝗂𝗌𝗉.encoded() as u32)"),
                    ["rd", "0(rs1)"] => format!("let parameter0:u32=parameter0.into();let base:u32=parameter1.𝖻𝖺𝗌𝖾.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|parameter0<<7|base<<15)"),
                    ["rd", "csr", "rs1"] => format!("let parameter0:u32=parameter0.into();let parameter1:u32=parameter1.into();let parameter2:u32=parameter2.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|parameter0<<7|parameter1<<20|parameter2<<15)"),
                    ["rd", "csr", "c:imm"] => format!("let parameter0:u32=parameter0.into();let parameter1:u32=parameter1.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|parameter0<<7|parameter1<<20|parameter2.encoded() as u32)"),
                    ["rd", "i:imm(rs1)"] => format!("let parameter0:u32=parameter0.into();let base:u32=parameter1.𝖻𝖺𝗌𝖾.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|parameter0<<7|base<<15|parameter1.𝖽𝗂𝗌𝗉.encoded() as u32)"),
                    ["rd", "j:imm"] => format!("let parameter0:u32=parameter0.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|parameter0<<7|parameter1.encoded() as u32)"),
                    ["rd", "rs1", "<:imm" | ">:imm" | "i:imm"] => format!("let parameter0:u32=parameter0.into();let parameter1:u32=parameter1.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|parameter0<<7|parameter1<<15|parameter2.encoded() as u32)"),
                    ["rd", "rs1", "rm"] => format!("let parameter0:u32=parameter0.into();let parameter1:u32=parameter1.into();let parameter2:u32=parameter2.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|parameter0<<7|parameter1<<15|parameter2<<12)"),
                    ["rd", "rs1", "rs2"] => format!("let parameter0:u32=parameter0.into();let parameter1:u32=parameter1.into();let parameter2:u32=parameter2.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|parameter0<<7|parameter1<<15|parameter2<<20)"),
                    ["rd", "rs1", "rs2", "rm"] => format!("let parameter0:u32=parameter0.into();let parameter1:u32=parameter1.into();let parameter2:u32=parameter2.into();let parameter3:u32=parameter3.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|parameter0<<7|parameter1<<15|parameter2<<20|parameter3<<12)"),
                    ["rd", "rs1", "rs2", "rs3", "rm"] => format!("let parameter0:u32=parameter0.into();let parameter1:u32=parameter1.into();let parameter2:u32=parameter2.into();let parameter3:u32=parameter3.into();let parameter4:u32=parameter4.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|parameter0<<7|parameter1<<15|parameter2<<20|parameter3<<27|parameter4<<12)"),
                    ["rd", "rs1"] => format!("let parameter0:u32=parameter0.into();let parameter1:u32=parameter1.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|parameter0<<7|parameter1<<15)"),
                    ["rd", "rs2", "0(rs1)"] =>  format!("let parameter0:u32=parameter0.into();let parameter1:u32=parameter1.into();let base:u32=parameter2.𝖻𝖺𝗌𝖾.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|parameter0<<7|parameter1<<20|base<<15)"),
                    ["rd", "u:imm"] => format!("let parameter0:u32=parameter0.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|parameter0<<7|parameter1.encoded() as u32)"),
                    ["rs1", "rs2", "b:imm"] => format!("let parameter0:u32=parameter0.into();let parameter1:u32=parameter1.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|parameter0<<15|parameter1<<20|parameter2.encoded() as u32)"),
                    ["rs2", "s:imm(rs1)"] => format!("let parameter0:u32=parameter0.into();let base:u32=parameter1.𝖻𝖺𝗌𝖾.into();self.emit_u32(0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}|parameter0<<20|base<<15|parameter1.𝖽𝗂𝗌𝗉.encoded() as u32)"),
                    _ => panic!("Unsupported combination of instruction arguments {arguments_sql_sources:?}"),
                };

                if 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄 & 0b1111111 != 0b1111111 || 𝗈𝗉𝖼𝗈𝖽𝖾 & 0b11 != 0b11 {
                    panic!("Unsupported instruction opcode 0x{𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄:08x} 0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}");
                }
                let arguments_sql_operands = 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌
                    .iter()
                    .map(|argument| argument.𝗌𝗊𝗅_𝗈𝗉𝖾𝗋𝖺𝗇𝖽.as_str())
                    .collect::<Vec<_>>();
                let instruction_info =
                    𝐝𝐞𝐜𝐨𝐝𝐞𝐝_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨 {
                        𝖼𝗌𝗋_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: arguments_sql_operands.contains(&"csr"),
                        𝖿𝖽_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: arguments_sql_operands.contains(&"fd"),
                        𝗋𝖽_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: arguments_sql_operands.contains(&"rd"),
                        𝖿𝗌𝟣_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: arguments_sql_operands.contains(&"fs1"),
                        𝗋𝗌𝟣_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: arguments_sql_operands
                            .iter()
                            .any(|&op| op.contains("rs1")),
                        𝖿𝗌𝟤_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: arguments_sql_operands.contains(&"fs2"),
                        𝗋𝗌𝟤_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: arguments_sql_operands.contains(&"rs2"),
                        𝖿𝗌𝟥_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: arguments_sql_operands.contains(&"fs3"),
                        𝗋𝗆_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: arguments_sql_operands.contains(&"rm"),
                        𝗂𝗆𝗆𝖾𝖽𝗂𝖺𝗍𝖾_𝗇𝖾𝖾𝖽𝖾𝖽: if arguments_sql_sources.contains(&"<:imm")
                        {
                            Some(𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞_𝐭𝐲𝐩𝐞::𝔰𝔥𝔦𝔣𝔱_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢)
                        } else if arguments_sql_sources.contains(&">:imm") {
                            if assembler_kind != 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳64𝔦 {
                                Some(𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞_𝐭𝐲𝐩𝐞::𝔰𝔥𝔦𝔣𝔱_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢)
                            } else {
                                Some(𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞_𝐭𝐲𝐩𝐞::𝔴𝔬𝔯𝔡_𝔰𝔥𝔦𝔣𝔱_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢)
                            }
                        } else if arguments_sql_sources.contains(&"b:imm") {
                            Some(𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞_𝐭𝐲𝐩𝐞::𝔟𝔯𝔞𝔫𝔠𝔥_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢)
                        } else if arguments_sql_sources.contains(&"c:imm") {
                            Some(𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞_𝐭𝐲𝐩𝐞::𝔠𝔰𝔯_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢)
                        } else if arguments_sql_sources.contains(&"j:imm") {
                            Some(𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞_𝐭𝐲𝐩𝐞::𝔧𝔲𝔪𝔭_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢)
                        } else if arguments_sql_sources.contains(&"u:imm") {
                            Some(𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞_𝐭𝐲𝐩𝐞::𝔲𝔭𝔭𝔢𝔯_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢)
                        } else if arguments_sql_sources.iter().any(|&op| op.contains("i:imm")) {
                            Some(𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞_𝐭𝐲𝐩𝐞::𝔰𝔬𝔲𝔯𝔠𝔢_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢)
                        } else if arguments_sql_sources.iter().any(|&op| op.contains("p:imm")) {
                            Some(𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞_𝐭𝐲𝐩𝐞::𝔭𝔯𝔢𝔣𝔢𝔱𝔠𝔥_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢)
                        } else if arguments_sql_sources.iter().any(|&op| op.contains("s:imm")) {
                            Some(𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞_𝐭𝐲𝐩𝐞::𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢)
                        } else {
                            None
                        },
                        𝖾𝗆𝗂𝗍_𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇: match arguments_sql_operands[..] {
                            [] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}(())"),
                            ["fd", "fs1", "fs2"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((fd,fs1,fs2))"),
                            ["fd", "fs1", "fs2", "rm"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((fd,fs1,fs2,rm))"),
                            ["fd", "fs1", "fs2", "fs3", "rm"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((fd,fs1,fs2,fs3,rm))"),
                            ["fd", "fs1", "rm"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((fd,fs1,rm))"),
                            ["fd", "rs1"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((fd,rs1))"),
                            ["fd", "rs1", "rm"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((fd,rs1,rm))"),
                            ["fd", "i:imm(rs1)"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((fd,𝒂𝒅𝒅𝒓𝒆𝒔𝒔{{𝖻𝖺𝗌𝖾:rs1,𝖽𝗂𝗌𝗉:imm}}))"),
                            ["fencep", "fences"] => format!(
                                "let Ok(fencep)=((instruction_bits>>24)&0b1111).try_into()else{{break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ}};let Ok(fences)=((instruction_bits>>20)&0b1111).try_into()else{{break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ}};return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((fencep,fences))"
                            ),
                            ["fs2", "s:imm(rs1)"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((fs2,𝒂𝒅𝒅𝒓𝒆𝒔𝒔{{𝖻𝖺𝗌𝖾:rs1,𝖽𝗂𝗌𝗉:imm}}))"),
                            ["p:imm(rs1)"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((𝒂𝒅𝒅𝒓𝒆𝒔𝒔{{𝖻𝖺𝗌𝖾:rs1,𝖽𝗂𝗌𝗉:imm}},))"),
                            ["rd", "0(rs1)"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((rd,𝒂𝒅𝒅𝒓𝒆𝒔𝒔{{𝖻𝖺𝗌𝖾:rs1,𝖽𝗂𝗌𝗉:𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞{{}}}}))"),
                            ["rd", "csr", "rs1"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((rd,csr,rs1))"),
                            ["rd", "csr", "c:imm"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((rd,csr,imm))"),
                            ["rd", "fs1", "rm"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((rd,fs1,rm))"),
                            ["rd", "fs1", "fs2"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((rd,fs1,fs2))"),
                            ["rd", "i:imm(rs1)"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((rd,𝒂𝒅𝒅𝒓𝒆𝒔𝒔{{𝖻𝖺𝗌𝖾:rs1,𝖽𝗂𝗌𝗉:imm}}))"),
                            ["rd", "j:imm"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((rd,imm))"),
                            ["rd", "fs1"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((rd,fs1))"),
                            ["rd", "rs1", "<:imm" | ">:imm" | "i:imm"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((rd,rs1,imm))"),
                            ["rd", "rs1", "rs2"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((rd,rs1,rs2))"),
                            ["rd", "rs2", "0(rs1)"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((rd,rs2,𝒂𝒅𝒅𝒓𝒆𝒔𝒔{{𝖻𝖺𝗌𝖾:rs1,𝖽𝗂𝗌𝗉:𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞{{}}}}))"),
                            ["rd", "u:imm"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((rd,imm))"),
                            ["rs1", "rs2", "b:imm"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((rs1,rs2,imm))"),
                            ["rs2", "s:imm(rs1)"] => format!("return self.{𝖿𝗇_𝗇𝖺𝗆𝖾}((rs2,𝒂𝒅𝒅𝒓𝒆𝒔𝒔{{𝖻𝖺𝗌𝖾:rs1,𝖽𝗂𝗌𝗉:imm}}))"),
                            _ => panic!("Unsupported combination of instruction arguments {arguments_sql_operands:?}"),
                        },
                    };
                match (𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄 >> 12) & 0b111 {
                    0b000 => {
                        for func3 in 0..=7 {
                            if disassembler_instructions_map[assembler_kind as usize][func3][((𝗈𝗉𝖼𝗈𝖽𝖾 >> 2) & 0b11111) as usize]
                                .insert((𝗈𝗉𝖼𝗈𝖽𝖾, !𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄), instruction_info.clone())
                                .is_some()
                            {
                                panic!("Duplicated opcode 0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x} 0x{𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄:08x}")
                            }
                        }
                    }
                    0b111 => {
                        if disassembler_instructions_map[assembler_kind as usize][((𝗈𝗉𝖼𝗈𝖽𝖾 >> 12) & 0b111) as usize]
                            [((𝗈𝗉𝖼𝗈𝖽𝖾 >> 2) & 0b11111) as usize]
                            .insert((𝗈𝗉𝖼𝗈𝖽𝖾, !𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄), instruction_info)
                            .is_some()
                        {
                            panic!("Duplicated opcode 0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x} 0x{𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄:08x}")
                        }
                    }
                    _ => panic!("Unsupported instruction opcode mask 0x{𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄:08x}"),
                }

                let instruction_info =
                    format!("impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮:𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}_𝒘𝒊𝒕𝒉<{arguments_type}>for 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮{{type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞=<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as 𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞=<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as 𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;#[inline(always)]fn {𝖿𝗇_𝗇𝖺𝗆𝖾}_with(&mut self,{parameters_list}:{arguments_type})->Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>{{{instruction_emit}}}}}");
                if rv64_long_shift {
                    let assembler_instructions = leaf_assembler_instructions[assembler_kind as usize]
                        .entry((
                            𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.to_owned(),
                            arguments_type.replace("𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞", "𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞"),
                        ))
                        .or_insert_with(Vec::new);
                    assembler_instructions.push(
                        instruction_info
                            .clone()
                            .replace("𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞", "𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞"),
                    );
                }
                let assembler_instructions = leaf_assembler_instructions[assembler_kind as usize]
                    .entry((𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.to_owned(), arguments_type.to_owned()))
                    .or_insert_with(Vec::new);
                assembler_instructions.push(instruction_info);

                let instruction_info: String = format!("𝗿𝗶𝘀𝗰_𝘃::{𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}<{arguments_trait_type}>");
                let 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌 = 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌[assembler_kind as usize]
                    .entry((𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.to_owned(), 𝖿𝗇_𝗇𝖺𝗆𝖾.to_owned()))
                    .or_insert_with(BTreeMap::new);
                if rv64_long_shift {
                    let 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌 = 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌
                        .entry(arguments_trait_type.replace("𝐬𝐡𝐢𝐟𝐭_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞", "𝐰𝐨𝐫𝐝_𝐬𝐡𝐢𝐟𝐭_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞"))
                        .or_insert_with(Vec::new);
                    𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌.push(instruction_info.replace("𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞", "𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞"));
                }
                let 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌 = 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌.entry(arguments_trait_type.to_owned()).or_insert_with(Vec::new);
                𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌.push(instruction_info);

                let 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝖾𝗇𝗎𝗆𝗌 = 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝖾𝗇𝗎𝗆𝗌[assembler_kind as usize]
                    .entry(𝖿𝗇_𝗇𝖺𝗆𝖾.to_owned())
                    .or_insert_with(Vec::new);
                𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝖾𝗇𝗎𝗆𝗌.push(𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍_𝗇𝖺𝗆𝖾.to_owned());

                let arguments_trait_type =
                    arguments_trait_type.replace("Self::", "<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::");
                let instruction_info =
                    format!("#[inline(always)]fn {𝖿𝗇_𝗇𝖺𝗆𝖾}(self,{parameters_list}:{arguments_trait_type})->Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>{{self.instruction(𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝::{𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍_𝗇𝖺𝗆𝖾},{enums_vector_list})}}");
                let disassembler_instructions = leaf_disassembler_instructions[assembler_kind as usize]
                    .entry((𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾.to_owned(), arguments_type.to_owned()))
                    .or_insert_with(Vec::new);
                disassembler_instructions.push(instruction_info);
            }
        }
        for func3 in 0b0000..0b1000 {
            let 𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌 =
                &mut 𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌[assembler_kind as usize][func3];
            𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈˡᵒⁿᵍ:{match opcode{".to_owned());
            for 𝗈𝗉𝖼𝗈𝖽𝖾 in 0b000000..0b100000 {
                let disassembler_instructions_map =
                    &disassembler_instructions_map[assembler_kind as usize][func3][𝗈𝗉𝖼𝗈𝖽𝖾];

                let mut 𝖼𝗌𝗋_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 = false;
                let mut 𝖿𝖽_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 = false;
                let mut 𝗋𝖽_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 = false;
                let mut 𝖿𝗌𝟣_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 = false;
                let mut 𝗋𝗌𝟣_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 = false;
                let mut 𝖿𝗌𝟤_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 = false;
                let mut 𝗋𝗌𝟤_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 = false;
                let mut 𝖿𝗌𝟥_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 = false;
                let mut 𝗂𝗆𝗆𝖾𝖽𝗂𝖺𝗍𝖾_𝗇𝖾𝖾𝖽𝖾𝖽 = None;

                let mut candidate_instructions = 0;
                let mut collected_opcode_and_mask = None;
                for ((𝗈𝗉𝖼𝗈𝖽𝖾, 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄), instruction) in disassembler_instructions_map.iter()
                {
                    let 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄 = (!𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄) & 0xffff8f80; // Ignore func3 and main_opcode parts.
                                                                   // No operands instructions can be added to any others.
                    if 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄 == 0xffff8f80 {
                        candidate_instructions += 1;
                        continue;
                    }
                    // Values 5 and 6 are reserved for rm field, instruction would be handled by unimplemented_32bit_instruction.
                    if instruction.𝗋𝗆_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 && (func3 == 5 || func3 == 6) {
                        continue;
                    }

                    𝖼𝗌𝗋_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 |= instruction.𝖼𝗌𝗋_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽;
                    𝖿𝖽_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 |= instruction.𝖿𝖽_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽;
                    𝗋𝖽_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 |= instruction.𝗋𝖽_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽;
                    𝖿𝗌𝟣_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 |= instruction.𝖿𝗌𝟣_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽;
                    𝗋𝗌𝟣_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 |= instruction.𝗋𝗌𝟣_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽;
                    𝖿𝗌𝟤_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 |= instruction.𝖿𝗌𝟤_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽;
                    𝗋𝗌𝟤_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 |= instruction.𝗋𝗌𝟤_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽;
                    𝖿𝗌𝟥_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 |= instruction.𝖿𝗌𝟥_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽;

                    if let Some(immediate) = instruction.𝗂𝗆𝗆𝖾𝖽𝗂𝖺𝗍𝖾_𝗇𝖾𝖾𝖽𝖾𝖽 {
                        if let Some(𝗂𝗆𝗆𝖾𝖽𝗂𝖺𝗍𝖾_𝗇𝖾𝖾𝖽𝖾𝖽) = 𝗂𝗆𝗆𝖾𝖽𝗂𝖺𝗍𝖾_𝗇𝖾𝖾𝖽𝖾𝖽
                        {
                            // Prefetch is a hint and it's injected in what otherwise would have been 𝔬𝔯𝔦 instruction.
                            // That means that we would decode immediate twice: first as 𝔬𝔯𝔦 operand and then, if 𝔯𝔡 is zero,
                            // also as 𝔭𝔯𝔢𝔣𝔢𝔱𝔠𝔥 operand.
                            //
                            // Other insrtructions always use the same immediate type when major opcode is fixed.
                            if let (𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞_𝐭𝐲𝐩𝐞::𝔭𝔯𝔢𝔣𝔢𝔱𝔠𝔥_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢, 𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞_𝐭𝐲𝐩𝐞::𝔰𝔬𝔲𝔯𝔠𝔢_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢) =
                                (𝗂𝗆𝗆𝖾𝖽𝗂𝖺𝗍𝖾_𝗇𝖾𝖾𝖽𝖾𝖽, immediate)
                            {
                                collected_opcode_and_mask = Some((*𝗈𝗉𝖼𝗈𝖽𝖾, 0x01f00f80));
                                candidate_instructions += 1;
                                continue;
                            } else if 𝗂𝗆𝗆𝖾𝖽𝗂𝖺𝗍𝖾_𝗇𝖾𝖾𝖽𝖾𝖽 != immediate {
                                panic!("Different immediates: {𝗂𝗆𝗆𝖾𝖽𝗂𝖺𝗍𝖾_𝗇𝖾𝖾𝖽𝖾𝖽:?} {immediate:?}");
                            }
                        } else {
                            𝗂𝗆𝗆𝖾𝖽𝗂𝖺𝗍𝖾_𝗇𝖾𝖾𝖽𝖾𝖽 = Some(immediate)
                        }
                    }
                    if let Some((collected_opcode, collected_opcode_mask)) = collected_opcode_and_mask {
                        // That's a corner case where we first need to look on top 7 bits and then,
                        // sometimes, may additionally look on rs2, which is used as opcode extension.
                        if let (0xfff00000, 0xfe000000) = (collected_opcode_mask, 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄) {
                            collected_opcode_and_mask = Some((*𝗈𝗉𝖼𝗈𝖽𝖾, 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄));
                            candidate_instructions += 1;
                            continue;
                        }
                        if let (0xfe000000, 0xfff00000) = (collected_opcode_mask, 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄) {
                            candidate_instructions += 1;
                            continue;
                        }
                        if 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄 != collected_opcode_mask {
                            panic!("Incompatible opcodes: 0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x} 0x{𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄:08x} 0x{collected_opcode:08x} 0x{collected_opcode_mask:08x}");
                        }
                    } else {
                        collected_opcode_and_mask = Some((*𝗈𝗉𝖼𝗈𝖽𝖾, 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄));
                        candidate_instructions += 1;
                    }
                }

                if candidate_instructions > 0 {
                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push(format!("{𝗈𝗉𝖼𝗈𝖽𝖾}=>{{").to_owned());
                }

                if 𝖼𝗌𝗋_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 {
                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push(
                        "let Ok(csr)=((instruction_bits>>20)&0b111111111111).try_into()else{break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈˡᵒⁿᵍ};".to_owned(),
                    );
                }
                if 𝖿𝖽_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 || 𝗋𝖽_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽
                {
                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("let rd_bits=compressed_instruction_step as u32+rd_bits as u32;".to_owned());
                }
                if 𝖿𝖽_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 {
                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("let Ok(fd)=rd_bits.try_into()else{break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈˡᵒⁿᵍ};".to_owned());
                }
                if 𝗋𝖽_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 {
                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("let Ok(rd)=rd_bits.try_into()else{break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈˡᵒⁿᵍ};".to_owned());
                }
                if 𝖿𝗌𝟣_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 || 𝗋𝗌𝟣_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽
                {
                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("let rs1_bits=(instruction_bits>>15)&0b11111;".to_owned());
                }
                if 𝖿𝗌𝟣_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 {
                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("let Ok(fs1)=rs1_bits.try_into()else{break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈˡᵒⁿᵍ};".to_owned());
                }
                if 𝗋𝗌𝟣_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 {
                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("let Ok(rs1)=rs1_bits.try_into()else{break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈˡᵒⁿᵍ};".to_owned());
                }
                if 𝖿𝗌𝟤_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 || 𝗋𝗌𝟤_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽
                {
                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("let rs2_bits=(instruction_bits>>20)&0b11111;".to_owned());
                }
                if 𝖿𝗌𝟤_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 {
                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("let Ok(fs2)=rs2_bits.try_into()else{break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈˡᵒⁿᵍ};".to_owned());
                }
                if 𝗋𝗌𝟤_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 {
                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("let Ok(rs2)=rs2_bits.try_into()else{break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈˡᵒⁿᵍ};".to_owned());
                }
                if 𝖿𝗌𝟥_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 {
                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("let rs3_bits=(instruction_bits>>27)&0b11111;".to_owned());
                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("let Ok(fs3)=rs3_bits.try_into()else{break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈˡᵒⁿᵍ};".to_owned());
                }
                if 𝗂𝗆𝗆𝖾𝖽𝗂𝖺𝗍𝖾_𝗇𝖾𝖾𝖽𝖾𝖽.is_some() {
                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("let imm=𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧(instruction_bits as i32).into();".to_owned());
                }

                let mut first_if = true;
                let mut unprocessed_instructions = 0;
                for ((𝗈𝗉𝖼𝗈𝖽𝖾, 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄), instruction) in disassembler_instructions_map.iter()
                {
                    // Ignore func3 and main_opcode parts.
                    let 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄 = (!𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄) & 0xffff8f80;
                    let 𝖾𝗆𝗂𝗍_𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇: &str = instruction.𝖾𝗆𝗂𝗍_𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇.as_ref();
                    // Process all no-operand instructions here.
                    if 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄 == 0xffff8f80 {
                        if first_if {
                            first_if = false;
                        } else {
                            𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("else ".to_owned());
                        }
                        𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push(format!("if instruction_bits==0x{𝗈𝗉𝖼𝗈𝖽𝖾:08x}{{{𝖾𝗆𝗂𝗍_𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇}}}"));
                        continue;
                    }
                    // Values 5 and 6 are reserved for rm field, instruction would be handled by unimplemented_32bit_instruction.
                    if instruction.𝗋𝗆_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 && (func3 == 5 || func3 == 6) {
                        continue;
                    }
                    unprocessed_instructions += 1;
                }
                if let Some((collected_opcode, collected_opcode_mask)) = collected_opcode_and_mask {
                    if unprocessed_instructions > 0 {
                        if !first_if {
                            𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("else{".to_owned());
                        }
                        match collected_opcode_mask {
                            0x00000000 => (),
                            0x01f00f80 => 𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push(format!("if rd_bits=={}{{let rs2=(instruction_bits>>20)&0b11111;let imm=𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧(instruction_bits as i32).into();match rs2{{", (collected_opcode>>7)&0b11111)),
                            0x06000000 => 𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("match(instruction_bits >> 25)&0b11{".to_owned()),
                            0xfc000000 => 𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("match(instruction_bits >> 26)&0b111111{".to_owned()),
                            0xfe000000 => 𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("match(instruction_bits >> 25)&0b1111111{".to_owned()),
                            _ => panic!("Unsupported opcode mask: 0x{collected_opcode_mask:08x}"),
                        }
                        // Note: RISC-V design specifically designates 0 as invalid instruction and it's also compressed one, thus
                        // it may never be a valid instruction.
                        let mut last_processed_opcode = 0;
                        let mut submatch_used = None;
                        for ((𝗈𝗉𝖼𝗈𝖽𝖾, 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄), instruction) in disassembler_instructions_map.iter()
                        {
                            // Ignore func3 and main_opcode parts.
                            let 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄 = (!𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄) & 0xffff8f80;
                            // Skip no-operand instructions.
                            if 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄 == 0xffff8f80 {
                                continue;
                            }
                            let 𝖾𝗆𝗂𝗍_𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇: &str = instruction.𝖾𝗆𝗂𝗍_𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇.as_ref();
                            // Values 5 and 6 are reserved for rm field, instruction would be handled by unimplemented_32bit_instruction.
                            if instruction.𝗋𝗆_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽 && (func3 == 5 || func3 == 6) {
                                continue;
                            }
                            match (collected_opcode_mask, 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄) {
                                (0x01f00f80, 0x00000000) => {
                                    submatch_used = Some(format!("_=>(),}}}}{𝖾𝗆𝗂𝗍_𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇}"));
                                    unprocessed_instructions -= 1;
                                }
                                (0x01f00f80, 0x01f00f80) => {
                                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push(format!("{}=>{{{𝖾𝗆𝗂𝗍_𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇}}}", (𝗈𝗉𝖼𝗈𝖽𝖾 >> 20) & 0b11111));
                                    unprocessed_instructions -= 1;
                                }
                                (0xfe000000, 0xfff00000) => {
                                    if last_processed_opcode != 𝗈𝗉𝖼𝗈𝖽𝖾 & 0xfe000000 {
                                        if let Some(submatch_used) = submatch_used.take() {
                                            𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push(submatch_used);
                                        }
                                        last_processed_opcode = 𝗈𝗉𝖼𝗈𝖽𝖾 & 0xfe000000;
                                        𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push(format!("{}=>match rs2_bits{{", (𝗈𝗉𝖼𝗈𝖽𝖾>>25)&0b1111111));
                                        submatch_used = Some("_=>break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈˡᵒⁿᵍ,}".to_owned());
                                    }
                                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push(format!("{}=>{𝖾𝗆𝗂𝗍_𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇},", (𝗈𝗉𝖼𝗈𝖽𝖾 >> 20) & 0b11111));
                                }
                                (collected_opcode_mask, 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄)
                                    if collected_opcode_mask == 𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄 =>
                                {
                                    if let Some(submatch_used) = submatch_used.take() {
                                        𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push(submatch_used);
                                    }
                                    match collected_opcode_mask {
                                        0x00000000 => 𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push(format!("{𝖾𝗆𝗂𝗍_𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇};")),
                                        0x06000000 => 𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push(format!("{}=>{𝖾𝗆𝗂𝗍_𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇},", (𝗈𝗉𝖼𝗈𝖽𝖾>>25)&0b11)),
                                        0xfc000000 => 𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push(format!("{}=>{𝖾𝗆𝗂𝗍_𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇},", (𝗈𝗉𝖼𝗈𝖽𝖾>>26)&0b111111)),
                                        0xfe000000 => 𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push(format!("{}=>{𝖾𝗆𝗂𝗍_𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇},", (𝗈𝗉𝖼𝗈𝖽𝖾>>25)&0b1111111)),
                                        _ => panic!("Internal error {𝗈𝗉𝖼𝗈𝖽𝖾} 0x{collected_opcode_mask:08x}"), // Should have been reported already.
                                    }
                                }
                                _ => panic!("Inconsistent opcode masks: 0x{collected_opcode_mask:08x} 0x{𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄:08x}"),
                            }
                        }
                        if let Some(submatch_used) = submatch_used.take() {
                            𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push(submatch_used);
                        }
                        if collected_opcode_mask != 0x00000000 && collected_opcode_mask != 0x01f00f80 {
                            𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("_=>break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈˡᵒⁿᵍ}".to_owned());
                        }
                        if !first_if {
                            𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("}".to_owned());
                        }
                    }
                }
                if candidate_instructions > 0 {
                    𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("}".to_owned());
                }
            }
            𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌.push("_=>break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈˡᵒⁿᵍ,}}".to_owned());
        }
        assembler_instructions[assembler_kind as usize].extend(leaf_assembler_instructions[assembler_kind as usize].values().map(
            |assembler_instruction| {
                assert_eq!(assembler_instruction.len(), 1);
                assembler_instruction[0].clone()
            },
        ));
        disassembler_instructions[assembler_kind as usize].extend(
            leaf_disassembler_instructions[assembler_kind as usize]
                .values()
                .map(|assembler_instruction| {
                    assert_eq!(assembler_instruction.len(), 1);
                    assembler_instruction[0].clone()
                }),
        );
        let mut position = 0;
        let mut names_literal = Vec::new();
        let (enum_variant_list, enum_match_list): (Vec<_>, Vec<_>) = 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝖾𝗇𝗎𝗆𝗌[assembler_kind as usize]
            .iter()
            .map(|(instruction_name, enum_instruction_names)| {
                assert_eq!(enum_instruction_names.len(), 1);
                names_literal.extend_from_slice(format!("\\x{:02x}", instruction_name.len()).as_bytes());
                names_literal.extend_from_slice(instruction_name.as_bytes());
                let defintion = format!("{}={}", enum_instruction_names[0], position);
                let match_arm = format!("{} => Ok(𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝::{}),", position, enum_instruction_names[0]);
                position += instruction_name.len() + 1;
                (defintion, match_arm)
            })
            .unzip();
        instructions_enum_declararion[assembler_kind as usize] =
            format!("#[derive(Clone,Copy,Debug,Eq,Ord,PartialEq,PartialOrd)]#[repr(i16)]pub enum 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝{{{}}}#[cfg(feature = \"std\")]#[allow(non_upper_case_globals)]impl std::fmt::Display for 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝{{#[inline(always)]fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{{std::fmt::Write::write_str(formatter,unsafe{{core::str::from_utf8_unchecked(&𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢𝔰.as_bytes()[*self as usize+1..*self as usize+1+((𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢𝔰.as_bytes()[*self as usize])as usize)])}})}}}}const 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢𝔰:&str=\"{}\";impl TryFrom<i16> for 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝{{type Error=super::super::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;#[inline(always)]fn try_from(value: i16) -> Result<Self, Self::Error>{{match value {{{}_ => Err(super::super::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))}}}}}}", enum_variant_list.join(","), core::str::from_utf8(&names_literal).unwrap(), enum_match_list.concat());
    }

    let 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌 = 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌.map(|traits_info| {
        traits_info
            .iter()
            .map(|((𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾, 𝖿𝗇_𝗇𝖺𝗆𝖾), trait_info)| {
                let trait_info = trait_info
                    .values()
                    .map(|trait_info| {
                        assert_eq!(trait_info.len(), 1);
                        trait_info[0].clone()
                    })
                    .collect::<Vec<_>>()
                    .join("+");
                format!("pub trait {𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}⋇:Æ+{trait_info}æ{{#[inline(always)]fn {𝖿𝗇_𝗇𝖺𝗆𝖾}<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>(&mut self,arguments:𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮)->Result<<Self as 𝗿𝗶𝘀𝗰_𝘃::{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞,<Self as 𝗿𝗶𝘀𝗰_𝘃::{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>where Self:𝗿𝗶𝘀𝗰_𝘃::{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>{{𝗿𝗶𝘀𝗰_𝘃::{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}::<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮>::{𝖿𝗇_𝗇𝖺𝗆𝖾}(self,arguments)}}}}impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮:Æ+{trait_info}>{𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾}⋇ for 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 æ{{}}")
            })
            .collect::<Vec<_>>()
    });

    𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
        𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇: riscv_assembler_instructions.concat(),
        𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈: [
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
                𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝖾𝗇𝗎𝗆𝗌: instructions_enum_declararion[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔢 as usize].clone(),
                𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌: 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔢 as usize].concat(),
                𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌: 𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔢 as usize].concat(),
                𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇: assembler_instructions[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔢 as usize].concat(),
                𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌: 𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔢 as usize]
                    .iter().map(|v| v.concat()).collect::<Vec<_>>().try_into().unwrap(),
                𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖼𝗈𝗇𝗌𝗎𝗆𝖾_𝗋𝖾𝖽𝗂𝗋𝖾𝖼𝗍𝗈𝗋𝗌: disassembler_instructions[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔢 as usize].concat(),
            },
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
                𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝖾𝗇𝗎𝗆𝗌: instructions_enum_declararion[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔦 as usize].clone(),
                𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌: 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔦 as usize].concat(),
                𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌: 𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔦 as usize].concat(),
                𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇: assembler_instructions[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔦 as usize].concat(),
                𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌: 𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔦 as usize]
                    .iter().map(|v| v.concat()).collect::<Vec<_>>().try_into().unwrap(),
                𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖼𝗈𝗇𝗌𝗎𝗆𝖾_𝗋𝖾𝖽𝗂𝗋𝖾𝖼𝗍𝗈𝗋𝗌: disassembler_instructions[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳32𝔦 as usize].concat(),
            },
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
                𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝖾𝗇𝗎𝗆𝗌: instructions_enum_declararion[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳64𝔦 as usize].clone(),
                𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌: 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳64𝔦 as usize].concat(),
                𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌: 𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳64𝔦 as usize].concat(),
                𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇: assembler_instructions[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳64𝔦 as usize].concat(),
                𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌: 𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳64𝔦 as usize]
                    .iter().map(|v| v.concat()).collect::<Vec<_>>().try_into().unwrap(),
                𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖼𝗈𝗇𝗌𝗎𝗆𝖾_𝗋𝖾𝖽𝗂𝗋𝖾𝖼𝗍𝗈𝗋𝗌: disassembler_instructions[𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳64𝔦 as usize].concat(),
            },
        ],
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct 𝐝𝐞𝐜𝐨𝐝𝐞𝐝_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨 {
    𝖼𝗌𝗋_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: bool,
    𝖿𝖽_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: bool,
    𝗋𝖽_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: bool,
    𝖿𝗌𝟣_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: bool,
    𝗋𝗌𝟣_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: bool,
    𝖿𝗌𝟤_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: bool,
    𝗋𝗌𝟤_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: bool,
    𝖿𝗌𝟥_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: bool,
    𝗋𝗆_𝖿𝗂𝖾𝗅𝖽_𝗇𝖾𝖾𝖽𝖾𝖽: bool,
    𝗂𝗆𝗆𝖾𝖽𝗂𝖺𝗍𝖾_𝗇𝖾𝖾𝖽𝖾𝖽: Option<𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞_𝐭𝐲𝐩𝐞>,
    𝖾𝗆𝗂𝗍_𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum 𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞_𝐭𝐲𝐩𝐞 {
    𝔟𝔯𝔞𝔫𝔠𝔥_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢,
    𝔠𝔰𝔯_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢,
    𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢,
    𝔭𝔯𝔢𝔣𝔢𝔱𝔠𝔥_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢,
    𝔰𝔬𝔲𝔯𝔠𝔢_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢,
    𝔧𝔲𝔪𝔭_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢,
    𝔰𝔥𝔦𝔣𝔱_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢,
    𝔲𝔭𝔭𝔢𝔯_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢,
    𝔴𝔬𝔯𝔡_𝔰𝔥𝔦𝔣𝔱_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢,
}

async fn get_database_connection() -> sqlx::SqliteConnection {
    use sqlx::Connection;
    let root_path = std::env::current_dir().expect("Obtaining crate root path");
    let root_path = root_path.to_str().expect("Turning crate root path into unicode string");
    // Note: during regular build root_path points to the yace workspace root, but in doctests
    // we get nested crate root.  Try to access both paths.
    let database_url = format!("sqlite:{root_path}/riscv-instructions.db?immutable=1");
    let database_url_fallback = format!("sqlite:{root_path}/../riscv-instructions.db?immutable=1");
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
    for<'ᵇʸᵗᵉ> u32: sqlx::Type<𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>
        + sqlx::Encode<'ᵇʸᵗᵉ, 𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>
        + sqlx::Decode<'ᵇʸᵗᵉ, 𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    <𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as sqlx::database::HasArguments<'ᵉˣᵉᶜᵘᵗᵒʳ>>::Arguments: sqlx::IntoArguments<'ᵉˣᵉᶜᵘᵗᵒʳ, 𝓭𝓪𝓽𝓪𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
{
    let rust_types_map = assembler_kind.as_rust_types_map();

    if operands_count == 0 {
        *query = indoc! {"
            SELECT instruction.name AS name,
                   opcode,
                   opcode_mask
            FROM instruction LEFT JOIN
            operands ON operands = short_name
            WHERE (instruction.assembler_kind IS NULL OR instruction.assembler_kind == $1) AND
                  operands.operand0 IS NULL
            GROUP BY instruction.name
            ORDER BY instruction.name;"}
        .to_owned();
    } else {
        let mut operand_requests = Vec::new();
        let mut select_traits = Vec::new();
        let mut operand_information = Vec::new();
        let mut trait_information = Vec::new();
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
        for index in 0..operands_count {
            operand_requests.push(formatdoc! {"
                operand{index}.parameter_type AS type{index},
                trait{index}.name AS trait{index},
                operand{index}.operand_source AS source{index},
                operand{index},"});
            let (prefix, suffix) = if index == 0 {
                ("", "".to_owned())
            } else {
                (" LEFT JOIN", format!("ON name0 = name{index}"))
            };
            select_traits.push(formatdoc! {"
                {prefix}(
                    SELECT name{index}, trait{index}
                    FROM (
                        SELECT instruction.name AS name{index}, traits_information.name AS trait{index}, priority
                        FROM instruction LEFT JOIN
                             operands ON operands = short_name {operand_count_check} LEFT JOIN
                             operand ON operand{index} = operand.name LEFT JOIN
                             traits_information ON parameter_type = allowed_operand
                             LEFT JOIN traits_priority ON traits_information.name = traits_priority.name
                        WHERE instruction.assembler_kind IS NULL OR
                              instruction.assembler_kind == $1
                        GROUP BY instruction.name, operands
                        HAVING priority = MIN(priority)
                        ORDER BY instruction.name, operands, priority
                    )
                    GROUP BY name{index}
                    HAVING priority = MAX(priority)
                    ORDER BY name{index}
                ){suffix}"});
            operand_information.push(format!(
                " LEFT JOIN operand AS operand{index} ON operand{index} = operand{index}.name"
            ));
            trait_information.push(formatdoc! {"
                ,traits_information AS trait{index} ON trait{index} = trait{index}.name
                AND operand{index}.parameter_type = trait{index}.allowed_operand"});
            type_list.push(format!(", type{index}"));
        }
        let operand_requests = operand_requests.concat();
        let select_traits = select_traits.concat();
        let operand_information = operand_information.concat();
        let trait_information = trait_information.concat();
        let type_list = type_list.concat();
        *query = formatdoc! {"
            SELECT name0 AS name,
                   {operand_requests}
                   opcode,
                   opcode_mask
            FROM {select_traits} LEFT JOIN
            instruction ON name0 = instruction.name LEFT JOIN
            operands ON operands = short_name
            {operand_information}
            {trait_information}
            WHERE (instruction.assembler_kind IS NULL OR instruction.assembler_kind == $1)
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
            let instruction_enum_variant_name =
                super::𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍_𝗇𝖺𝗆𝖾(instruction_name.as_str());
            let instruction_auto_trait_name =
                super::𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾(instruction_name.as_str());

            let mut 𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌 = Vec::new();
            for i in 0..operands_count {
                const TYPE: [&str; 5] = ["type0", "type1", "type2", "type3", "type4"];
                let argument: String = row.try_get(TYPE[i])?;
                const TRAIT: [&str; 5] = ["trait0", "trait1", "trait2", "trait3", "trait4"];
                let argument_trait: String = row.try_get(TRAIT[i])?;
                const OPERAND: [&str; 5] = ["operand0", "operand1", "operand2", "operand3", "operand4"];
                let operand: String = row.try_get(OPERAND[i])?;
                const SOURCE: [&str; 5] = ["source0", "source1", "source2", "source3", "source4"];
                let source: String = row.try_get(SOURCE[i])?;

                let argument_type = *rust_types_map
                    .get(argument.as_str())
                    .expect("Failed to convert sql type to rust type");
                let argument_trait = 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱
                    .get(argument_trait.as_str())
                    .expect("Failed to convert sql type to rust type");
                let argument_str = if assembler_kind != 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞::𝔯𝔳64𝔦 && argument == ">:imm"
                {
                    "<:imm"
                } else {
                    argument.as_str()
                };
                let argument_enum_variant = 𝔰𝔮𝔩_𝔱𝔬_𝔢𝔫𝔲𝔪
                    .get(argument_str)
                    .expect("Failed to convert sql type to rust type");
                let argument_trait_type = 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱
                    .get(argument_str)
                    .expect("Failed to convert sql type to rust type");

                𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌.push(𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭_𝐭𝐲𝐩𝐞 {
                    𝗌𝗊𝗅_𝗍𝗒𝗉𝖾: argument,
                    𝗌𝗊𝗅_𝗌𝗈𝗎𝗋𝖼𝖾: source,
                    𝗌𝗊𝗅_𝗈𝗉𝖾𝗋𝖺𝗇𝖽: operand,
                    𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾: argument_type,
                    𝗋𝗎𝗌𝗍_𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍: argument_enum_variant,
                    𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍: argument_trait,
                    𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍_𝗍𝗒𝗉𝖾: argument_trait_type,
                });
            }
            Ok(𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
                𝖿𝗇_𝗇𝖺𝗆𝖾: instruction_name,
                𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾: instruction_auto_trait_name,
                𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾: instruction_trait_name,
                𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍_𝗇𝖺𝗆𝖾: instruction_enum_variant_name,
                𝗈𝗉𝖼𝗈𝖽𝖾: row.try_get("opcode")?,
                𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄: row.try_get("opcode_mask")?,
                𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌,
            })
        })
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(i8)]
pub(crate) enum 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞 {
    𝔯𝔳32𝔢 = 0,
    𝔯𝔳32𝔦 = 1,
    𝔯𝔳64𝔦 = 2,
}

impl 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞 {
    fn as_str(self) -> &'static str {
        ["rv32", "rv32", "rv64"][self as usize]
    }
    fn as_rust_types_map(self) -> &'static HashMap<&'static str, &'static str> {
        [&*𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_𝔯𝔳32𝔢, &*𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_𝔯𝔳32, &*𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_𝔯𝔳64][self as usize]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
    𝖿𝗇_𝗇𝖺𝗆𝖾: String,
    𝖺𝗎𝗍𝗈_𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾: String,
    𝗍𝗋𝖺𝗂𝗍_𝗇𝖺𝗆𝖾: String,
    𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍_𝗇𝖺𝗆𝖾: String,
    𝗈𝗉𝖼𝗈𝖽𝖾: u32,
    𝗈𝗉𝖼𝗈𝖽𝖾_𝗆𝖺𝗌𝗄: u32,
    𝖺𝗋𝗀𝗎𝗆𝖾𝗇𝗍𝗌: Vec<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭_𝐭𝐲𝐩𝐞>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭_𝐭𝐲𝐩𝐞 {
    𝗌𝗊𝗅_𝗍𝗒𝗉𝖾: String,
    𝗌𝗊𝗅_𝗌𝗈𝗎𝗋𝖼𝖾: String,
    𝗌𝗊𝗅_𝗈𝗉𝖾𝗋𝖺𝗇𝖽: String,
    𝗋𝗎𝗌𝗍_𝗍𝗒𝗉𝖾: &'static str,
    𝗋𝗎𝗌𝗍_𝖾𝗇𝗎𝗆_𝗏𝖺𝗋𝗂𝖺𝗇𝗍: &'static str,
    𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍: &'static str,
    𝗋𝗎𝗌𝗍_𝗍𝗋𝖺𝗂𝗍_𝗍𝗒𝗉𝖾: &'static str,
}

pub(crate) static 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫𝔰_𝔦𝔫𝔣𝔬: Lazy<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞> = Lazy::new(get_instrution_info);
static 𝔰𝔮𝔩_𝔱𝔬_𝔢𝔫𝔲𝔪: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    hashmap! {
        "<:imm" => "𝔰𝔥𝔦𝔣𝔱_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡",
        ">:imm" => "𝔴𝔬𝔯𝔡_𝔰𝔥𝔦𝔣𝔱_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡",
        "0(gpr)" => "𝔞𝔱𝔬𝔪𝔦𝔠_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡",
        "b:imm" => "𝔟𝔯𝔞𝔫𝔠𝔥_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡",
        "c:imm" => "𝔠𝔰𝔯_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡",
        "csr" => "𝔠𝔰𝔯_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡",
        "fence" => "𝔣𝔢𝔫𝔠𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡",
        "fpr" => "𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡",
        "gpr" => "𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡",
        "i:imm" => "𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡",
        "i:imm(gpr)" => "𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡",
        "j:imm" => "𝔧𝔲𝔪𝔭_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡",
        "p:imm(gpr)" => "𝔭𝔯𝔢𝔣𝔢𝔱𝔠𝔥_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡",
        "rm" => "𝔯𝔬𝔲𝔫𝔡𝔦𝔫𝔤_𝔪𝔬𝔡𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡",
        "s:imm" => "𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡",
        "s:imm(gpr)" => "𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡",
        "u:imm" => "𝔲𝔭𝔭𝔢𝔯_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡"
    }
});
static 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    hashmap! {
        "<:imm" => "Self::𝐬𝐡𝐢𝐟𝐭_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        ">:imm" => "Self::𝐰𝐨𝐫𝐝_𝐬𝐡𝐢𝐟𝐭_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "0(gpr)" => "𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞>",
        "b:imm" => "Self::𝐛𝐫𝐚𝐧𝐜𝐡_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "c:imm" => "Self::𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "csr" => "Self::𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫",
        "csr_assembler_operand" => "𝒄𝒔𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅",
        "destination_assembler_operand" => "𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅",
        "fence" => "Self::𝐟𝐞𝐧𝐜𝐞",
        "fpr" => "Self::𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜",
        "gpr" => "Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜",
        "i:imm" => "Self::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "i:imm(gpr)" => "𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, Self::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞>",
        "j:imm" => "Self::𝐣𝐮𝐦𝐩_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "p:imm(gpr)" => "𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, Self::𝐩𝐫𝐞𝐟𝐞𝐭𝐜𝐡_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞>",
        "prefetch_assembler_operand" => "𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅",
        "s:imm" => "𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "s:imm(gpr)" => "𝒂𝒅𝒅𝒓𝒆𝒔𝒔<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞>",
        "shift_rv32_assembler_operand" => "𝒔𝒉𝒊𝒇𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅",
        "shift_rv64_assembler_operand" => "𝒔𝒉𝒊𝒇𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅",
        "source_assembler_operand" => "𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅",
        "u:imm" => "Self::𝐮𝐩𝐩𝐞𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "zero_offset_base_assembler_operand" => "𝒛𝒆𝒓𝒐_𝒐𝒇𝒇𝒔𝒆𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒐𝒑𝒆𝒓𝒂𝒏𝒅",
        "rm" => "Self::𝐫𝐨𝐮𝐧𝐝𝐢𝐧𝐠_𝐦𝐨𝐝𝐞"
    }
});
static 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_𝔯𝔳32𝔢: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    hashmap! {
        ">:imm" => "𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "0(gpr)" => "𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞>",
        "b:imm" => "𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "c:imm" => "𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "csr" => "𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐫𝐯𝟑𝟐",
        "fence" => "𝐟𝐞𝐧𝐜𝐞",
        "fpr" => "𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜",
        "gpr" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞",
        "i:imm" => "𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "i:imm(gpr)" => "𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞, 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞>",
        "j:imm" => "𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "p:imm(gpr)" => "𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞, 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞>",
        "s:imm" => "𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "s:imm(gpr)" => "𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞, 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞>",
        "u:imm" => "𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "rm" => "𝐫𝐨𝐮𝐧𝐝𝐢𝐧𝐠_𝐦𝐨𝐝𝐞"
    }
});
static 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_𝔯𝔳32: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    hashmap! {
        ">:imm" => "𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "0(gpr)" => "𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞>",
        "b:imm" => "𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "c:imm" => "𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "csr" => "𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐫𝐯𝟑𝟐",
        "fence" => "𝐟𝐞𝐧𝐜𝐞",
        "fpr" => "𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜",
        "gpr" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜",
        "i:imm" => "𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "i:imm(gpr)" => "𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞>",
        "j:imm" => "𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "p:imm(gpr)" => "𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞>",
        "s:imm" => "𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "s:imm(gpr)" => "𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞>",
        "u:imm" => "𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "rm" => "𝐫𝐨𝐮𝐧𝐝𝐢𝐧𝐠_𝐦𝐨𝐝𝐞"
    }
});
static 𝔰𝔮𝔩_𝔱𝔬_𝔯𝔲𝔰𝔱_𝔯𝔳64: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    hashmap! {
        "<:imm" => "𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        ">:imm" => "𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "0(gpr)" => "𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞>",
        "b:imm" => "𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "c:imm" => "𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "csr" => "𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐫𝐯𝟔𝟒",
        "fence" => "𝐟𝐞𝐧𝐜𝐞",
        "fpr" => "𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜",
        "gpr" => "𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜",
        "i:imm" => "𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "i:imm(gpr)" => "𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞>",
        "j:imm" => "𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "p:imm(gpr)" => "𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞>",
        "s:imm" => "𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "s:imm(gpr)" => "𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞>",
        "u:imm" => "𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞",
        "rm" => "𝐫𝐨𝐮𝐧𝐝𝐢𝐧𝐠_𝐦𝐨𝐝𝐞"
    }
});

pub(crate) struct 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧𝐬_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
    pub(crate) 𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇: String,
    // These are indexed by 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐭𝐲𝐩𝐞 as usize.
    pub(crate) 𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋_𝗂𝗇𝖿𝗈: [𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞; 3],
}

pub(crate) struct 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐢𝐧𝐟𝐨_𝐭𝐲𝐩𝐞 {
    pub(crate) 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝖾𝗇𝗎𝗆𝗌: String,
    pub(crate) 𝖽𝖾𝖼𝗅𝖺𝗋𝖾_𝗍𝗋𝖺𝗂𝗍𝗌: String,
    pub(crate) 𝖿𝗈𝗋𝗐𝖺𝗋𝖽_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍_𝗍𝗋𝖺𝗂𝗍𝗌: String,
    pub(crate) 𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖻𝗒𝗍𝖾_𝖾𝗆𝗂𝗍_𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇: String,
    pub(crate) 𝖽𝖾𝖼𝗈𝖽𝖾_𝟥𝟤𝖻𝗂𝗍_𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌: [String; 8],
    pub(crate) 𝗂𝗇𝗌𝗍𝗋𝗎𝖼𝗍𝗂𝗈𝗇𝗌_𝖼𝗈𝗇𝗌𝗎𝗆𝖾_𝗋𝖾𝖽𝗂𝗋𝖾𝖼𝗍𝗈𝗋𝗌: String,
}
