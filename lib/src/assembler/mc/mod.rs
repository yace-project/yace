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

// Machine code is something that deals with labels and can also be used as 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓.
// Couple of [possible] examples: 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞 (code is intended to be used in address space of current process),
// 𝐫𝐞𝐬𝐭𝐫𝐢𝐜𝐭𝐞𝐝_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐬𝐩𝐚𝐜𝐞_𝐜𝐨𝐝𝐞 (code uses address pointers smaller then current process address pointers and would live
// in a sandbox), 𝐞𝐱𝐭𝐞𝐫𝐧𝐚𝐥_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐜𝐨𝐝𝐞 (code is not being emitted, instead assembler text would be emitted) and so on.
pub trait 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆 {
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;
    type 𝐥𝐚𝐛𝐞𝐥:
        Clone +
        core::ops::Add<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Neg<Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Not<Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shl<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shl<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shl<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shr<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shr<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shr<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧>;
    type 𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧:
        core::ops::Add<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Neg<Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Not<Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shl<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shl<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shl<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shr<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shr<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shr<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧>;
    type 𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭:
        Clone +
        core::ops::Add<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭> +
        core::ops::BitAnd<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭> +
        core::ops::BitOr<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭> +
        core::ops::BitXor<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭> +
        core::ops::Div<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭> +
        core::ops::Mul<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭> +
        core::ops::Neg<Output = Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭> +
        core::ops::Not<Output = Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭> +
        core::ops::Rem<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭> +
        core::ops::Shl<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shl<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shl<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭> +
        core::ops::Shr<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shr<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shr<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭> +
        core::ops::Sub<Self::𝐥𝐚𝐛𝐞𝐥, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = Self::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭, Output = Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭>;
    type 𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤; // Note: code block is passive, you can only use it via 𝒄𝒐𝒅𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓.
    type 𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾> where Self: 'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ;

    fn new_label(&mut self) -> Self::𝐥𝐚𝐛𝐞𝐥;
    fn set_label(&mut self, lbl: Self::𝐥𝐚𝐛𝐞𝐥) -> Result<(),Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;

    fn new_code_block(&mut self) -> Self::𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤;
    fn with_code_block<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾>(self: &'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ mut Self,
                                                       𝖻𝗅𝗈𝖼𝗄: &'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾 mut Self::𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤)
        -> Self::𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾>;
    fn attach_code_block(&mut self, mut 𝖻𝗅𝗈𝖼𝗄: Self::𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤) -> Result<(),Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.attach_code_block_contents(&mut 𝖻𝗅𝗈𝖼𝗄)
    }
    // This function makes code block empty, but keeps allocated data structures.
    fn attach_code_block_contents(&mut self, 𝖻𝗅𝗈𝖼𝗄: &mut Self::𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤) -> Result<(),Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
}

pub trait 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 {
    type 𝐜𝐨𝐝𝐞: 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆;

    fn new_label(&mut self) -> <Self::𝐜𝐨𝐝𝐞 as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥;
    fn set_label(&mut self, lbl: <Self::𝐜𝐨𝐝𝐞 as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥) -> Result<(),<Self::𝐜𝐨𝐝𝐞 as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
}

#[path = "native_code.rs"]
pub mod 𝗻𝗮𝘁𝗶𝘃𝗲_𝗰𝗼𝗱𝗲;

#[path = "numbered_labels.rs"]
pub mod 𝗻𝘂𝗺𝗯𝗲𝗿𝗲𝗱_𝗹𝗮𝗯𝗲𝗹𝘀;
