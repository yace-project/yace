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
    proc_macro::{Delimiter, Group, Ident, TokenStream, TokenTree},
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬<'ᵉˣᵗʳᵃ> {
    𝗋𝗏_𝗆𝗈𝖽𝖾: Option<𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞>,
    𝗋𝗏_𝖺𝖻𝗂: Option<𝐫𝐢𝐬𝐜_𝐯_𝐚𝐛𝐢>,
    𝖾𝗑𝗍𝗋𝖺_𝖺𝗍𝗍𝗋𝗂𝖻𝗎𝗍𝖾𝗌: &'ᵉˣᵗʳᵃ super::𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐞𝐱𝐭𝐫𝐚_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum 𝐫𝐢𝐬𝐜_𝐯_𝐦𝐨𝐝𝐞 {
    𝔯𝔳32𝔢 = 0,
    𝔯𝔳32𝔦 = 1,
    𝔯𝔳64𝔦 = 2
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum 𝐫𝐢𝐬𝐜_𝐯_𝐚𝐛𝐢 {
    𝔢𝔞𝔟𝔦 = 0,
    𝔲𝔞𝔟𝔦 = 1
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
                    _ => 𝐫𝐢𝐬𝐜_𝐯_𝐚𝐛𝐢::𝔲𝔞𝔟𝔦
                });
            }
        }
        Ok(result)
    }
}

pub(crate) fn filter_riscv_markers_iterable(
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
            "𝔸𝕦𝕥𝕠𝕀𝕞𝕡𝕝𝕖𝕞𝕖𝕟𝕥𝕖𝕕𝕋𝕣𝕒𝕚𝕥" => {
                todo!();
            }
            _ => emit_tokens(output, output_extra, [token])
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
        todo!();
    }
}

fn filter_riscv_markers_group(
    input: &mut Group, attributes: 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐚𝐭𝐭𝐫𝐢𝐛𝐮𝐭𝐞𝐬
) -> TokenTree {
    let mut content = TokenStream::new();
    filter_riscv_markers_iterable(&mut content, &mut None, &mut input.stream().into_iter(), attributes);
    Group::new(input.delimiter(), content).into()
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
