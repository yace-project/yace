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

macro_rules! 𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙 {
    ($𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮:ident: $($𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮:ident => $𝓮𝓷𝓾𝓶_𝓼𝓽𝓻𝓲𝓷𝓰:literal),*) => {
        $(
            assert_eq!(super::𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮::$𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮.to_string(), $𝓮𝓷𝓾𝓶_𝓼𝓽𝓻𝓲𝓷𝓰);
         )*
    }
}

#[test]
fn test_enums_display() {
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ: 𝔞𝔩 => "al");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ: 𝔞𝔥 => "ah");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ: 𝔞𝔵 => "ax");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ: 𝔢𝔞𝔵 => "eax");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ: 𝔯𝔞𝔵 => "rax");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ: 𝔠𝔩 => "cl");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ: 𝔠𝔥 => "ch");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ: 𝔠𝔵 => "cx");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ: 𝔢𝔠𝔵 => "ecx");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ: 𝔯𝔠𝔵 => "rcx");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ: 𝔡𝔩 => "dl");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ: 𝔡𝔥 => "dh");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ: 𝔡𝔵 => "dx");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ: 𝔢𝔡𝔵 => "edx");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ: 𝔯𝔡𝔵 => "rdx");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ: 𝔟𝔩 => "bl");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ: 𝔟𝔥 => "bh");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ: 𝔟𝔵 => "bx");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ: 𝔢𝔟𝔵 => "ebx");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ: 𝔯𝔟𝔵 => "rbx");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ: 𝔰𝔭𝔩 => "spl");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ: 𝔰𝔭 => "sp");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ: 𝔢𝔰𝔭 => "esp");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ: 𝔯𝔰𝔭 => "rsp");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ: 𝔟𝔭𝔩 => "bpl");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ: 𝔟𝔭 => "bp");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ: 𝔢𝔟𝔭 => "ebp");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ: 𝔯𝔟𝔭 => "rbp");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ: 𝔰𝔦𝔩 => "sil");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ: 𝔰𝔦 => "si");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ: 𝔢𝔰𝔦 => "esi");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ: 𝔯𝔰𝔦 => "rsi");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ: 𝔡𝔦𝔩 => "dil");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ: 𝔡𝔦 => "di");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ: 𝔢𝔡𝔦 => "edi");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ: 𝔯𝔡𝔦 => "rdi");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫: 𝔢𝔦𝔷 => "eiz");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫: 𝔯𝔦𝔷 => "riz");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ: 𝔟𝔵 => "bx", 𝔟𝔭 => "bp");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ: 𝔰𝔦 => "si", 𝔡𝔦 => "di");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386:
        𝔢𝔞𝔵 => "eax", 𝔢𝔟𝔵 => "ebx", 𝔢𝔠𝔵 => "ecx", 𝔢𝔡𝔵 => "edx", 𝔢𝔟𝔭 => "ebp", 𝔢𝔰𝔦 => "esi", 𝔢𝔡𝔦 => "edi");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64:
        𝔢𝔞𝔵 => "eax", 𝔢𝔟𝔵 => "ebx", 𝔢𝔠𝔵 => "ecx", 𝔢𝔡𝔵 => "edx", 𝔢𝔟𝔭 => "ebp", 𝔢𝔰𝔦 => "esi", 𝔢𝔡𝔦 => "edi",
        𝔯8𝔡 => "r8d", 𝔯9𝔡 => "r9d", 𝔯10𝔡 => "r10d", 𝔯11𝔡 => "r11d", 𝔯12𝔡 => "r12d", 𝔯13𝔡 => "r13d", 𝔯14𝔡 => "r14d", 𝔯15𝔡 => "r15d");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ:
        𝔯𝔞𝔵 => "rax", 𝔯𝔟𝔵 => "rbx", 𝔯𝔠𝔵 => "rcx", 𝔯𝔡𝔵 => "rdx", 𝔯𝔟𝔭 => "rbp", 𝔯𝔰𝔦 => "rsi", 𝔯𝔡𝔦 => "rdi");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ:
        𝔯𝔞𝔵 => "rax", 𝔯𝔟𝔵 => "rbx", 𝔯𝔠𝔵 => "rcx", 𝔯𝔡𝔵 => "rdx", 𝔯𝔟𝔭 => "rbp", 𝔯𝔰𝔦 => "rsi", 𝔯𝔡𝔦 => "rdi",
        𝔯8 => "r8", 𝔯9 => "r9", 𝔯10 => "r10", 𝔯11 => "r11", 𝔯12 => "r12", 𝔯13 => "r13", 𝔯14 => "r14", 𝔯15 => "r15");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386:
        𝔢𝔞𝔵 => "eax", 𝔢𝔟𝔵 => "ebx", 𝔢𝔠𝔵 => "ecx", 𝔢𝔡𝔵 => "edx", 𝔢𝔦𝔷 => "eiz", 𝔢𝔟𝔭 => "ebp", 𝔢𝔰𝔦 => "esi", 𝔢𝔡𝔦 => "edi");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64:
        𝔢𝔞𝔵 => "eax", 𝔢𝔟𝔵 => "ebx", 𝔢𝔠𝔵 => "ecx", 𝔢𝔡𝔵 => "edx", 𝔢𝔦𝔷 => "eiz", 𝔢𝔟𝔭 => "ebp", 𝔢𝔰𝔦 => "esi", 𝔢𝔡𝔦 => "edi",
        𝔯8𝔡 => "r8d", 𝔯9𝔡 => "r9d", 𝔯10𝔡 => "r10d", 𝔯11𝔡 => "r11d", 𝔯12𝔡 => "r12d", 𝔯13𝔡 => "r13d", 𝔯14𝔡 => "r14d", 𝔯15𝔡 => "r15d");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ:
        𝔯𝔞𝔵 => "rax", 𝔯𝔟𝔵 => "rbx", 𝔯𝔠𝔵 => "rcx", 𝔯𝔡𝔵 => "rdx", 𝔯𝔦𝔷 => "riz", 𝔯𝔟𝔭 => "rbp", 𝔯𝔰𝔦 => "rsi", 𝔯𝔡𝔦 => "rdi");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ:
        𝔯𝔞𝔵 => "rax", 𝔯𝔟𝔵 => "rbx", 𝔯𝔠𝔵 => "rcx", 𝔯𝔡𝔵 => "rdx", 𝔯𝔦𝔷 => "riz", 𝔯𝔟𝔭 => "rbp", 𝔯𝔰𝔦 => "rsi", 𝔯𝔡𝔦 => "rdi",
        𝔯8 => "r8", 𝔯9 => "r9", 𝔯10 => "r10", 𝔯11 => "r11", 𝔯12 => "r12", 𝔯13 => "r13", 𝔯14 => "r14", 𝔯15 => "r15");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ: 𝔞𝔩 => "al", 𝔟𝔩 => "bl", 𝔠𝔩 => "cl", 𝔡𝔩 => "dl");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ: 𝔞𝔥 => "ah", 𝔟𝔥 => "bh", 𝔠𝔥 => "ch", 𝔡𝔥 => "dh");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086:
        𝔞𝔩 => "al", 𝔟𝔩 => "bl", 𝔠𝔩 => "cl", 𝔡𝔩 => "dl", 𝔞𝔥 => "ah", 𝔟𝔥 => "bh", 𝔠𝔥 => "ch", 𝔡𝔥 => "dh");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64:
        𝔞𝔩 => "al", 𝔟𝔩 => "bl", 𝔠𝔩 => "cl", 𝔡𝔩 => "dl", 𝔞𝔥 => "ah", 𝔟𝔥 => "bh", 𝔠𝔥 => "ch", 𝔡𝔥 => "dh",
        𝔰𝔭𝔩 => "spl", 𝔟𝔭𝔩 => "bpl", 𝔰𝔦𝔩 => "sil", 𝔡𝔦𝔩 => "dil", 𝔯8𝔟 => "r8b", 𝔯9𝔟 => "r9b",
        𝔯10𝔟 => "r10b", 𝔯11𝔟 => "r11b", 𝔯12𝔟 => "r12b", 𝔯13𝔟 => "r13b", 𝔯14𝔟 => "r14b", 𝔯15𝔟 => "r15b");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64:
        𝔞𝔩 => "al", 𝔟𝔩 => "bl", 𝔠𝔩 => "cl", 𝔡𝔩 => "dl", 𝔰𝔭𝔩 => "spl", 𝔟𝔭𝔩 => "bpl", 𝔰𝔦𝔩 => "sil", 𝔡𝔦𝔩 => "dil",
        𝔯8𝔟 => "r8b", 𝔯9𝔟 => "r9b", 𝔯10𝔟 => "r10b", 𝔯11𝔟 => "r11b", 𝔯12𝔟 => "r12b", 𝔯13𝔟 => "r13b", 𝔯14𝔟 => "r14b", 𝔯15𝔟 => "r15b");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086:
        𝔞𝔵 => "ax", 𝔟𝔵 => "bx", 𝔠𝔵 => "cx", 𝔡𝔵 => "dx", 𝔰𝔭 => "sp", 𝔟𝔭 => "bp", 𝔰𝔦 => "si", 𝔡𝔦 => "di");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64:
        𝔞𝔵 => "ax", 𝔟𝔵 => "bx", 𝔠𝔵 => "cx", 𝔡𝔵 => "dx", 𝔰𝔭 => "sp", 𝔟𝔭 => "bp", 𝔰𝔦 => "si", 𝔡𝔦 => "di",
        𝔯8𝔴 => "r8w", 𝔯9𝔴 => "r9w", 𝔯10𝔴 => "r10w", 𝔯11𝔴 => "r11w", 𝔯12𝔴 => "r12w", 𝔯13𝔴 => "r13w", 𝔯14𝔴 => "r14w", 𝔯15𝔴 => "r15w");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386:
        𝔢𝔞𝔵 => "eax", 𝔢𝔟𝔵 => "ebx", 𝔢𝔠𝔵 => "ecx", 𝔢𝔡𝔵 => "edx", 𝔢𝔰𝔭 => "esp", 𝔢𝔟𝔭 => "ebp", 𝔢𝔰𝔦 => "esi", 𝔢𝔡𝔦 => "edi");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64:
        𝔢𝔞𝔵 => "eax", 𝔢𝔟𝔵 => "ebx", 𝔢𝔠𝔵 => "ecx", 𝔢𝔡𝔵 => "edx", 𝔢𝔰𝔭 => "esp", 𝔢𝔟𝔭 => "ebp", 𝔢𝔰𝔦 => "esi", 𝔢𝔡𝔦 => "edi",
        𝔯8𝔡 => "r8d", 𝔯9𝔡 => "r9d", 𝔯10𝔡 => "r10d", 𝔯11𝔡 => "r11d", 𝔯12𝔡 => "r12d", 𝔯13𝔡 => "r13d", 𝔯14𝔡 => "r14d", 𝔯15𝔡 => "r15d");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ:
        𝔯𝔞𝔵 => "rax", 𝔯𝔟𝔵 => "rbx", 𝔯𝔠𝔵 => "rcx", 𝔯𝔡𝔵 => "rdx", 𝔯𝔰𝔭 => "rsp", 𝔯𝔟𝔭 => "rbp", 𝔯𝔰𝔦 => "rsi", 𝔯𝔡𝔦 => "rdi");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ:
        𝔯𝔞𝔵 => "rax", 𝔯𝔟𝔵 => "rbx", 𝔯𝔠𝔵 => "rcx", 𝔯𝔡𝔵 => "rdx", 𝔯𝔰𝔭 => "rsp", 𝔯𝔟𝔭 => "rbp", 𝔯𝔰𝔦 => "rsi", 𝔯𝔡𝔦 => "rdi",
        𝔯8 => "r8", 𝔯9 => "r9", 𝔯10 => "r10", 𝔯11 => "r11", 𝔯12 => "r12", 𝔯13 => "r13", 𝔯14 => "r14", 𝔯15 => "r15");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞: 𝔵1 => "1", 𝔵2 => "2", 𝔵4 => "4", 𝔵8 => "8");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝔢𝔰_𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫: 𝔢𝔰 => "es");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64: 𝔣𝔰 => "fs", 𝔤𝔰 => "gs");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐨_𝐜𝐬: 𝔢𝔰 => "es", 𝔰𝔰 => "ss", 𝔡𝔰 => "ds", 𝔣𝔰 => "fs", 𝔤𝔰 => "gs");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086: 𝔢𝔰 => "es", 𝔠𝔰 => "cs", 𝔰𝔰 => "ss", 𝔡𝔰 => "ds", 𝔣𝔰 => "fs", 𝔤𝔰 => "gs");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐬𝐞𝐠𝐦𝐞𝐧𝐭_8086_𝐫𝐞𝐠:
        𝔢𝔰 => "es", 𝔠𝔰 => "cs", 𝔰𝔰 => "ss", 𝔡𝔰 => "ds", 𝔣𝔰 => "fs", 𝔤𝔰 => "gs", _𝔡𝔲𝔪𝔪𝔶6 => "d6", _𝔡𝔲𝔪𝔪𝔶7 => "d7");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫: 𝔰𝔱 => "st");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫:
        𝔰𝔱0 => "st(0)", 𝔰𝔱1 => "st(1)", 𝔰𝔱2 => "st(2)", 𝔰𝔱3 => "st(3)", 𝔰𝔱4 => "st(4)", 𝔰𝔱5 => "st(5)", 𝔰𝔱6 => "st(6)", 𝔰𝔱7 => "st(7)");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐜𝐨𝐧𝐭𝐫𝐨𝐥_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_80386: 𝔠𝔯0 => "cr0", 𝔠𝔯1 => "cr1", 𝔠𝔯2 => "cr2", 𝔠𝔯3 => "cr3", 𝔠𝔯4 => "cr4");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐜𝐨𝐧𝐭𝐫𝐨𝐥_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64:
        𝔠𝔯0 => "cr0", 𝔠𝔯1 => "cr1", 𝔠𝔯2 => "cr2", 𝔠𝔯3 => "cr3", 𝔠𝔯4 => "cr4", 𝔠𝔯8 => "cr8");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐝𝐞𝐛𝐮𝐠_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫: 𝔡𝔯0 => "dr0", 𝔡𝔯1 => "dr1", 𝔡𝔯2 => "dr2", 𝔡𝔯3 => "dr3", 𝔡𝔯6 => "dr6", 𝔡𝔯7 => "dr7");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐭𝐞𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫: 𝔱𝔯3 => "tr3", 𝔱𝔯4 => "tr4", 𝔱𝔯5 => "tr5", 𝔱𝔯6 => "tr6", 𝔱𝔯7 => "tr7");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐦𝐦𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫:
        𝔪𝔪0 => "mm0", 𝔪𝔪1 => "mm1", 𝔪𝔪2 => "mm2", 𝔪𝔪3 => "mm3", 𝔪𝔪4 => "mm4", 𝔪𝔪5 => "mm5", 𝔪𝔪6 => "mm6", 𝔪𝔪7 => "mm7");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫:
        𝔨1 => "k1", 𝔨2 => "k2", 𝔨3 => "k3", 𝔨4 => "k4", 𝔨5 => "k5", 𝔨6 => "k6", 𝔨7 => "k7");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫:
        𝔨0 => "k0", 𝔨1 => "k1", 𝔨2 => "k2", 𝔨3 => "k3", 𝔨4 => "k4", 𝔨5 => "k5", 𝔨6 => "k6", 𝔨7 => "k7");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫: 𝔵𝔪𝔪0 => "xmm0");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₚₑₙₜᵢᵤₘ3:
        𝔵𝔪𝔪0 => "xmm0", 𝔵𝔪𝔪1 => "xmm1", 𝔵𝔪𝔪2 => "xmm2", 𝔵𝔪𝔪3 => "xmm3", 𝔵𝔪𝔪4 => "xmm4", 𝔵𝔪𝔪5 => "xmm5", 𝔵𝔪𝔪6 => "xmm6", 𝔵𝔪𝔪7 => "xmm7");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64:
        𝔵𝔪𝔪0 => "xmm0", 𝔵𝔪𝔪1 => "xmm1", 𝔵𝔪𝔪2 => "xmm2", 𝔵𝔪𝔪3 => "xmm3", 𝔵𝔪𝔪4 => "xmm4", 𝔵𝔪𝔪5 => "xmm5",
        𝔵𝔪𝔪6 => "xmm6", 𝔵𝔪𝔪7 => "xmm7", 𝔵𝔪𝔪8 => "xmm8", 𝔵𝔪𝔪9 => "xmm9", 𝔵𝔪𝔪10 => "xmm10",
        𝔵𝔪𝔪11 => "xmm11", 𝔵𝔪𝔪12 => "xmm12", 𝔵𝔪𝔪13 => "xmm13", 𝔵𝔪𝔪14 => "xmm14", 𝔵𝔪𝔪15 => "xmm15");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512:
        𝔵𝔪𝔪0 => "xmm0", 𝔵𝔪𝔪1 => "xmm1", 𝔵𝔪𝔪2 => "xmm2", 𝔵𝔪𝔪3 => "xmm3", 𝔵𝔪𝔪4 => "xmm4", 𝔵𝔪𝔪5 => "xmm5",
        𝔵𝔪𝔪6 => "xmm6", 𝔵𝔪𝔪7 => "xmm7", 𝔵𝔪𝔪8 => "xmm8", 𝔵𝔪𝔪9 => "xmm9", 𝔵𝔪𝔪10 => "xmm10", 𝔵𝔪𝔪11 => "xmm11",
        𝔵𝔪𝔪12 => "xmm12", 𝔵𝔪𝔪13 => "xmm13", 𝔵𝔪𝔪14 => "xmm14", 𝔵𝔪𝔪15 => "xmm15", 𝔵𝔪𝔪16 => "xmm16",
        𝔵𝔪𝔪17 => "xmm17", 𝔵𝔪𝔪18 => "xmm18", 𝔵𝔪𝔪19 => "xmm19", 𝔵𝔪𝔪20 => "xmm20", 𝔵𝔪𝔪21 => "xmm21",
        𝔵𝔪𝔪22 => "xmm22", 𝔵𝔪𝔪23 => "xmm23", 𝔵𝔪𝔪24 => "xmm24", 𝔵𝔪𝔪25 => "xmm25", 𝔵𝔪𝔪26 => "xmm26",
        𝔵𝔪𝔪27 => "xmm27", 𝔵𝔪𝔪28 => "xmm28", 𝔵𝔪𝔪29 => "xmm29", 𝔵𝔪𝔪30 => "xmm30", 𝔵𝔪𝔪31 => "xmm31");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86:
        𝔷𝔪𝔪0 => "zmm0", 𝔷𝔪𝔪1 => "zmm1", 𝔷𝔪𝔪2 => "zmm2", 𝔷𝔪𝔪3 => "zmm3", 𝔷𝔪𝔪4 => "zmm4", 𝔷𝔪𝔪5 => "zmm5", 𝔷𝔪𝔪6 => "zmm6", 𝔷𝔪𝔪7 => "zmm7");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64:
        𝔷𝔪𝔪0 => "zmm0", 𝔷𝔪𝔪1 => "zmm1", 𝔷𝔪𝔪2 => "zmm2", 𝔷𝔪𝔪3 => "zmm3", 𝔷𝔪𝔪4 => "zmm4", 𝔷𝔪𝔪5 => "zmm5",
        𝔷𝔪𝔪6 => "zmm6", 𝔷𝔪𝔪7 => "zmm7", 𝔷𝔪𝔪8 => "zmm8", 𝔷𝔪𝔪9 => "zmm9", 𝔷𝔪𝔪10 => "zmm10", 𝔷𝔪𝔪11 => "zmm11",
        𝔷𝔪𝔪12 => "zmm12", 𝔷𝔪𝔪13 => "zmm13", 𝔷𝔪𝔪14 => "zmm14", 𝔷𝔪𝔪15 => "zmm15", 𝔷𝔪𝔪16 => "zmm16",
        𝔷𝔪𝔪17 => "zmm17", 𝔷𝔪𝔪18 => "zmm18", 𝔷𝔪𝔪19 => "zmm19", 𝔷𝔪𝔪20 => "zmm20", 𝔷𝔪𝔪21 => "zmm21",
        𝔷𝔪𝔪22 => "zmm22", 𝔷𝔪𝔪23 => "zmm23", 𝔷𝔪𝔪24 => "zmm24", 𝔷𝔪𝔪25 => "zmm25", 𝔷𝔪𝔪26 => "zmm26",
        𝔷𝔪𝔪27 => "zmm27", 𝔷𝔪𝔪28 => "zmm28", 𝔷𝔪𝔪29 => "zmm29", 𝔷𝔪𝔪30 => "zmm30", 𝔷𝔪𝔪31 => "zmm31");
}

use super::{super::{𝒆𝒎𝒊𝒕𝒕𝒆𝒓,𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓},𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀};

struct 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec<u8>,
}

impl 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    pub const fn new() -> Self {
        𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
            𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec::<u8>::new(),
        }
    }
}

impl<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ> 𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for &'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ mut 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = ();
    #[inline(always)]
    fn combine_results(_: &mut (), _: ()) {
    }
}

impl<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ> 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for &'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ mut 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    #[inline(always)]
    fn emit_u8(&mut self, value: u8) -> Result<(), ()> {
        self.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.push(value);
        Ok(())
    }
}

#[allow(non_upper_case_globals)]
pub(crate) trait 𝒂𝒅𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<const 𝓸𝓹𝓬𝓸𝓭𝓮: u8> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

#[allow(non_upper_case_globals)]
impl<
    𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓹𝓻𝓮𝓯𝓲𝔁,
    𝓭𝓪𝓽𝓪_𝓼𝓲𝔃𝓮_𝓹𝓻𝓮𝓯𝓲𝔁,
    𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮_𝓹𝓻𝓮𝓯𝓲𝔁,
    𝓻𝓮𝓹ₓ_𝓹𝓻𝓮𝓯𝓲𝔁,
    𝓵𝓸𝓬𝓴_𝓹𝓻𝓮𝓯𝓲𝔁,
    𝔁𝟬𝗙_𝓹𝓻𝓮𝓯𝓲𝔁,
    𝔁𝟯𝘅_𝓹𝓻𝓮𝓯𝓲𝔁,
    𝓲𝓶𝓶𝓮𝓭𝓲𝓪𝓽𝓮_𝓸𝓹𝓬𝓸𝓭𝓮,
    const 𝓸𝓹𝓬𝓸𝓭𝓮: u8,
>
𝒂𝒅𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<𝓸𝓹𝓬𝓸𝓭𝓮>
    for 𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀::𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓹𝓻𝓮𝓯𝓲𝔁,
        𝓭𝓪𝓽𝓪_𝓼𝓲𝔃𝓮_𝓹𝓻𝓮𝓯𝓲𝔁,
        𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮_𝓹𝓻𝓮𝓯𝓲𝔁,
        𝓻𝓮𝓹ₓ_𝓹𝓻𝓮𝓯𝓲𝔁,
        𝓵𝓸𝓬𝓴_𝓹𝓻𝓮𝓯𝓲𝔁,
        𝔁𝟬𝗙_𝓹𝓻𝓮𝓯𝓲𝔁,
        𝔁𝟯𝘅_𝓹𝓻𝓮𝓯𝓲𝔁,
        𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀::𝐧𝐨_𝐩𝐫𝐞𝐟𝐢𝐱,
        𝓲𝓶𝓶𝓮𝓭𝓲𝓪𝓽𝓮_𝓸𝓹𝓬𝓸𝓭𝓮,
    >
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀::𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓹𝓻𝓮𝓯𝓲𝔁,
        𝓭𝓪𝓽𝓪_𝓼𝓲𝔃𝓮_𝓹𝓻𝓮𝓯𝓲𝔁,
        𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮_𝓹𝓻𝓮𝓯𝓲𝔁,
        𝓻𝓮𝓹ₓ_𝓹𝓻𝓮𝓯𝓲𝔁,
        𝓵𝓸𝓬𝓴_𝓹𝓻𝓮𝓯𝓲𝔁,
        𝔁𝟬𝗙_𝓹𝓻𝓮𝓯𝓲𝔁,
        𝔁𝟯𝘅_𝓹𝓻𝓮𝓯𝓲𝔁,
        𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞<𝓸𝓹𝓬𝓸𝓭𝓮>,
        𝓲𝓶𝓶𝓮𝓭𝓲𝓪𝓽𝓮_𝓸𝓹𝓬𝓸𝓭𝓮,
    >;
}


#[test]
fn test_add_𝔞𝔩_𝔞𝔩() {
    use super::super::𝗶𝗮𝟯𝟮::{𝑪𝑷𝑼,𝑨𝒅𝒅};
    type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ = super::super::𝗶𝗮𝟯𝟮::𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16;
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    super::super::𝗶𝗮𝟯𝟮::𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16::with(&mut raw_emitter)
        .add((𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔞𝔩, 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔞𝔩))
        .expect("Testing assembler");
    assert_eq!(&[0x00, 0xc0], &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]);
}

#[test]
fn test_add_𝔞𝔩_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_16ᵇⁱᵗ_𝔟𝔵_𝔰𝔦() {
    use super::super::𝗶𝗮𝟯𝟮::{𝑪𝑷𝑼,𝑨𝒅𝒅};
    type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ = super::super::𝗶𝗮𝟯𝟮::𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16;
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    super::super::𝗶𝗮𝟯𝟮::𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16::with(&mut raw_emitter)
        .add((
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔞𝔩,
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔞𝔡𝔡𝔯𝔢𝔰𝔰_16ᵇⁱᵗ
                .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔟𝔵)
                .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔰𝔦)
                .with_disp(0x1234i16),
        ))
        .expect("Testing assembler");
    assert_eq!(
        &[0x02, 0x80, 0x34, 0x12],
        &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
    );
}

#[test]
fn test_add_𝔞𝔩_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ_𝔢𝔰𝔭_𝔢𝔟𝔭() {
    use super::super::𝗶𝗮𝟯𝟮::{𝑪𝑷𝑼,𝑨𝒅𝒅};
    type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ = super::super::𝗶𝗮𝟯𝟮::𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16;
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    super::super::𝗶𝗮𝟯𝟮::𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16::with(&mut raw_emitter)
        .add((
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔞𝔩,
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ
                .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔢𝔰𝔭)
                .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔢𝔟𝔭)
                .with_disp(0x12345678),
        ))
        .expect("Testing assembler");
    assert_eq!(
        &[0x67, 0x02, 0x84, 0x2c, 0x78, 0x56, 0x34, 0x12],
        &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
    );
}

#[test]
fn test_add_𝔞𝔩_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_64ᵇⁱᵗ_𝔢𝔰𝔭_𝔢𝔟𝔭() {
    use super::super::𝘅𝟴𝟲_𝟲𝟰::{𝑪𝑷𝑼,𝑨𝒅𝒅};
    type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ = super::super::𝘅𝟴𝟲_𝟲𝟰::𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫;
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::with(&mut raw_emitter)
        .add((
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔞𝔩,
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ
                .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔰𝔭)
                .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔟𝔭)
                .with_disp(0x12345678),
        ))
        .expect("Testing assembler");
    assert_eq!(
        &[0x67, 0x02, 0x84, 0x2c, 0x78, 0x56, 0x34, 0x12],
        &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
    );
}

// This takes takes few second without miri but few hours when miri is used.
// And there are no unsafe code in it so just skip it on miri.
#[cfg(all(not(miri),feature = "slow-tests"))]
#[test]
fn test_emit_legacy_instruction_sanity() {
    use super::{𝗮𝗱𝗱𝗿𝗲𝘀𝘀,𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿};

    macro_rules! 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙 {
        ($𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮:ty[𝔫𝔬𝔫𝔢, $($𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮:ident),*]) => {
            [None, $(Some(<$𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>::$𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮)),*]
        };
        ($𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮:ty[$($𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮:ident),*]) => {
            [$(<$𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>::$𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮),*]
        }
    }

    macro_rules! 𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗 {
        ($𝓫𝓪𝓼𝓮_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷:ident,
         [$($𝓭𝓮𝓻𝓲𝓿𝓮𝓭_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷:ident [$($𝓮𝔁𝓽𝓻𝓪_𝓮𝓶𝓲𝓽:tt)*] [$($𝓮𝔁𝓽𝓻𝓪_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼:tt)*]),*],
         $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident, $𝓻𝓶_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident
        ) => {
            $(
                let emitter1 = &mut 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
                let emitter2 = &mut 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
                <𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 as 𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀::𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
                    <𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀::𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 as 𝒂𝒅𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<0x00>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::
                    $𝓫𝓪𝓼𝓮_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷(
                        emitter1,
                            $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                            $𝓻𝓶_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽)
                    .expect("Testing assembler");
                emitter1.$($𝓮𝔁𝓽𝓻𝓪_𝓮𝓶𝓲𝓽)*.expect("Testing assembler");
                <𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 as 𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀::𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
                    <𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀::𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 as 𝒂𝒅𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<0x00>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::
                    $𝓭𝓮𝓻𝓲𝓿𝓮𝓭_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷(
                        emitter2,
                            $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                            $𝓻𝓶_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                            $($𝓮𝔁𝓽𝓻𝓪_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼)*)
                    .expect("Testing assembler");
                assert_eq!(
                    &emitter1.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..emitter1.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()],
                    &emitter2.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..emitter2.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
                );
             )*
        };
        ($𝓫𝓪𝓼𝓮_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷:ident,
         [$($𝓭𝓮𝓻𝓲𝓿𝓮𝓭_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷:ident [$($𝓮𝔁𝓽𝓻𝓪_𝓮𝓶𝓲𝓽:tt)*] [$($𝓮𝔁𝓽𝓻𝓪_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼:tt)*]),*], $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident,
         𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<$𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident $𝓫𝓪𝓼𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident $𝓲𝓷𝓭𝓮𝔁_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident $𝓭𝓲𝓼𝓹_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident>
        ) => {
            $(
                let emitter1 = &mut 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
                let emitter2 = &mut 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
                <𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 as 𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀::𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
                    <𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀::𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 as 𝒂𝒅𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<0x00>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::
                    $𝓫𝓪𝓼𝓮_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷(
                        emitter1,
                            $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                            𝗮𝗱𝗱𝗿𝗲𝘀𝘀::𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,
                                                    𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                                                    𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                                                    i16,
                                                    0> {
                                𝗌𝖾𝗀𝗆𝖾𝗇𝗍: $𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝖻𝖺𝗌𝖾: $𝓫𝓪𝓼𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝗂𝗇𝖽𝖾𝗑: $𝓲𝓷𝓭𝓮𝔁_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝖽𝗂𝗌𝗉: $𝓭𝓲𝓼𝓹_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽
                            })
                    .expect("Testing assembler");
                emitter1.$($𝓮𝔁𝓽𝓻𝓪_𝓮𝓶𝓲𝓽)*.expect("Testing assembler");
                <𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 as 𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀::𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
                    <𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀::𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 as 𝒂𝒅𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<0x00>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::
                    $𝓭𝓮𝓻𝓲𝓿𝓮𝓭_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷(
                        emitter2,
                            $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                            𝗮𝗱𝗱𝗿𝗲𝘀𝘀::𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,
                                                    𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                                                    𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                                                    i16,
                                                    0> {
                                𝗌𝖾𝗀𝗆𝖾𝗇𝗍: $𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝖻𝖺𝗌𝖾: $𝓫𝓪𝓼𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝗂𝗇𝖽𝖾𝗑: $𝓲𝓷𝓭𝓮𝔁_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝖽𝗂𝗌𝗉: $𝓭𝓲𝓼𝓹_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽
                            },
                            $($𝓮𝔁𝓽𝓻𝓪_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼)*)
                    .expect("Testing assembler");
                assert_eq!(
                    &emitter1.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..emitter1.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()],
                    &emitter2.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..emitter2.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
                );
             )*
        };
        ($𝓫𝓪𝓼𝓮_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷:ident,
         [$($𝓭𝓮𝓻𝓲𝓿𝓮𝓭_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷:ident [$($𝓮𝔁𝓽𝓻𝓪_𝓮𝓶𝓲𝓽:tt)*] [$($𝓮𝔁𝓽𝓻𝓪_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼:tt)*]),*],
         $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮:ty, $𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮:ty, $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident,
         𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<$𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident $𝓫𝓪𝓼𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident $𝓲𝓷𝓭𝓮𝔁_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident $𝓼𝓬𝓪𝓵𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident $𝓭𝓲𝓼𝓹_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident>
        ) => {
            $(
                let emitter1 = &mut 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
                let emitter2 = &mut 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
                <𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 as 𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀::𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
                    <𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀::𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 as 𝒂𝒅𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<0x00>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::
                    $𝓫𝓪𝓼𝓮_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷(
                        emitter1,
                            $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                            𝗮𝗱𝗱𝗿𝗲𝘀𝘀::𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,
                                                   $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
                                                   $𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
                                                   𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
                                                   i32,
                                                   0> {
                                𝗌𝖾𝗀𝗆𝖾𝗇𝗍: $𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝖻𝖺𝗌𝖾: $𝓫𝓪𝓼𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝗂𝗇𝖽𝖾𝗑: $𝓲𝓷𝓭𝓮𝔁_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝗌𝖼𝖺𝗅𝖾: $𝓼𝓬𝓪𝓵𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝖽𝗂𝗌𝗉: $𝓭𝓲𝓼𝓹_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽
                            })
                    .expect("Testing assembler");
                emitter1.$($𝓮𝔁𝓽𝓻𝓪_𝓮𝓶𝓲𝓽)*.expect("Testing assembler");
                <𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 as 𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀::𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
                    <𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀::𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 as 𝒂𝒅𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<0x00>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::
                    $𝓭𝓮𝓻𝓲𝓿𝓮𝓭_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷(
                        emitter2,
                            $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                            𝗮𝗱𝗱𝗿𝗲𝘀𝘀::𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086,
                                                   $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
                                                   $𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
                                                   𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
                                                   i32,
                                                   0> {
                                𝗌𝖾𝗀𝗆𝖾𝗇𝗍: $𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝖻𝖺𝗌𝖾: $𝓫𝓪𝓼𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝗂𝗇𝖽𝖾𝗑: $𝓲𝓷𝓭𝓮𝔁_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝗌𝖼𝖺𝗅𝖾: $𝓼𝓬𝓪𝓵𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝖽𝗂𝗌𝗉: $𝓭𝓲𝓼𝓹_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽
                            },
                            $($𝓮𝔁𝓽𝓻𝓪_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼)*)
                    .expect("Testing assembler");
                assert_eq!(
                    &emitter1.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..emitter1.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()],
                    &emitter2.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..emitter2.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
                );
             )*
        }
    }

    for 𝗋𝖾𝗀 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
            [𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔰𝔭, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦, 𝔯8𝔡, 𝔯9𝔡, 𝔯10𝔡, 𝔯11𝔡, 𝔯12𝔡, 𝔯13𝔡, 𝔯14𝔡, 𝔯15𝔡]) {
        for 𝗋𝗆 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
                [𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔰𝔭, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦, 𝔯8𝔡, 𝔯9𝔡, 𝔯10𝔡, 𝔯11𝔡, 𝔯12𝔡, 𝔯13𝔡, 𝔯14𝔡, 𝔯15𝔡]) {
            𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗!(
                emit_legacy_reg_rm_instruction,
                [   emit_legacy_reg_rm_instruction_with_u8 [emit_u8(0xf1)] [0xf1],
                    emit_legacy_reg_rm_instruction_with_i8 [emit_u8(0xf1)] [-15],
                    emit_legacy_reg_rm_instruction_with_2ₓu8 [emit_u16(0xf2f1)] [0xf1, 0xf2],
                    emit_legacy_reg_rm_instruction_with_2ₓi8 [emit_u16(0xf2f1)] [-15, -14],
                    emit_legacy_reg_rm_instruction_with_u16 [emit_u16(0xf2f1)] [0xf2f1],
                    emit_legacy_reg_rm_instruction_with_i16 [emit_u16(0xf2f1)] [-3343],
                    emit_legacy_reg_rm_instruction_with_4ₓu8 [emit_u32(0xf4f3f2f1)] [0xf1, 0xf2, 0xf3, 0xf4],
                    emit_legacy_reg_rm_instruction_with_4ₓi8 [emit_u32(0xf4f3f2f1)] [-15, -14, -13, -12],
                    emit_legacy_reg_rm_instruction_with_u32 [emit_u32(0xf4f3f2f1)] [0xf4f3f2f1],
                    emit_legacy_reg_rm_instruction_with_i32 [emit_u32(0xf4f3f2f1)] [-185339151]],
                𝗋𝖾𝗀,
                𝗋𝗆
            );
            𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗!(
                emit_legacy_reg_rm_instruction_with_rex8,
                [   emit_legacy_reg_rm_instruction_with_rex8_and_u8 [emit_u8(0xf1)] [0xf1],
                    emit_legacy_reg_rm_instruction_with_rex8_and_i8 [emit_u8(0xf1)] [-15],
                    emit_legacy_reg_rm_instruction_with_rex8_and_2ₓu8 [emit_u16(0xf2f1)] [0xf1, 0xf2],
                    emit_legacy_reg_rm_instruction_with_rex8_and_2ₓi8 [emit_u16(0xf2f1)] [-15, -14],
                    emit_legacy_reg_rm_instruction_with_rex8_and_u16 [emit_u16(0xf2f1)] [0xf2f1],
                    emit_legacy_reg_rm_instruction_with_rex8_and_i16 [emit_u16(0xf2f1)] [-3343],
                    emit_legacy_reg_rm_instruction_with_rex8_and_4ₓu8 [emit_u32(0xf4f3f2f1)] [0xf1, 0xf2, 0xf3, 0xf4],
                    emit_legacy_reg_rm_instruction_with_rex8_and_4ₓi8 [emit_u32(0xf4f3f2f1)] [-15, -14, -13, -12],
                    emit_legacy_reg_rm_instruction_with_rex8_and_u32 [emit_u32(0xf4f3f2f1)] [0xf4f3f2f1],
                    emit_legacy_reg_rm_instruction_with_rex8_and_i32 [emit_u32(0xf4f3f2f1)] [-185339151]],
                𝗋𝖾𝗀,
                𝗋𝗆
            );
            𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗!(
                emit_legacy_reg_rm_instruction_with_rexw,
                [   emit_legacy_reg_rm_instruction_with_rexw_and_u8 [emit_u8(0xf1)] [0xf1],
                    emit_legacy_reg_rm_instruction_with_rexw_and_i8 [emit_u8(0xf1)] [-15],
                    emit_legacy_reg_rm_instruction_with_rexw_and_2ₓu8 [emit_u16(0xf2f1)] [0xf1, 0xf2],
                    emit_legacy_reg_rm_instruction_with_rexw_and_2ₓi8 [emit_u16(0xf2f1)] [-15, -14],
                    emit_legacy_reg_rm_instruction_with_rexw_and_u16 [emit_u16(0xf2f1)] [0xf2f1],
                    emit_legacy_reg_rm_instruction_with_rexw_and_i16 [emit_u16(0xf2f1)] [-3343],
                    emit_legacy_reg_rm_instruction_with_rexw_and_4ₓu8 [emit_u32(0xf4f3f2f1)] [0xf1, 0xf2, 0xf3, 0xf4],
                    emit_legacy_reg_rm_instruction_with_rexw_and_4ₓi8 [emit_u32(0xf4f3f2f1)] [-15, -14, -13, -12],
                    emit_legacy_reg_rm_instruction_with_rexw_and_u32 [emit_u32(0xf4f3f2f1)] [0xf4f3f2f1],
                    emit_legacy_reg_rm_instruction_with_rexw_and_i32 [emit_u32(0xf4f3f2f1)] [-185339151]],
                𝗋𝖾𝗀,
                𝗋𝗆
            );
        }
    }

    for 𝗋𝖾𝗀 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 [𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔰𝔭, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦]) {
        for 𝗌𝖾𝗀𝗆𝖾𝗇𝗍 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086 [𝔫𝔬𝔫𝔢, 𝔢𝔰, 𝔠𝔰, 𝔰𝔰, 𝔡𝔰, 𝔣𝔰, 𝔤𝔰]) {
            for 𝖻𝖺𝗌𝖾 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ [𝔫𝔬𝔫𝔢, 𝔟𝔵, 𝔟𝔭]) {
                for 𝗂𝗇𝖽𝖾𝗑 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ [𝔫𝔬𝔫𝔢, 𝔰𝔦, 𝔡𝔦]) {
                    for 𝖽𝗂𝗌𝗉 in [0, 0x11, 0x1234] {
                        𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗!(
                            emit_legacy_reg_address_8086_memory_instruction,
                            [   emit_legacy_reg_address_8086_memory_instruction_with_u8 [emit_u8(0xf1)] [0xf1],
                                emit_legacy_reg_address_8086_memory_instruction_with_i8 [emit_u8(0xf1)] [-15],
                                emit_legacy_reg_address_8086_memory_instruction_with_2ₓu8 [emit_u16(0xf2f1)] [0xf1, 0xf2],
                                emit_legacy_reg_address_8086_memory_instruction_with_2ₓi8 [emit_u16(0xf2f1)] [-15, -14],
                                emit_legacy_reg_address_8086_memory_instruction_with_u16 [emit_u16(0xf2f1)] [0xf2f1],
                                emit_legacy_reg_address_8086_memory_instruction_with_i16 [emit_u16(0xf2f1)] [-3343],
                                emit_legacy_reg_address_8086_memory_instruction_with_4ₓu8
                                    [emit_u32(0xf4f3f2f1)] [0xf1, 0xf2, 0xf3, 0xf4],
                                emit_legacy_reg_address_8086_memory_instruction_with_4ₓi8
                                    [emit_u32(0xf4f3f2f1)] [-15, -14, -13, -12],
                                emit_legacy_reg_address_8086_memory_instruction_with_u32 [emit_u32(0xf4f3f2f1)] [0xf4f3f2f1],
                                emit_legacy_reg_address_8086_memory_instruction_with_i32 [emit_u32(0xf4f3f2f1)] [-185339151]],
                            𝗋𝖾𝗀,
                            𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝗌𝖾𝗀𝗆𝖾𝗇𝗍 𝖻𝖺𝗌𝖾 𝗂𝗇𝖽𝖾𝗑 𝖽𝗂𝗌𝗉>);
                    }
                }
            }
        }
    }

    for 𝗋𝖾𝗀 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 [𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔰𝔭, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦]) {
        for 𝗌𝖾𝗀𝗆𝖾𝗇𝗍 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086 [𝔫𝔬𝔫𝔢, 𝔢𝔰, 𝔠𝔰, 𝔰𝔰, 𝔡𝔰, 𝔣𝔰, 𝔤𝔰]) {
            for 𝖻𝖺𝗌𝖾 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 [𝔫𝔬𝔫𝔢, 𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔰𝔭, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦]) {
                for 𝗂𝗇𝖽𝖾𝗑 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386
                        [𝔫𝔬𝔫𝔢, 𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔦𝔷, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦]) {
                    for 𝗌𝖼𝖺𝗅𝖾 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞 [𝔵1, 𝔵2, 𝔵4, 𝔵8]) {
                        for 𝖽𝗂𝗌𝗉 in [0, 0x11, 0x1234, 0x12345678] {
                            𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗!(
                                emit_legacy_reg_address_80386_memory_instruction,
                                [   emit_legacy_reg_address_80386_memory_instruction_with_u8 [emit_u8(0xf1)] [0xf1],
                                    emit_legacy_reg_address_80386_memory_instruction_with_i8 [emit_u8(0xf1)] [-15],
                                    emit_legacy_reg_address_80386_memory_instruction_with_2ₓu8 [emit_u16(0xf2f1)] [0xf1, 0xf2],
                                    emit_legacy_reg_address_80386_memory_instruction_with_2ₓi8 [emit_u16(0xf2f1)] [-15, -14],
                                    emit_legacy_reg_address_80386_memory_instruction_with_u16 [emit_u16(0xf2f1)] [0xf2f1],
                                    emit_legacy_reg_address_80386_memory_instruction_with_i16 [emit_u16(0xf2f1)] [-3343],
                                    emit_legacy_reg_address_80386_memory_instruction_with_4ₓu8
                                        [emit_u32(0xf4f3f2f1)] [0xf1, 0xf2, 0xf3, 0xf4],
                                    emit_legacy_reg_address_80386_memory_instruction_with_4ₓi8
                                        [emit_u32(0xf4f3f2f1)] [-15, -14, -13, -12],
                                    emit_legacy_reg_address_80386_memory_instruction_with_u32 [emit_u32(0xf4f3f2f1)] [0xf4f3f2f1],
                                    emit_legacy_reg_address_80386_memory_instruction_with_i32 [emit_u32(0xf4f3f2f1)] [-185339151]],
                                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
                                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
                                𝗋𝖾𝗀,
                                𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝗌𝖾𝗀𝗆𝖾𝗇𝗍 𝖻𝖺𝗌𝖾 𝗂𝗇𝖽𝖾𝗑 𝗌𝖼𝖺𝗅𝖾 𝖽𝗂𝗌𝗉>);
                        }
                    }
                }
            }
        }
    }

    for 𝗋𝖾𝗀 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
            [𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔰𝔭, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦, 𝔯8𝔡, 𝔯9𝔡, 𝔯10𝔡, 𝔯11𝔡, 𝔯12𝔡, 𝔯13𝔡, 𝔯14𝔡, 𝔯15𝔡]) {
        for 𝗌𝖾𝗀𝗆𝖾𝗇𝗍 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8086 [𝔫𝔬𝔫𝔢, 𝔢𝔰, 𝔠𝔰, 𝔰𝔰, 𝔡𝔰, 𝔣𝔰, 𝔤𝔰]) {
            for 𝖻𝖺𝗌𝖾 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
                    [𝔫𝔬𝔫𝔢, 𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔰𝔭, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦, 𝔯8𝔡, 𝔯9𝔡, 𝔯10𝔡, 𝔯11𝔡, 𝔯12𝔡, 𝔯13𝔡, 𝔯14𝔡, 𝔯15𝔡]) {
                for 𝗂𝗇𝖽𝖾𝗑 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 [
                        𝔫𝔬𝔫𝔢, 𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔦𝔷, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦, 𝔯8𝔡, 𝔯9𝔡, 𝔯10𝔡, 𝔯11𝔡, 𝔯12𝔡, 𝔯13𝔡, 𝔯14𝔡, 𝔯15𝔡]) {
                    for 𝗌𝖼𝖺𝗅𝖾 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞 [𝔵1, 𝔵2, 𝔵4, 𝔵8]) {
                        for 𝖽𝗂𝗌𝗉 in [0, 0x11, 0x1234, 0x12345678] {
                            𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗!(
                                emit_legacy_reg_address_ₓ86_64_memory_instruction,
                                [   emit_legacy_reg_address_ₓ86_64_memory_instruction_with_u8 [emit_u8(0xf1)] [0xf1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_i8 [emit_u8(0xf1)] [-15],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_2ₓu8 [emit_u16(0xf2f1)] [0xf1, 0xf2],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_2ₓi8 [emit_u16(0xf2f1)] [-15, -14],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_u16 [emit_u16(0xf2f1)] [0xf2f1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_i16 [emit_u16(0xf2f1)] [-3343],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_4ₓu8
                                        [emit_u32(0xf4f3f2f1)] [0xf1, 0xf2, 0xf3, 0xf4],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_4ₓi8
                                        [emit_u32(0xf4f3f2f1)] [-15, -14, -13, -12],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_u32 [emit_u32(0xf4f3f2f1)] [0xf4f3f2f1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_i32 [emit_u32(0xf4f3f2f1)] [-185339151]],
                                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
                                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
                                𝗋𝖾𝗀,
                                𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝗌𝖾𝗀𝗆𝖾𝗇𝗍 𝖻𝖺𝗌𝖾 𝗂𝗇𝖽𝖾𝗑 𝗌𝖼𝖺𝗅𝖾 𝖽𝗂𝗌𝗉>);
                            𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗!(
                                emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8,
                                [   emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_u8 [emit_u8(0xf1)] [0xf1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_i8 [emit_u8(0xf1)] [-15],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_2ₓu8 [emit_u16(0xf2f1)] [0xf1, 0xf2],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_2ₓi8 [emit_u16(0xf2f1)] [-15, -14],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_u16 [emit_u16(0xf2f1)] [0xf2f1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_i16 [emit_u16(0xf2f1)] [-3343],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_4ₓu8
                                        [emit_u32(0xf4f3f2f1)] [0xf1, 0xf2, 0xf3, 0xf4],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_4ₓi8
                                        [emit_u32(0xf4f3f2f1)] [-15, -14, -13, -12],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_u32 [emit_u32(0xf4f3f2f1)] [0xf4f3f2f1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_i32 [emit_u32(0xf4f3f2f1)] [-185339151]],
                                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
                                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
                                𝗋𝖾𝗀,
                                𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝗌𝖾𝗀𝗆𝖾𝗇𝗍 𝖻𝖺𝗌𝖾 𝗂𝗇𝖽𝖾𝗑 𝗌𝖼𝖺𝗅𝖾 𝖽𝗂𝗌𝗉>);
                            𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗!(
                                emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw,
                                [   emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_u8 [emit_u8(0xf1)] [0xf1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_i8 [emit_u8(0xf1)] [-15],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_2ₓu8 [emit_u16(0xf2f1)] [0xf1, 0xf2],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_2ₓi8 [emit_u16(0xf2f1)] [-15, -14],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_u16 [emit_u16(0xf2f1)] [0xf2f1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_i16 [emit_u16(0xf2f1)] [-3343],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_4ₓu8
                                        [emit_u32(0xf4f3f2f1)] [0xf1, 0xf2, 0xf3, 0xf4],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_4ₓi8
                                        [emit_u32(0xf4f3f2f1)] [-15, -14, -13, -12],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_u32 [emit_u32(0xf4f3f2f1)] [0xf4f3f2f1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_i32 [emit_u32(0xf4f3f2f1)] [-185339151]],
                                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
                                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
                                𝗋𝖾𝗀,
                                𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝗌𝖾𝗀𝗆𝖾𝗇𝗍 𝖻𝖺𝗌𝖾 𝗂𝗇𝖽𝖾𝗑 𝗌𝖼𝖺𝗅𝖾 𝖽𝗂𝗌𝗉>);
                        }
                    }
                }
            }
        }
    }
}
