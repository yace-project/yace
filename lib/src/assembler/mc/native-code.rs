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

// Machine code for the native code executed in the address space of Rust program.
#[derive(Clone, Debug, Default)]
pub struct 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> {
    𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄: 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤,
    // New label starts equal to 0, after set_label or set_data_label is called it becomes equal to !0usize and it receives actual
    // address in memory when “freeze” is called.
    𝗅𝖺𝖻𝖾𝗅𝗌: Vec<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>,
}

pub trait 𝒏𝒂𝒕𝒊𝒗𝒆_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 {
    #[allow(non_upper_case_globals)]
    const 𝔫𝔬_𝔞𝔡𝔡𝔯𝔢𝔰𝔰: Self;
    #[allow(non_upper_case_globals)]
    const 𝔷𝔢𝔯𝔬: Self;
    #[allow(non_upper_case_globals)]
    const 𝔬𝔫𝔢: Self;
    fn sar(self, rhs: Self) -> Self;
    fn shr(self, rhs: Self) -> Self;
}

impl 𝒏𝒂𝒕𝒊𝒗𝒆_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 for i16 {
    #[allow(non_upper_case_globals)]
    const 𝔫𝔬_𝔞𝔡𝔡𝔯𝔢𝔰𝔰: Self = !0;
    #[allow(non_upper_case_globals)]
    const 𝔷𝔢𝔯𝔬: Self = 0;
    #[allow(non_upper_case_globals)]
    const 𝔬𝔫𝔢: Self = 1;
    #[inline(always)]
    fn sar(self, rhs: Self) -> Self {
        self >> rhs
    }
    #[inline(always)]
    fn shr(self, rhs: Self) -> Self {
        ((self as u16) >> (rhs as u16)) as i16
    }
}

impl 𝒏𝒂𝒕𝒊𝒗𝒆_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 for i32 {
    #[allow(non_upper_case_globals)]
    const 𝔫𝔬_𝔞𝔡𝔡𝔯𝔢𝔰𝔰: Self = !0;
    #[allow(non_upper_case_globals)]
    const 𝔷𝔢𝔯𝔬: Self = 0;
    #[allow(non_upper_case_globals)]
    const 𝔬𝔫𝔢: Self = 1;
    #[inline(always)]
    fn sar(self, rhs: Self) -> Self {
        self >> rhs
    }
    #[inline(always)]
    fn shr(self, rhs: Self) -> Self {
        ((self as u32) >> (rhs as u32)) as i32
    }
}

impl 𝒏𝒂𝒕𝒊𝒗𝒆_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 for i64 {
    #[allow(non_upper_case_globals)]
    const 𝔫𝔬_𝔞𝔡𝔡𝔯𝔢𝔰𝔰: Self = !0;
    #[allow(non_upper_case_globals)]
    const 𝔷𝔢𝔯𝔬: Self = 0;
    #[allow(non_upper_case_globals)]
    const 𝔬𝔫𝔢: Self = 1;
    #[inline(always)]
    fn sar(self, rhs: Self) -> Self {
        self >> rhs
    }
    #[inline(always)]
    fn shr(self, rhs: Self) -> Self {
        ((self as u64) >> (rhs as u64)) as i64
    }
}

impl 𝒏𝒂𝒕𝒊𝒗𝒆_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 for isize {
    #[allow(non_upper_case_globals)]
    const 𝔫𝔬_𝔞𝔡𝔡𝔯𝔢𝔰𝔰: Self = !0;
    #[allow(non_upper_case_globals)]
    const 𝔷𝔢𝔯𝔬: Self = 0;
    #[allow(non_upper_case_globals)]
    const 𝔬𝔫𝔢: Self = 1;
    #[inline(always)]
    fn sar(self, rhs: Self) -> Self {
        self >> rhs
    }
    #[inline(always)]
    fn shr(self, rhs: Self) -> Self {
        ((self as usize) >> (rhs as usize)) as isize
    }
}

impl<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: 𝒏𝒂𝒕𝒊𝒗𝒆_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔> 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆 for 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>
where
    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>:
        Clone +
        core::ops::Add<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Neg<Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Not<Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧>,
    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧:
        Clone +
        core::ops::Add<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Neg<Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Not<Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧>,
    𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮:
        Copy +
        TryInto<i64> +
        TryInto<usize> +
        core::cmp::Eq +
        core::ops::Add<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Neg<Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Not<Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Rem<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shl<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Shl<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shl<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shr<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Shr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧>,
    i64: TryInto<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>,
    usize: TryInto<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>,
    <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<i64>>::Error: core::fmt::Debug,
    <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<usize>>::Error: core::fmt::Debug,
    <i64 as TryInto<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>>::Error: core::fmt::Debug,
    <usize as TryInto<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>>::Error: core::fmt::Debug,
{
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬;
    type 𝐥𝐚𝐛𝐞𝐥 = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭>;
    type 𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧;
    type 𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭 = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮;
    type 𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤 = 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤;
    type 𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾> = 𝐧𝐚𝐭𝐢𝐯𝐞_𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>
         where Self: 'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ;

    #[inline(always)]
    fn new_code_block(&mut self) -> Self::𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤 {
        Default::default()
    }
    #[inline(always)]
    fn with_code_block<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾>(self: &'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ mut Self,
                                                       𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄: &'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾 mut Self::𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤)
        -> Self::𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾>
    {
        𝐧𝐚𝐭𝐢𝐯𝐞_𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 { 𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄: self, 𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄 }
    }
    #[inline(always)]
    fn attach_code_block_contents(&mut self, 𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄: &mut Self::𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤)
        -> Result<(), <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
    {
        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.close_bytes_fragment();
        𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.close_bytes_fragment();
        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺);
        Ok(())
    }
}

impl<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: 𝒏𝒂𝒕𝒊𝒗𝒆_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔> 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>
where
    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>:
        Clone +
        core::ops::Add<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Neg<Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Not<Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧>,
    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧:
        Clone +
        core::ops::Add<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Neg<Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Not<Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧>,
    𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮:
        Copy +
        TryInto<i64> +
        TryInto<usize> +
        core::cmp::Eq +
        core::ops::Add<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Neg<Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Not<Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Rem<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shl<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Shl<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shl<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shr<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Shr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧>,
    i64: TryInto<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>,
    usize: TryInto<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>,
    <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<i64>>::Error: core::fmt::Debug,
    <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<usize>>::Error: core::fmt::Debug,
    <i64 as TryInto<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>>::Error: core::fmt::Debug,
    <usize as TryInto<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>>::Error: core::fmt::Debug
{
    type 𝐜𝐨𝐝𝐞 = 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>;

    #[inline(always)]
    fn new_label(&mut self) -> <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥 {
        let len: 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 = self.𝗅𝖺𝖻𝖾𝗅𝗌.len().try_into().expect("Label count would match address type");
        let lbl = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥::<<Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭>(len);
        self.𝗅𝖺𝖻𝖾𝗅𝗌.push(𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮::𝔫𝔬_𝔞𝔡𝔡𝔯𝔢𝔰𝔰);
        lbl
    }
    #[inline(always)]
    fn set_label(&mut self, lbl: <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥) -> Result<(), <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let label_id: usize = lbl.0.try_into().expect("Label number should fit into usize because it was made from usize initially");
        if self.𝗅𝖺𝖻𝖾𝗅𝗌.len() <= label_id {
            Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔦𝔡_𝔬𝔲𝔱_𝔬𝔣_𝔯𝔞𝔫𝔤𝔢)
        } else if self.𝗅𝖺𝖻𝖾𝗅𝗌[label_id] != 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮::𝔫𝔬_𝔞𝔡𝔡𝔯𝔢𝔰𝔰 {
            Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔞𝔩𝔯𝔢𝔞𝔡𝔶_𝔞𝔱𝔱𝔞𝔠𝔥𝔢𝔡)
        } else {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.close_bytes_fragment();
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&label_id.to_ne_bytes());
            self.𝗅𝖺𝖻𝖾𝗅𝗌[label_id] = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮::𝔫𝔬_𝔞𝔡𝔡𝔯𝔢𝔰𝔰;
            Ok(())
        }
    }
    #[inline(always)]
    fn emit_byte(&mut self, expr: <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧) -> Result<(), <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_byte::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_2byte(&mut self, expr: <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧) -> Result<(), <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_2byte::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_2byte_be(&mut self, expr: <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧) -> Result<(), <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_2byte_be::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_2byte_le(&mut self, expr: <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧) -> Result<(), <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_2byte_le::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_4byte(&mut self, expr: <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧) -> Result<(), <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_4byte::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_4byte_be(&mut self, expr: <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧) -> Result<(), <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_4byte_be::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_4byte_le(&mut self, expr: <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧) -> Result<(), <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_4byte_le::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_8byte(&mut self, expr: <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧) -> Result<(), <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_8byte::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_8byte_be(&mut self, expr: <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧) -> Result<(), <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_8byte_be::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_8byte_le(&mut self, expr: <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧) -> Result<(), <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_8byte_le::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
}

impl<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: 𝒏𝒂𝒕𝒊𝒗𝒆_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔> 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> {
    #[inline(always)]
    fn emit_u8(&mut self, value: u8) -> Result<(), Box<dyn std::error::Error>> {
        if self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.push(value);
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 1;
        } else {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.push(value);
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 1;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_u16(&mut self, value: u16) -> Result<(), Box<dyn std::error::Error>> {
        if self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 2;
        } else {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 2;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_u32(&mut self, value: u32) -> Result<(), Box<dyn std::error::Error>> {
        if self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 4;
        } else {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 4;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_u64(&mut self, value: u64) -> Result<(), Box<dyn std::error::Error>> {
        if self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 8;
        } else {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 8;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_u128(&mut self, value: u128) -> Result<(), Box<dyn std::error::Error>> {
        if self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 16;
        } else {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 16;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_u8_slice(&mut self, 𝗌𝗅𝗂𝖼𝖾: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if 𝗌𝗅𝗂𝖼𝖾.len() != 0 {
            if self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(𝗌𝗅𝗂𝖼𝖾);
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 𝗌𝗅𝗂𝖼𝖾.len();
            } else {
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(𝗌𝗅𝗂𝖼𝖾);
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 𝗌𝗅𝗂𝖼𝖾.len();
            }
        }
        Ok(())
    }
}

impl<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: 𝒏𝒂𝒕𝒊𝒗𝒆_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔> 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>
where Self: 𝒆𝒎𝒊𝒕𝒕𝒆𝒓<𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = (), 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = core::convert::Infallible>
{
    fn emit_u8(&mut self, value: u8) -> Result<(), core::convert::Infallible> {
        if self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.push(value);
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 1;
        } else {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.push(value);
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 1;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_bytes_u16(&mut self, value: u16) -> Result<(), core::convert::Infallible> {
        if self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 2;
        } else {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 2;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_bytes_u32(&mut self, value: u32) -> Result<(), core::convert::Infallible> {
        if self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 4;
        } else {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 4;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_bytes_u64(&mut self, value: u64) -> Result<(), core::convert::Infallible> {
        if self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 8;
        } else {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 8;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_bytes_u128(&mut self, value: u128) -> Result<(), core::convert::Infallible> {
        if self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 16;
        } else {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 16;
        }
        Ok(())
    }
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    fn emit_u8_array<const 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮: usize>(&mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮]) -> Result<(), core::convert::Infallible>
    where Self: 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, { 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮 }> {
        if 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮 != 0 {
            if self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮;
            } else {
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮;
            }
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_u8_slice(&mut self, 𝗌𝗅𝗂𝖼𝖾: &[u8]) -> Result<(), core::convert::Infallible> {
        if 𝗌𝗅𝗂𝖼𝖾.len() != 0 {
            if self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(𝗌𝗅𝗂𝖼𝖾);
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 𝗌𝗅𝗂𝖼𝖾.len();
            } else {
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(𝗌𝗅𝗂𝖼𝖾);
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 𝗌𝗅𝗂𝖼𝖾.len();
            }
        }
        Ok(())
    }
}

impl<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> 𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> {
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = core::convert::Infallible;
    fn combine_results(_: &mut (), _: ()) {
    }
}

impl<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: 𝒏𝒂𝒕𝒊𝒗𝒆_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔> 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>
where
    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>:
        Clone +
        core::ops::Add<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Neg<Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Not<Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧>,
    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧:
        Clone +
        core::ops::Add<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Neg<Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Not<Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧>,
    𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮:
        Copy +
        TryInto<i64> +
        TryInto<usize> +
        core::cmp::Eq +
        core::ops::Add<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Neg<Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Not<Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Rem<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shl<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Shl<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shl<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shr<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Shr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧>,
    i64: TryInto<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>,
    usize: TryInto<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>,
    <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<i64>>::Error: core::fmt::Debug,
    <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<usize>>::Error: core::fmt::Debug,
    <i64 as TryInto<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>>::Error: core::fmt::Debug,
    <usize as TryInto<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>>::Error: core::fmt::Debug,
{
    #[inline(always)]
    pub const fn new() -> Self {
        𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞 {
            𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄: 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤 { 𝖽𝖺𝗍𝖺: Vec::new(), 𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾: 0 },
            𝗅𝖺𝖻𝖾𝗅𝗌: Vec::new()
        }
    }
    // Calculate all offsets when final address of the start of the code is known.
    // First pass calculates all expressions with labels as zero (which, hopefully, produces smaller results; assemblers which can
    // not support zero immedate in some cases would need special treatment, but currently we have not such CPUs), subsequent
    // passed actually calculate values and compare values of labels to what what produced in previous pass.
    // If layout changes after first pass then there would be additional passes, up to extra_passes
    #[inline(always)]
    pub fn finalize(&mut self, start_address: 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, extra_passes: usize) -> Result<usize, <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.close_bytes_fragment();
        let address_minus_one: 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 = start_address + 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮::𝔫𝔬_𝔞𝔡𝔡𝔯𝔢𝔰𝔰;
        for label in self.𝗅𝖺𝖻𝖾𝗅𝗌.iter_mut() {
            *label = address_minus_one
        }
        // First pass: all labels are initialized to 0 (which, hopefully, produces smaller results; assemblers which can not
        // support zero immedate in some cases would need special treatment, but currently we have not such CPUs).
        let mut code_size: 𝐝𝐲𝐧_𝐜𝐨𝐝𝐞_𝐬𝐢𝐳𝐞_𝐜𝐨𝐮𝐧𝐭𝐞𝐫 = Default::default();
        let mut index: usize = 0;
        while index < self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.len() {
            let selector: [u8; core::mem::size_of::<usize>()] =
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[index..index+core::mem::size_of::<usize>()].try_into().unwrap();
            let selector = usize::from_ne_bytes(selector);
            index += core::mem::size_of::<usize>();
            match selector {
                0 => {
                    let lbl: [u8; core::mem::size_of::<usize>()] =
                        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[index..index+core::mem::size_of::<usize>()].try_into().unwrap();
                    let lbl = usize::from_ne_bytes(lbl);
                    let code_size =
                        code_size.accumulated_size().try_into().map_err(|_| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔱𝔬𝔬_𝔟𝔦𝔤)?;
                    self.𝗅𝖺𝖻𝖾𝗅𝗌[lbl] = start_address + code_size;
                    index += core::mem::size_of::<usize>();
                }
                chunk_size if chunk_size as isize > 0 => {
                    code_size.emit_u8_slice(&self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[index..index+chunk_size as usize])
                        .map_err(|err| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔯𝔯𝔬𝔯(err))?;
                    index = (index + chunk_size + (core::mem::align_of::<usize>()-1)) & !(core::mem::align_of::<usize>()-1);
                }
                emit_label_fn_info => {
                    let function: [u8; core::mem::size_of::<usize>()] =
                        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[index..index+core::mem::size_of::<usize>()].try_into().unwrap();
                    let function = usize::from_ne_bytes(function);
                    // SAFETY: we are retriveing 'static function pointer which was stored as properly aligned array of bytes.
                    #[cfg(not(miri))]
                    let function = unsafe {
                        core::mem::transmute::<
                            usize,
                            fn(
                                code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, function_arguments: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰, 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞>
                            ) -> Result<(), Box<dyn std::error::Error>>,
                        >(function)
                    };
                    #[cfg(miri)]
                    let function = {
                        let functions = 𝔪𝔦𝔯𝔦_𝔣𝔲𝔫𝔠𝔱𝔦𝔬𝔫𝔰.lock().unwrap();
                        let Some(ref functions) = *functions else { panic!("Internal error: map is none after creation"); };
                        let function = functions.get(&function).unwrap();
                        unsafe {
                            core::mem::transmute::<
                                fn(
                                    code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, function_arguments: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<i64, 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰, 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞>
                                ) -> Result<(), Box<dyn std::error::Error>>,
                                fn(
                                    code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, function_arguments: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰, 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞>
                                ) -> Result<(), Box<dyn std::error::Error>>,
                            >(*function)
                        }
                    };
                    index += core::mem::size_of::<usize>();
                    // SAFETY: An uninitialized `[use core::mem::MaybeUninit<_>; LEN]` is valid.
                    #[cfg(not(miri))]
                    let mut function_arguments = 𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬_𝐮𝐧𝐢𝐨𝐧::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> {
                        𝗎𝗇𝗂𝗇𝗂𝗍_𝖻𝗎𝖿𝖿𝖾𝗋: unsafe {
                            core::mem::MaybeUninit::<
                                [core::mem::MaybeUninit<u8>; 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰 * core::mem::size_of::<i64>() + 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞]
                            >::uninit().assume_init()
                        }
                    };
                    #[cfg(miri)]
                    let mut function_arguments = 𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬_𝐮𝐧𝐢𝐨𝐧::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> {
                        𝗋𝖺𝗐_𝖻𝗎𝖿𝖿𝖾𝗋: [0; 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰 * core::mem::size_of::<i64>() + 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞]
                    };
                    let lbls: usize = emit_label_fn_info as u8 as usize;
                    let extra: usize = (emit_label_fn_info >> 8) as u8 as usize;
                    for lbl in 0..lbls {
                        // SAFETY: we are writing values here before it may be overwritten.
                        unsafe {
                            function_arguments.𝗅𝖺𝖻𝖾𝗅𝗌_𝖺𝗇𝖽_𝗌𝗍𝖺𝖼𝗄.𝗅𝖺𝖻𝖾𝗅_𝗏𝖺𝗅𝗎𝖾𝗌[lbl] = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮::𝔷𝔢𝔯𝔬;
                        }
                    }
                    for _ in 0..lbls {
                        loop {
                            match self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[index] {
                                0 => {
                                    index += 1;
                                    break;
                                }
                                value if value > 0 && value < 64 => {
                                    index += 1 + (1 << (value & 7));
                                }
                                _ => {
                                    index += 1;
                                }
                            }
                        }
                    }
                    // SAFETY: copy to buffer withing allocated size.
                    if extra > 0 {
                        unsafe {
                            core::ptr::copy_nonoverlapping(&self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[index],
                            core::ptr::addr_of_mut!(function_arguments.𝗋𝖺𝗐_𝖻𝗎𝖿𝖿𝖾𝗋[lbls*core::mem::size_of::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>()]),
                            extra);
                        }
                    }
                    index = (index + extra + (core::mem::align_of::<usize>()-1)) & !(core::mem::align_of::<usize>()-1);

                    function(𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔠𝔬𝔲𝔫𝔱𝔢𝔯(&mut code_size),
                             // SAFETY: construction is symmetric to what inject_label_function does.
                             &unsafe {function_arguments.𝖿𝗎𝗇𝖼𝗍𝗂𝗈𝗇_𝖽𝖺𝗍𝖺}
                    ).map_err(|err| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔯𝔯𝔬𝔯(err))?;
                }
            }
        }
        // Additional passes with actual calculations of label expressions.
        for _ in 0..extra_passes {
            let mut layout_changed = false;

            let mut code_size: 𝐝𝐲𝐧_𝐜𝐨𝐝𝐞_𝐬𝐢𝐳𝐞_𝐜𝐨𝐮𝐧𝐭𝐞𝐫 = Default::default();
            let mut index: usize = 0;
            while (index as usize) < self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.len() {
                let selector: [u8; core::mem::size_of::<usize>()] =
                    self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[index..index+core::mem::size_of::<usize>()].try_into().unwrap();
                let selector = usize::from_ne_bytes(selector);
                index += core::mem::size_of::<usize>();
                match selector {
                    0 => {
                        let lbl: [u8; core::mem::size_of::<usize>()] =
                            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[index..index+core::mem::size_of::<usize>()].try_into().unwrap();
                        let code_size =
                            code_size.accumulated_size().try_into().map_err(|_| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔱𝔬𝔬_𝔟𝔦𝔤)?;
                        let lbl = usize::from_ne_bytes(lbl);
                        if self.𝗅𝖺𝖻𝖾𝗅𝗌[lbl] != start_address + code_size {
                            layout_changed = true;
                            self.𝗅𝖺𝖻𝖾𝗅𝗌[lbl] = start_address + code_size;
                        }
                        index += core::mem::size_of::<usize>();
                    }
                    chunk_size if chunk_size as isize > 0 => {
                        code_size.emit_u8_slice(&self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[index..index+chunk_size])
                            .map_err(|err| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔯𝔯𝔬𝔯(err))?;
                        index = (index + chunk_size + (core::mem::align_of::<usize>()-1)) & !(core::mem::align_of::<usize>()-1);
                    }
                    emit_label_fn_info => {
                        let function: [u8; core::mem::size_of::<usize>()] =
                            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[index..index+core::mem::size_of::<usize>()].try_into().unwrap();
                        let function = usize::from_ne_bytes(function);
                        // SAFETY: we are retriveing 'static function pointer which was stored as properly aligned array of bytes.
                        #[cfg(not(miri))]
                        let function = unsafe {
                            core::mem::transmute::<
                                usize,
                                fn(
                                    code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, function_arguments: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰, 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞>
                                ) -> Result<(), Box<dyn std::error::Error>>,
                            >(function)
                        };
                        #[cfg(miri)]
                        let function = {
                            let functions = 𝔪𝔦𝔯𝔦_𝔣𝔲𝔫𝔠𝔱𝔦𝔬𝔫𝔰.lock().unwrap();
                            let Some(ref functions) = *functions else { panic!("Internal error: map is none after creation"); };
                            let function = functions.get(&function).unwrap();
                            unsafe {
                                core::mem::transmute::<
                                    fn(
                                        code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, function_arguments: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<i64, 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰, 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞>
                                    ) -> Result<(), Box<dyn std::error::Error>>,
                                    fn(
                                        code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, function_arguments: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰, 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞>
                                    ) -> Result<(), Box<dyn std::error::Error>>,
                                >(*function)
                            }
                        };
                        index += core::mem::size_of::<usize>();
                        // SAFETY: An uninitialized `[use core::mem::MaybeUninit<_>; LEN]` is valid.
                        #[cfg(not(miri))]
                        let mut function_arguments = 𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬_𝐮𝐧𝐢𝐨𝐧::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> {
                            𝗎𝗇𝗂𝗇𝗂𝗍_𝖻𝗎𝖿𝖿𝖾𝗋: unsafe {
                                core::mem::MaybeUninit::<
                                    [core::mem::MaybeUninit<u8>; 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰 * core::mem::size_of::<i64>() + 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞]
                                >::uninit().assume_init()
                            }
                        };
                        #[cfg(miri)]
                        let mut function_arguments = 𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬_𝐮𝐧𝐢𝐨𝐧::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> {
                            𝗋𝖺𝗐_𝖻𝗎𝖿𝖿𝖾𝗋: [0; 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰 * core::mem::size_of::<i64>() + 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞]
                        };
                        let lbls: usize = emit_label_fn_info as u8 as usize;
                        let extra: usize = (emit_label_fn_info >> 8) as u8 as usize;
                        for lbl in 0..lbls {
                            // SAFETY: we are writing values here before it may be overwritten.
                            // SAFETY: we are using array dedicated for extras as stack.
                            unsafe {
                                let new_value =
                                    self.label_value(&mut index, &mut function_arguments.𝗅𝖺𝖻𝖾𝗅𝗌_𝖺𝗇𝖽_𝗌𝗍𝖺𝖼𝗄.𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄)?;
                                function_arguments.𝗅𝖺𝖻𝖾𝗅𝗌_𝖺𝗇𝖽_𝗌𝗍𝖺𝖼𝗄.𝗅𝖺𝖻𝖾𝗅_𝗏𝖺𝗅𝗎𝖾𝗌[lbl] = new_value;
                            }
                        }
                        // SAFETY: copy to buffer withing allocated size.
                        if extra > 0 {
                            unsafe {
                                core::ptr::copy_nonoverlapping(&self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[index],
                                core::ptr::addr_of_mut!(function_arguments.𝗋𝖺𝗐_𝖻𝗎𝖿𝖿𝖾𝗋[lbls*core::mem::size_of::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>()]),
                                extra);
                            }
                        }
                        index = (index + extra + (core::mem::align_of::<usize>()-1)) & !(core::mem::align_of::<usize>()-1);
                        function(𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔠𝔬𝔲𝔫𝔱𝔢𝔯(&mut code_size),
                                 // SAFETY: construction is symmetric to what inject_label_function does.
                                 &unsafe {function_arguments.𝖿𝗎𝗇𝖼𝗍𝗂𝗈𝗇_𝖽𝖺𝗍𝖺}
                        ).map_err(|err| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔯𝔯𝔬𝔯(err))?;
                    }
                }
            }

            if !layout_changed {
                return Ok(code_size.accumulated_size());
            }
        }
        Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩𝔰_𝔩𝔞𝔶𝔬𝔲𝔱_𝔲𝔫𝔰𝔱𝔞𝔟𝔩𝔢)
    }
    #[inline(always)]
    pub fn emit_code(&mut self, destination: &mut impl 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓) -> Result<(), <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut index: usize = 0;
        while index < self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.len() {
            let selector: [u8; core::mem::size_of::<usize>()] =
                self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[index..index+core::mem::size_of::<usize>()].try_into().unwrap();
            let selector = usize::from_ne_bytes(selector);
            index += core::mem::size_of::<usize>();
            match selector {
                0 => {
                    index += core::mem::size_of::<usize>();
                }
                chunk_size if chunk_size as isize > 0 => {
                    destination.emit_u8_slice(&self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[index..index+chunk_size])
                            .map_err(|err| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔯𝔯𝔬𝔯(err))?;
                    index = (index + chunk_size + (core::mem::align_of::<usize>()-1)) & !(core::mem::align_of::<usize>()-1);
                }
                emit_label_fn_info => {
                    let function: [u8; core::mem::size_of::<usize>()] =
                        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[index..index+core::mem::size_of::<usize>()].try_into().unwrap();
                    let function = usize::from_ne_bytes(function);
                    // SAFETY: we are retriveing 'static function pointer which was stored as properly aligned array of bytes.
                    #[cfg(not(miri))]
                    let function = unsafe {
                        core::mem::transmute::<
                            usize,
                            fn(
                                code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, function_arguments: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰, 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞>
                            ) -> Result<(), Box<dyn std::error::Error>>,
                        >(function)
                    };
                    #[cfg(miri)]
                    let function = {
                        let functions = 𝔪𝔦𝔯𝔦_𝔣𝔲𝔫𝔠𝔱𝔦𝔬𝔫𝔰.lock().unwrap();
                        let Some(ref functions) = *functions else { panic!("Internal error: map is none after creation"); };
                        let function = functions.get(&function).unwrap();
                        unsafe {
                            core::mem::transmute::<
                                fn(
                                    //code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, function_arguments: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<i64, 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰, 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞>
                                    code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, function_arguments: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<i64, 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰, 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞>
                                ) -> Result<(), Box<dyn std::error::Error>>,
                                fn(
                                    code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, function_arguments: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰, 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞>
                                ) -> Result<(), Box<dyn std::error::Error>>,
                            >(*function)
                        }
                    };
                    index += core::mem::size_of::<usize>();
                    // SAFETY: An uninitialized `[use core::mem::MaybeUninit<_>; LEN]` is valid.
                    #[cfg(not(miri))]
                    let mut function_arguments = 𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬_𝐮𝐧𝐢𝐨𝐧::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> {
                        𝗎𝗇𝗂𝗇𝗂𝗍_𝖻𝗎𝖿𝖿𝖾𝗋: unsafe {
                            core::mem::MaybeUninit::<
                                [core::mem::MaybeUninit<u8>; 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰 * core::mem::size_of::<i64>() + 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞]
                            >::uninit().assume_init()
                        }
                    };
                    #[cfg(miri)]
                    let mut function_arguments = 𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬_𝐮𝐧𝐢𝐨𝐧::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> {
                        𝗋𝖺𝗐_𝖻𝗎𝖿𝖿𝖾𝗋: [0; 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰 * core::mem::size_of::<i64>() + 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞]
                    };
                    let lbls: usize = emit_label_fn_info as u8 as usize;
                    let extra: usize = (emit_label_fn_info >> 8) as u8 as usize;
                    for lbl in 0..lbls {
                        // SAFETY: we are writing values here before it may be overwritten.
                        // SAFETY: we are using array dedicated for extras as stack.
                        unsafe {
                            let new_value =
                                self.label_value(&mut index, &mut function_arguments.𝗅𝖺𝖻𝖾𝗅𝗌_𝖺𝗇𝖽_𝗌𝗍𝖺𝖼𝗄.𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄)?;
                            function_arguments.𝗅𝖺𝖻𝖾𝗅𝗌_𝖺𝗇𝖽_𝗌𝗍𝖺𝖼𝗄.𝗅𝖺𝖻𝖾𝗅_𝗏𝖺𝗅𝗎𝖾𝗌[lbl] = new_value;
                        }
                    }
                    // SAFETY: copy to buffer withing allocated size.
                    if extra > 0 {
                        unsafe {
                            core::ptr::copy_nonoverlapping(&self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[index],
                            core::ptr::addr_of_mut!(function_arguments.𝗋𝖺𝗐_𝖻𝗎𝖿𝖿𝖾𝗋[lbls*core::mem::size_of::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>()]),
                            extra);
                        }
                    }
                    index = (index + extra + (core::mem::align_of::<usize>()-1)) & !(core::mem::align_of::<usize>()-1);
                    function(𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔪𝔦𝔱𝔱𝔢𝔯(destination),
                             // SAFETY: construction is symmetric to what inject_label_function does.
                             &unsafe {function_arguments.𝖿𝗎𝗇𝖼𝗍𝗂𝗈𝗇_𝖽𝖺𝗍𝖺}
                    ).map_err(|err| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔯𝔯𝔬𝔯(err))?;
                }
            }
        }
        Ok(())
    }
    #[inline(always)]
    fn label_value(&mut self, index: &mut usize, 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄: &mut [𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮; 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞 / core::mem::size_of::<i64>()])
        -> Result<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, <Self as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
    {
        let mut stack_position = 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞 / core::mem::size_of::<i64>();
        loop {
            let op = self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[*index];
            if (op as i8) < 0 {
                    let lbl = (op - 128) as usize;
                    *index +=1;
                    if stack_position == 0 {
                        return Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔠𝔞𝔩𝔠𝔲𝔩𝔞𝔱𝔦𝔬𝔫𝔰_𝔰𝔱𝔞𝔠𝔨_𝔲𝔫𝔡𝔢𝔯𝔣𝔩𝔬𝔴);
                    }
                    stack_position -= 1;
                    𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = self.𝗅𝖺𝖻𝖾𝗅𝗌[lbl];
            // SAFETY: originally operation is placed with “𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝓼𝓸𝓶𝓮𝓽𝓱𝓲𝓷𝓰 as u8”.
            } else {
                match unsafe { core::mem::transmute::<u8, 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬>(op) } {
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔢𝔪𝔦𝔱_𝔯𝔢𝔰𝔲𝔩𝔱 => {
                        *index +=1;
                        return Ok(𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position])
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔩𝔞𝔟𝔢𝔩_𝔲8 => {
                        let lbl = self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[*index + 1] as usize;
                        *index +=2;
                        if stack_position == 0 {
                            return Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔠𝔞𝔩𝔠𝔲𝔩𝔞𝔱𝔦𝔬𝔫𝔰_𝔰𝔱𝔞𝔠𝔨_𝔲𝔫𝔡𝔢𝔯𝔣𝔩𝔬𝔴);
                        }
                        stack_position -= 1;
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = self.𝗅𝖺𝖻𝖾𝗅𝗌[lbl];
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔩𝔞𝔟𝔢𝔩_𝔲16 => {
                        let lbl: [u8; 2] = self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[*index+1..*index+3].try_into().unwrap();
                        let lbl = u16::from_ne_bytes(lbl) as usize;
                        *index +=3;
                        if stack_position == 0 {
                            return Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔠𝔞𝔩𝔠𝔲𝔩𝔞𝔱𝔦𝔬𝔫𝔰_𝔰𝔱𝔞𝔠𝔨_𝔲𝔫𝔡𝔢𝔯𝔣𝔩𝔬𝔴);
                        }
                        stack_position -= 1;
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = self.𝗅𝖺𝖻𝖾𝗅𝗌[lbl];
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔩𝔞𝔟𝔢𝔩_𝔲32 => {
                        let lbl: [u8; 4] = self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[*index+1..*index+5].try_into().unwrap();
                        let lbl = u32::from_ne_bytes(lbl) as usize;
                        *index +=5;
                        if stack_position == 0 {
                            return Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔠𝔞𝔩𝔠𝔲𝔩𝔞𝔱𝔦𝔬𝔫𝔰_𝔰𝔱𝔞𝔠𝔨_𝔲𝔫𝔡𝔢𝔯𝔣𝔩𝔬𝔴);
                        }
                        stack_position -= 1;
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = self.𝗅𝖺𝖻𝖾𝗅𝗌[lbl];
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔩𝔞𝔟𝔢𝔩_𝔲64 => {
                        let lbl: [u8; 8] = self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[*index+1..*index+9].try_into().unwrap();
                        let lbl = u64::from_ne_bytes(lbl) as usize;
                        *index +=9;
                        if stack_position == 0 {
                            return Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔠𝔞𝔩𝔠𝔲𝔩𝔞𝔱𝔦𝔬𝔫𝔰_𝔰𝔱𝔞𝔠𝔨_𝔲𝔫𝔡𝔢𝔯𝔣𝔩𝔬𝔴);
                        }
                        stack_position -= 1;
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = self.𝗅𝖺𝖻𝖾𝗅𝗌[lbl];
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦8 => {
                        let imm = self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[*index + 1] as i8 as i64;
                        *index +=2;
                        if stack_position == 0 {
                            return Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔠𝔞𝔩𝔠𝔲𝔩𝔞𝔱𝔦𝔬𝔫𝔰_𝔰𝔱𝔞𝔠𝔨_𝔲𝔫𝔡𝔢𝔯𝔣𝔩𝔬𝔴);
                        }
                        stack_position -= 1;
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] =
                            imm.try_into().map_err(|_| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔳𝔞𝔩𝔲𝔢_𝔬𝔲𝔱_𝔬𝔣_𝔯𝔞𝔫𝔤𝔢)?;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲8 => {
                        let imm = self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[*index + 1] as i64;
                        *index +=2;
                        if stack_position == 0 {
                            return Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔠𝔞𝔩𝔠𝔲𝔩𝔞𝔱𝔦𝔬𝔫𝔰_𝔰𝔱𝔞𝔠𝔨_𝔲𝔫𝔡𝔢𝔯𝔣𝔩𝔬𝔴);
                        }
                        stack_position -= 1;
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] =
                            imm.try_into().map_err(|_| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔳𝔞𝔩𝔲𝔢_𝔬𝔲𝔱_𝔬𝔣_𝔯𝔞𝔫𝔤𝔢)?;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦16 => {
                        let imm: [u8; 2] = self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[*index+1..*index+3].try_into().unwrap();
                        let imm = i16::from_ne_bytes(imm) as i64;
                        *index +=3;
                        if stack_position == 0 {
                            return Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔠𝔞𝔩𝔠𝔲𝔩𝔞𝔱𝔦𝔬𝔫𝔰_𝔰𝔱𝔞𝔠𝔨_𝔲𝔫𝔡𝔢𝔯𝔣𝔩𝔬𝔴);
                        }
                        stack_position -= 1;
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] =
                            imm.try_into().map_err(|_| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔳𝔞𝔩𝔲𝔢_𝔬𝔲𝔱_𝔬𝔣_𝔯𝔞𝔫𝔤𝔢)?;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲16 => {
                        let imm: [u8; 2] = self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[*index+1..*index+3].try_into().unwrap();
                        let imm = u16::from_ne_bytes(imm) as i64;
                        *index +=3;
                        if stack_position == 0 {
                            return Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔠𝔞𝔩𝔠𝔲𝔩𝔞𝔱𝔦𝔬𝔫𝔰_𝔰𝔱𝔞𝔠𝔨_𝔲𝔫𝔡𝔢𝔯𝔣𝔩𝔬𝔴);
                        }
                        stack_position -= 1;
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] =
                            imm.try_into().map_err(|_| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔳𝔞𝔩𝔲𝔢_𝔬𝔲𝔱_𝔬𝔣_𝔯𝔞𝔫𝔤𝔢)?;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦32 => {
                        let imm: [u8; 4] = self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[*index+1..*index+5].try_into().unwrap();
                        let imm = i32::from_ne_bytes(imm) as i64;
                        *index +=5;
                        if stack_position == 0 {
                            return Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔠𝔞𝔩𝔠𝔲𝔩𝔞𝔱𝔦𝔬𝔫𝔰_𝔰𝔱𝔞𝔠𝔨_𝔲𝔫𝔡𝔢𝔯𝔣𝔩𝔬𝔴);
                        }
                        stack_position -= 1;
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] =
                            imm.try_into().map_err(|_| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔳𝔞𝔩𝔲𝔢_𝔬𝔲𝔱_𝔬𝔣_𝔯𝔞𝔫𝔤𝔢)?;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲32 => {
                        let imm: [u8; 4] = self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[*index+1..*index+5].try_into().unwrap();
                        let imm = u32::from_ne_bytes(imm) as i64;
                        *index +=5;
                        if stack_position == 0 {
                            return Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔠𝔞𝔩𝔠𝔲𝔩𝔞𝔱𝔦𝔬𝔫𝔰_𝔰𝔱𝔞𝔠𝔨_𝔲𝔫𝔡𝔢𝔯𝔣𝔩𝔬𝔴);
                        }
                        stack_position -= 1;
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] =
                            imm.try_into().map_err(|_| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔳𝔞𝔩𝔲𝔢_𝔬𝔲𝔱_𝔬𝔣_𝔯𝔞𝔫𝔤𝔢)?;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦64 => {
                        let imm: [u8; 8] = self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[*index+1..*index+9].try_into().unwrap();
                        let imm = i64::from_ne_bytes(imm) as i64;
                        *index +=9;
                        if stack_position == 0 {
                            return Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔠𝔞𝔩𝔠𝔲𝔩𝔞𝔱𝔦𝔬𝔫𝔰_𝔰𝔱𝔞𝔠𝔨_𝔲𝔫𝔡𝔢𝔯𝔣𝔩𝔬𝔴);
                        }
                        stack_position -= 1;
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] =
                            imm.try_into().map_err(|_| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔳𝔞𝔩𝔲𝔢_𝔬𝔲𝔱_𝔬𝔣_𝔯𝔞𝔫𝔤𝔢)?;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲64 => {
                        let imm: [u8; 8] = self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺[*index+1..*index+9].try_into().unwrap();
                        let imm = u64::from_ne_bytes(imm) as i64;
                        *index +=9;
                        if stack_position == 0 {
                            return Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔠𝔞𝔩𝔠𝔲𝔩𝔞𝔱𝔦𝔬𝔫𝔰_𝔰𝔱𝔞𝔠𝔨_𝔲𝔫𝔡𝔢𝔯𝔣𝔩𝔬𝔴);
                        }
                        stack_position -= 1;
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] =
                            imm.try_into().map_err(|_| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔳𝔞𝔩𝔲𝔢_𝔬𝔲𝔱_𝔬𝔣_𝔯𝔞𝔫𝔤𝔢)?;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔞𝔡𝔡_𝔬𝔭 => {
                        let op1 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        stack_position += 1;
                        let op0 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = op0 + op1;
                        *index +=1;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔰𝔲𝔟_𝔬𝔭 => {
                        let op1 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        stack_position += 1;
                        let op0 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = op0 - op1;
                        *index +=1;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔪𝔲𝔩_𝔬𝔭 => {
                        let op1 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        stack_position += 1;
                        let op0 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = op0 * op1;
                        *index +=1;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔳_𝔬𝔭 => {
                        let op1 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        stack_position += 1;
                        let op0 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = op0 / op1;
                        *index +=1;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔯𝔢𝔪_𝔬𝔭 => {
                        let op1 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        stack_position += 1;
                        let op0 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = op0 % op1;
                        *index +=1;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔞𝔫𝔡_𝔬𝔭 => {
                        let op1 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        stack_position += 1;
                        let op0 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = op0 & op1;
                        *index +=1;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔬𝔯_𝔬𝔭 => {
                        let op1 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        stack_position += 1;
                        let op0 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = op0 | op1;
                        *index +=1;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔵𝔬𝔯_𝔬𝔭 => {
                        let op1 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        stack_position += 1;
                        let op0 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = op0 ^ op1;
                        *index +=1;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔰𝔞𝔯_𝔬𝔭 => {
                        let op1 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        stack_position += 1;
                        let op0 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = op0.sar(op1);
                        *index +=1;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔰𝔥𝔩_𝔬𝔭 => {
                        let op1 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        stack_position += 1;
                        let op0 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = op0 << op1;
                        *index +=1;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔰𝔥𝔯_𝔬𝔭 => {
                        let op1 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        stack_position += 1;
                        let op0 = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = 𝒏𝒂𝒕𝒊𝒗𝒆_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::shr(op0, op1);
                        *index +=1;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔫𝔢𝔤_𝔬𝔭 => {
                        let op = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = op.neg();
                        *index +=1;
                    }
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔫𝔬𝔱_𝔬𝔭 => {
                        let op = 𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position];
                        𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄[stack_position] = !op;
                        *index +=1;
                    }
                }
            }
        }
    }
}

#[repr(C)]
union 𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬_𝐮𝐧𝐢𝐨𝐧<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: Copy> {
    𝗎𝗇𝗂𝗇𝗂𝗍_𝖻𝗎𝖿𝖿𝖾𝗋: [core::mem::MaybeUninit<u8>; 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰 * core::mem::size_of::<i64>() + 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞],
    𝗋𝖺𝗐_𝖻𝗎𝖿𝖿𝖾𝗋: [u8; 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰 * core::mem::size_of::<i64>() + 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞],
    𝗅𝖺𝖻𝖾𝗅𝗌_𝖺𝗇𝖽_𝗌𝗍𝖺𝖼𝗄: 𝐥𝐚𝐛𝐞𝐥𝐬_𝐚𝐧𝐝_𝐬𝐭𝐚𝐜𝐤_𝐢𝐧𝐟𝐨<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>,
    𝖿𝗎𝗇𝖼𝗍𝗂𝗈𝗇_𝖽𝖺𝗍𝖺: 𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰, 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞>,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct 𝐥𝐚𝐛𝐞𝐥𝐬_𝐚𝐧𝐝_𝐬𝐭𝐚𝐜𝐤_𝐢𝐧𝐟𝐨<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> {
    𝗅𝖺𝖻𝖾𝗅_𝗏𝖺𝗅𝗎𝖾𝗌:  [𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮; 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰],
    𝖾𝗑𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇𝗌_𝗌𝗍𝖺𝖼𝗄: [𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮; 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞 / core::mem::size_of::<i64>()],
}


#[derive(Debug)]
pub struct 𝐧𝐚𝐭𝐢𝐯𝐞_𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> {
    𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄: &'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ mut 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>,
    𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄: &'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾 mut 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤,
}

impl<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: 𝒏𝒂𝒕𝒊𝒗𝒆_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔> 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓
for 𝐧𝐚𝐭𝐢𝐯𝐞_𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>
where
    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>:
        Clone +
        core::ops::Add<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Neg<Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Not<Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧>,
    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧:
        Clone +
        core::ops::Add<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Neg<Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Not<Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧>,
    𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮:
        Copy +
        TryInto<i64> +
        TryInto<usize> +
        core::cmp::Eq +
        core::ops::Add<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Add<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitAnd<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitOr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::BitXor<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Div<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Mul<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Neg<Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Not<Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Rem<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Rem<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shl<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Shl<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shl<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shr<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Shr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Shr<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, Output = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> +
        core::ops::Sub<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧, Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧>,
    i64: TryInto<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>,
    usize: TryInto<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>,
    <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<i64>>::Error: core::fmt::Debug,
    <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<usize>>::Error: core::fmt::Debug,
    <i64 as TryInto<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>>::Error: core::fmt::Debug,
    <usize as TryInto<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>>::Error: core::fmt::Debug,
{
    type 𝐜𝐨𝐝𝐞 = 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>;

    #[inline(always)]
    fn new_label(&mut self) -> <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥 {
        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.new_label()
    }
    #[inline(always)]
    fn set_label(&mut self, lbl: <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥)
        -> Result<(), <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
    {
        let label_id: usize = lbl.0.try_into().map_err(|_| 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔱𝔬𝔬_𝔟𝔦𝔤)?;
        if self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝖻𝖾𝗅𝗌.len() <= label_id {
            Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔦𝔡_𝔬𝔲𝔱_𝔬𝔣_𝔯𝔞𝔫𝔤𝔢)
        } else if self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝖻𝖾𝗅𝗌[label_id] != 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮::𝔷𝔢𝔯𝔬 {
            Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔞𝔩𝔯𝔢𝔞𝔡𝔶_𝔞𝔱𝔱𝔞𝔠𝔥𝔢𝔡)
        } else {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.close_bytes_fragment();
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&label_id.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝖻𝖾𝗅𝗌[label_id] = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮::𝔫𝔬_𝔞𝔡𝔡𝔯𝔢𝔰𝔰;
            Ok(())
        }
    }
    #[inline(always)]
    fn emit_byte(&mut self, expr: <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧)
        -> Result<(), <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
    {
        self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_byte::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_2byte(&mut self, expr: <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧)
        -> Result<(), <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
    {
        self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_2byte::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_2byte_be(&mut self, expr: <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧)
        -> Result<(), <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
    {
        self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_2byte_be::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_2byte_le(&mut self, expr: <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧)
        -> Result<(), <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
    {
        self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_2byte_le::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_4byte(&mut self, expr: <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧)
        -> Result<(), <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
    {
        self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_4byte::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_4byte_be(&mut self, expr: <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧)
        -> Result<(), <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
    {
        self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_4byte_be::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_4byte_le(&mut self, expr: <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧)
        -> Result<(), <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
    {
        self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_4byte_le::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_8byte(&mut self, expr: <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧)
        -> Result<(), <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
    {
        self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_8byte::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_8byte_be(&mut self, expr: <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧)
        -> Result<(), <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
    {
        self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_8byte_be::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
    #[inline(always)]
    fn emit_8byte_le(&mut self, expr: <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧)
        -> Result<(), <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
    {
        self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.inject_label_function(emit_8byte_le::<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>, [expr], []);
        Ok(())
    }
}

impl<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓
for 𝐧𝐚𝐭𝐢𝐯𝐞_𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> {
    #[inline(always)]
    fn emit_u8(&mut self, value: u8) -> Result<(), Box<dyn std::error::Error>> {
        if self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.push(value);
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 1;
        } else {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.push(value);
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 1;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_u16(&mut self, value: u16) -> Result<(), Box<dyn std::error::Error>> {
        if self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 2;
        } else {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 2;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_u32(&mut self, value: u32) -> Result<(), Box<dyn std::error::Error>> {
        if self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 4;
        } else {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 4;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_u64(&mut self, value: u64) -> Result<(), Box<dyn std::error::Error>> {
        if self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 8;
        } else {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 8;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_u128(&mut self, value: u128) -> Result<(), Box<dyn std::error::Error>> {
        if self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 16;
        } else {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 16;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_u8_slice(&mut self, 𝗌𝗅𝗂𝖼𝖾: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if 𝗌𝗅𝗂𝖼𝖾.len() != 0 {
            if self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
                self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
                self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(𝗌𝗅𝗂𝖼𝖾);
                self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 𝗌𝗅𝗂𝖼𝖾.len();
            } else {
                self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(𝗌𝗅𝗂𝖼𝖾);
                self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 𝗌𝗅𝗂𝖼𝖾.len();
            }
        }
        Ok(())
    }
}

impl<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓
for 𝐧𝐚𝐭𝐢𝐯𝐞_𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮>
where Self: 𝒆𝒎𝒊𝒕𝒕𝒆𝒓<𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = (), 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = core::convert::Infallible>
{
    fn emit_u8(&mut self, value: u8) -> Result<(), core::convert::Infallible> {
        if self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.push(value);
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 1;
        } else {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.push(value);
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 1;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_bytes_u16(&mut self, value: u16) -> Result<(), core::convert::Infallible> {
        if self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 2;
        } else {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 2;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_bytes_u32(&mut self, value: u32) -> Result<(), core::convert::Infallible> {
        if self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 4;
        } else {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 4;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_bytes_u64(&mut self, value: u64) -> Result<(), core::convert::Infallible> {
        if self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 8;
        } else {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 8;
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_bytes_u128(&mut self, value: u128) -> Result<(), core::convert::Infallible> {
        if self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 16;
        } else {
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&value.to_ne_bytes());
            self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 16;
        }
        Ok(())
    }
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    fn emit_u8_array<const 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮: usize>(&mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮]) -> Result<(), core::convert::Infallible>
    where Self: 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, { 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮 }> {
        if 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮 != 0 {
            if self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
                self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
                self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮;
            } else {
                self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮;
            }
        }
        Ok(())
    }
    #[inline(always)]
    fn emit_u8_slice(&mut self, 𝗌𝗅𝗂𝖼𝖾: &[u8]) -> Result<(), core::convert::Infallible> {
        if 𝗌𝗅𝗂𝖼𝖾.len() != 0 {
            if self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 == 0 {
                self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(&0usize.to_ne_bytes());
                self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(𝗌𝗅𝗂𝖼𝖾);
                self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 𝗌𝗅𝗂𝖼𝖾.len();
            } else {
                self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝖽𝖺𝗍𝖺.extend_from_slice(𝗌𝗅𝗂𝖼𝖾);
                self.𝖼𝗎𝗋𝗋𝖾𝗇𝗍_𝖻𝗅𝗈𝖼𝗄.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 += 𝗌𝗅𝗂𝖼𝖾.len();
            }
        }
        Ok(())
    }
}

impl<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> 𝒆𝒎𝒊𝒕𝒕𝒆𝒓
for 𝐧𝐚𝐭𝐢𝐯𝐞_𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮> {
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = core::convert::Infallible;
    fn combine_results(_: &mut (), _: ()) {
    }
}

#[derive(Clone, Debug, Default)]
pub struct 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤 {
    // Vector contains collected assembler code. Code is collected in chunks.
    //
    // First size of chunk in ne_chunk format is specified then sequence of instructions follow.
    // Last chunk is usually not “closed”, it's length is kept in a 𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾.
    //
    // If instruction which uses label operands is stored with top bit is set, low byte includes number of labels to be calculated
    // and provided for the emitter function, next byte includes size of data needed besides labels, after that pointer of function
    // to call follows.
    //
    // Size of chunk equal to zero (if chunk is closed) means that next usize bytes include label number to set.
    𝖽𝖺𝗍𝖺: Vec<u8>,
    // Note: initial value is zero which means that there no uncloses code chunks yet.
    𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾: usize,
}

impl 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤 {
    #[inline(always)]
    fn close_bytes_fragment(&mut self) {
        if self.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 != 0 {
            let len = self.𝖽𝖺𝗍𝖺.len();
            let size_position = len - self.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 - core::mem::size_of::<usize>();
            if size_position > len {
                // SAFETY: 𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 and grows with vector, overflow is impossible because Vec::push would panic.
                unsafe { core::hint::unreachable_unchecked() };
            }
            self.𝖽𝖺𝗍𝖺[size_position..size_position+core::mem::size_of::<usize>()]
                .copy_from_slice(&self.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾.to_ne_bytes());
            let misalignment = len & (core::mem::align_of::<usize>()-1);
            if misalignment != 0 {
                self.𝖽𝖺𝗍𝖺.extend_from_slice(&𝔷𝔢𝔯𝔬𝔰[..core::mem::align_of::<usize>()-misalignment]);
            }
            self.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 0;
        }
    }
    #[inline(always)]
    #[allow(non_upper_case_globals)]
    pub fn inject_label_function<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, const 𝔩𝔞𝔟𝔢𝔩𝔰_𝔰𝔦𝔷𝔢: usize, const 𝔢𝔵𝔱𝔯𝔞_𝔰𝔦𝔷𝔢: usize>(
        &mut self, function: fn(
            code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, function_arguments: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 𝔩𝔞𝔟𝔢𝔩𝔰_𝔰𝔦𝔷𝔢, 𝔢𝔵𝔱𝔯𝔞_𝔰𝔦𝔷𝔢>
        ) -> Result<(), Box<dyn std::error::Error>>,
        label_expressions: [𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧; 𝔩𝔞𝔟𝔢𝔩𝔰_𝔰𝔦𝔷𝔢],
        𝖾𝗑𝗍𝗋𝖺: [u8; 𝔢𝔵𝔱𝔯𝔞_𝔰𝔦𝔷𝔢],
    ) {
        self.close_bytes_fragment();
        if 𝔩𝔞𝔟𝔢𝔩𝔰_𝔰𝔦𝔷𝔢 > 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰 as usize {
            panic!("𝔩𝔞𝔟𝔢𝔩𝔰_𝔰𝔦𝔷𝔢 ({𝔩𝔞𝔟𝔢𝔩𝔰_𝔰𝔦𝔷𝔢}) must be less or equal to u8::MAX");
        }
        if 𝔢𝔵𝔱𝔯𝔞_𝔰𝔦𝔷𝔢 > 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞 as usize {
            panic!("𝔢𝔵𝔱𝔯𝔞_𝔰𝔦𝔷𝔢 ({𝔢𝔵𝔱𝔯𝔞_𝔰𝔦𝔷𝔢}) must be less or equal to u8::MAX");
        }
        self.𝖽𝖺𝗍𝖺.extend_from_slice(&(isize::MIN as usize | 𝔩𝔞𝔟𝔢𝔩𝔰_𝔰𝔦𝔷𝔢 | 𝔢𝔵𝔱𝔯𝔞_𝔰𝔦𝔷𝔢 << 8).to_ne_bytes());
        // SAFETY: we are storing 'static function pointer as properly aligned array of bytes.
        let function_address = unsafe {
            core::mem::transmute::<
                fn(
                    code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, function_arguments: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 𝔩𝔞𝔟𝔢𝔩𝔰_𝔰𝔦𝔷𝔢, 𝔢𝔵𝔱𝔯𝔞_𝔰𝔦𝔷𝔢>
                ) -> Result<(), Box<dyn std::error::Error>>,
                usize,
            >(function)
        };
        #[cfg(miri)]
        {
            let mut functions = 𝔪𝔦𝔯𝔦_𝔣𝔲𝔫𝔠𝔱𝔦𝔬𝔫𝔰.lock().unwrap();
            if functions.is_none() {
               functions.replace(HashMap::new());
            }
            let Some(ref mut functions) = *functions else { panic!("Internal error: map is none after creation"); };
            functions.insert(function_address, unsafe {
                core::mem::transmute::<
                    fn(
                        code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, function_arguments: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 𝔩𝔞𝔟𝔢𝔩𝔰_𝔰𝔦𝔷𝔢, 𝔢𝔵𝔱𝔯𝔞_𝔰𝔦𝔷𝔢>
                    ) -> Result<(), Box<dyn std::error::Error>>,
                    fn(
                        code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, function_arguments: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<i64, 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰, 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞>
                    ) -> Result<(), Box<dyn std::error::Error>>,
                >(function)
            });
        }

        self.𝖽𝖺𝗍𝖺.extend_from_slice(&function_address.to_ne_bytes());
        for label_expression in label_expressions {
            let buffer: &[u8] = (&label_expression).into();
            self.𝖽𝖺𝗍𝖺.extend_from_slice(buffer);
            self.𝖽𝖺𝗍𝖺.push(𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔢𝔪𝔦𝔱_𝔯𝔢𝔰𝔲𝔩𝔱 as u8);
        }
        self.𝖽𝖺𝗍𝖺.extend_from_slice(&𝖾𝗑𝗍𝗋𝖺[..]);
        let misalignment = self.𝖽𝖺𝗍𝖺.len() & (core::mem::align_of::<usize>()-1);
        if misalignment != 0 {
            self.𝖽𝖺𝗍𝖺.extend_from_slice(&𝔷𝔢𝔯𝔬𝔰[..core::mem::align_of::<usize>()-misalignment]);
        }
    }
}

fn emit_byte<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: Copy + TryInto<i64>>(
    code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬{𝗅𝖺𝖻𝖾𝗅_𝗏𝖺𝗅𝗎𝖾𝗌:[expr], 𝖾𝗑𝗍𝗋𝖺:[]}: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 1, 0>
)
    -> Result<(), Box<dyn std::error::Error>> where <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<i64>>::Error: core::fmt::Debug
{
    match code_emitter {
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔠𝔬𝔲𝔫𝔱𝔢𝔯(counter) => counter.emit_u8(0),
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔪𝔦𝔱𝔱𝔢𝔯(emit_byte) => emit_byte.emit_u8(expr.try_into().unwrap() as u8),
    }
}

fn emit_2byte<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: Copy + TryInto<i64>>(
    code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬{𝗅𝖺𝖻𝖾𝗅_𝗏𝖺𝗅𝗎𝖾𝗌:[expr], 𝖾𝗑𝗍𝗋𝖺:[]}: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 1, 0>
)
    -> Result<(), Box<dyn std::error::Error>> where <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<i64>>::Error: core::fmt::Debug
{
    match code_emitter {
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔠𝔬𝔲𝔫𝔱𝔢𝔯(counter) => counter.emit_u16(0),
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔪𝔦𝔱𝔱𝔢𝔯(emit_byte) => emit_byte.emit_u16(expr.try_into().unwrap() as u16),
    }
}

fn emit_2byte_be<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: Copy + TryInto<i64>>(
    code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬{𝗅𝖺𝖻𝖾𝗅_𝗏𝖺𝗅𝗎𝖾𝗌:[expr], 𝖾𝗑𝗍𝗋𝖺:[]}: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 1, 0>
)
    -> Result<(), Box<dyn std::error::Error>> where <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<i64>>::Error: core::fmt::Debug
{
    match code_emitter {
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔠𝔬𝔲𝔫𝔱𝔢𝔯(counter) => counter.emit_u16(0),
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔪𝔦𝔱𝔱𝔢𝔯(emit_byte) => emit_byte.emit_u16((expr.try_into().unwrap() as u16).to_be()),
    }
}

fn emit_2byte_le<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: Copy + TryInto<i64>>(
    code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬{𝗅𝖺𝖻𝖾𝗅_𝗏𝖺𝗅𝗎𝖾𝗌:[expr], 𝖾𝗑𝗍𝗋𝖺:[]}: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 1, 0>
)
    -> Result<(), Box<dyn std::error::Error>> where <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<i64>>::Error: core::fmt::Debug
{
    match code_emitter {
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔠𝔬𝔲𝔫𝔱𝔢𝔯(counter) => counter.emit_u16(0),
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔪𝔦𝔱𝔱𝔢𝔯(emit_byte) => emit_byte.emit_u16((expr.try_into().unwrap() as u16).to_le()),
    }
}

fn emit_4byte<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: Copy + TryInto<i64>>(
    code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬{𝗅𝖺𝖻𝖾𝗅_𝗏𝖺𝗅𝗎𝖾𝗌:[expr], 𝖾𝗑𝗍𝗋𝖺:[]}: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 1, 0>
)
    -> Result<(), Box<dyn std::error::Error>> where <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<i64>>::Error: core::fmt::Debug
{
    match code_emitter {
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔠𝔬𝔲𝔫𝔱𝔢𝔯(counter) => counter.emit_u32(0),
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔪𝔦𝔱𝔱𝔢𝔯(emit_byte) => emit_byte.emit_u32(expr.try_into().unwrap() as u32),
    }
}

fn emit_4byte_be<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: Copy + TryInto<i64>>(
    code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬{𝗅𝖺𝖻𝖾𝗅_𝗏𝖺𝗅𝗎𝖾𝗌:[expr], 𝖾𝗑𝗍𝗋𝖺:[]}: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 1, 0>
)
    -> Result<(), Box<dyn std::error::Error>> where <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<i64>>::Error: core::fmt::Debug
{
    match code_emitter {
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔠𝔬𝔲𝔫𝔱𝔢𝔯(counter) => counter.emit_u32(0),
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔪𝔦𝔱𝔱𝔢𝔯(emit_byte) => emit_byte.emit_u32((expr.try_into().unwrap() as u32).to_be()),
    }
}

fn emit_4byte_le<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: Copy + TryInto<i64>>(
    code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬{𝗅𝖺𝖻𝖾𝗅_𝗏𝖺𝗅𝗎𝖾𝗌:[expr], 𝖾𝗑𝗍𝗋𝖺:[]}: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 1, 0>
)
    -> Result<(), Box<dyn std::error::Error>> where <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<i64>>::Error: core::fmt::Debug
{
    match code_emitter {
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔠𝔬𝔲𝔫𝔱𝔢𝔯(counter) => counter.emit_u32(0),
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔪𝔦𝔱𝔱𝔢𝔯(emit_byte) => emit_byte.emit_u32((expr.try_into().unwrap() as u32).to_le()),
    }
}

fn emit_8byte<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: Copy + TryInto<i64>>(
    code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬{𝗅𝖺𝖻𝖾𝗅_𝗏𝖺𝗅𝗎𝖾𝗌:[expr], 𝖾𝗑𝗍𝗋𝖺:[]}: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 1, 0>
)
    -> Result<(), Box<dyn std::error::Error>> where <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<i64>>::Error: core::fmt::Debug
{
    match code_emitter {
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔠𝔬𝔲𝔫𝔱𝔢𝔯(counter) => counter.emit_u64(0),
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔪𝔦𝔱𝔱𝔢𝔯(emit_byte) => emit_byte.emit_u64(expr.try_into().unwrap() as u64),
    }
}

fn emit_8byte_be<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: Copy + TryInto<i64>>(
    code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬{𝗅𝖺𝖻𝖾𝗅_𝗏𝖺𝗅𝗎𝖾𝗌:[expr], 𝖾𝗑𝗍𝗋𝖺:[]}: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 1, 0>
)
    -> Result<(), Box<dyn std::error::Error>> where <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<i64>>::Error: core::fmt::Debug
{
    match code_emitter {
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔠𝔬𝔲𝔫𝔱𝔢𝔯(counter) => counter.emit_u64(0),
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔪𝔦𝔱𝔱𝔢𝔯(emit_byte) => emit_byte.emit_u64((expr.try_into().unwrap() as u64).to_be()),
    }
}

fn emit_8byte_le<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮: Copy + TryInto<i64>>(
    code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬{𝗅𝖺𝖻𝖾𝗅_𝗏𝖺𝗅𝗎𝖾𝗌:[expr], 𝖾𝗑𝗍𝗋𝖺:[]}: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 1, 0>
)
    -> Result<(), Box<dyn std::error::Error>> where <𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮 as TryInto<i64>>::Error: core::fmt::Debug
{
    match code_emitter {
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔠𝔬𝔲𝔫𝔱𝔢𝔯(counter) => counter.emit_u64(0),
        𝐞𝐦𝐢𝐭𝐭𝐞𝐫::𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔪𝔦𝔱𝔱𝔢𝔯(emit_byte) => emit_byte.emit_u64((expr.try_into().unwrap() as u64).to_le()),
    }
}

pub enum 𝐞𝐦𝐢𝐭𝐭𝐞𝐫<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ> {
    𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔠𝔬𝔲𝔫𝔱𝔢𝔯(&'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ mut 𝐝𝐲𝐧_𝐜𝐨𝐝𝐞_𝐬𝐢𝐳𝐞_𝐜𝐨𝐮𝐧𝐭𝐞𝐫),
    𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔪𝔦𝔱𝔱𝔢𝔯(&'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ mut dyn super::super::𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓),
}

#[derive(Clone, Copy, Debug)]
#[allow(non_upper_case_globals)]
#[repr(C)]
pub struct 𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, const 𝔩𝔞𝔟𝔢𝔩𝔰_𝔰𝔦𝔷𝔢: usize, const 𝔢𝔵𝔱𝔯𝔞_𝔰𝔦𝔷𝔢: usize> {
    pub 𝗅𝖺𝖻𝖾𝗅_𝗏𝖺𝗅𝗎𝖾𝗌: [𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮; 𝔩𝔞𝔟𝔢𝔩𝔰_𝔰𝔦𝔷𝔢],
    pub 𝖾𝗑𝗍𝗋𝖺: [u8; 𝔢𝔵𝔱𝔯𝔞_𝔰𝔦𝔷𝔢]
}

#[derive(Debug)]
#[non_exhaustive]
pub enum 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬 {
    𝔠𝔬𝔡𝔢_𝔰𝔦𝔷𝔢_𝔱𝔬𝔬_𝔟𝔦𝔤,
    𝔩𝔞𝔟𝔢𝔩_𝔞𝔩𝔯𝔢𝔞𝔡𝔶_𝔞𝔱𝔱𝔞𝔠𝔥𝔢𝔡,
    𝔩𝔞𝔟𝔢𝔩_𝔠𝔞𝔩𝔠𝔲𝔩𝔞𝔱𝔦𝔬𝔫𝔰_𝔰𝔱𝔞𝔠𝔨_𝔲𝔫𝔡𝔢𝔯𝔣𝔩𝔬𝔴,
    𝔩𝔞𝔟𝔢𝔩_𝔦𝔡_𝔬𝔲𝔱_𝔬𝔣_𝔯𝔞𝔫𝔤𝔢,
    𝔩𝔞𝔟𝔢𝔩𝔰_𝔩𝔞𝔶𝔬𝔲𝔱_𝔲𝔫𝔰𝔱𝔞𝔟𝔩𝔢,
    𝔳𝔞𝔩𝔲𝔢_𝔬𝔲𝔱_𝔬𝔣_𝔯𝔞𝔫𝔤𝔢,
    𝔡𝔶𝔫𝔞𝔪𝔦𝔠_𝔢𝔯𝔯𝔬𝔯(Box<dyn std::error::Error>),
}

#[allow(non_upper_case_globals)]
const 𝔷𝔢𝔯𝔬𝔰: [u8; core::mem::size_of::<usize>()] = [0; core::mem::size_of::<usize>()];

// Maximum number of labels supported in 𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬. Can be at most 255 because we use byte field to store it.
#[allow(non_upper_case_globals)]
const 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰: usize = 4;

// Maximum number of labels supported in 𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬. Can be at most 255 because we use byte field to store it.
// Note that we are also using that array as stack for calculations and need at least 4 slots thus we are using
// 4 * core::mem::size_of::<i64> (128bit CPUs are not supported).
#[allow(non_upper_case_globals)]
const 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞: usize = 4 * core::mem::size_of::<i64>();

// We are converting pointers to functions to usize integer and then convert these integers back to function pointers.
// This is not supported by miri because of strict_provenance issues, but there are no appropriate functions for
// function pointers yet.
//
// Thus under miri we would just stash addresses of functions and use them when needed.
// It's not what code in non-miri case is doing but allows us to test the rest of the code.
#[cfg(miri)]
#[allow(non_upper_case_globals)]
static 𝔪𝔦𝔯𝔦_𝔣𝔲𝔫𝔠𝔱𝔦𝔬𝔫𝔰: Mutex<Option<HashMap<
    usize, fn(code_emitter: 𝐞𝐦𝐢𝐭𝐭𝐞𝐫, function_arguments: &𝐚𝐫𝐠𝐮𝐦𝐞𝐧𝐭𝐬<i64, 𝔪𝔞𝔵_𝔩𝔞𝔟𝔢𝔩𝔰, 𝔪𝔞𝔵_𝔢𝔵𝔱𝔯𝔞>)
        -> Result<(), Box<dyn std::error::Error>>,
>>> = Mutex::new(None);

#[cfg(miri)]
use std::collections::HashMap;
#[cfg(miri)]
use std::sync::Mutex;

use super::super::𝒆𝒎𝒊𝒕𝒕𝒆𝒓;
use super::super::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;
use super::super::𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;
use super::super::𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;

use super::super::𝐝𝐲𝐧_𝐜𝐨𝐝𝐞_𝐬𝐢𝐳𝐞_𝐜𝐨𝐮𝐧𝐭𝐞𝐫;

use super::𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆;
use super::𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;

use super::𝗻𝘂𝗺𝗯𝗲𝗿𝗲𝗱_𝗹𝗮𝗯𝗲𝗹𝘀::𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥;
use super::𝗻𝘂𝗺𝗯𝗲𝗿𝗲𝗱_𝗹𝗮𝗯𝗲𝗹𝘀::𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧;
use super::𝗻𝘂𝗺𝗯𝗲𝗿𝗲𝗱_𝗹𝗮𝗯𝗲𝗹𝘀::𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬;

#[path = "native-code-tests.rs"]
#[cfg(test)]
mod 𝘁𝗲𝘀𝘁𝘀;
