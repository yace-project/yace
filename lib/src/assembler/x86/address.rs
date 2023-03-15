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

// Marker trait to prevenct conflict for Option<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞>: should it be Some(𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞{}) or None?
pub trait 𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆 {}

// Address includes some values which are optional and can be unfilled. We use empty type to mark these.
// Note: we can not use just an empty tuple because then we couldn't define From trait for it.
// Note2: scale and displacement must be obtainable from 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞.  Integer types are obtainable automatically.
#[derive(Clone, Copy, Default, Debug)]
pub struct 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {}

// All assembler support two addresses: 16ᵇⁱᵗ and 32ᵇⁱᵗ in legacy mode or 32ᵇⁱᵗ and 64ᵇⁱᵗ in ₓ86_64 mode.
// We provide different address constants for these three modes — that way there are no ambiguity even if simple 𝔞𝔡𝔡𝔯𝔢𝔰𝔰 [0] is used.
#[derive(Clone, Copy, Default, Debug)]
pub struct 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ {} // We don't really need that because of 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086 vs 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86 difference. Maybe remove?
#[derive(Clone, Copy, Default, Debug)]
pub struct 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ {}
#[derive(Clone, Copy, Default, Debug)]
pub struct 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ {}

// In ₓ86_64 mode 𝔯𝔦𝔭 base register implies no index register in address.
// We accept it as Option<𝐧𝐨_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum 𝐧𝐨_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 {}

impl From<𝐧𝐨_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> for u8 {
    #[inline(always)]
    fn from(_: 𝐧𝐨_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫) -> u8 {
      // SAFETY: It's impossible to create 𝐧𝐨_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 value and thus not possible to call that function.
      unsafe { core::hint::unreachable_unchecked() }
    }
}

// Address type is just a combination of arguments and it's mostly used to simplify interface. There are few address types:
//   1. Absolute address — segment plus 16ᵇⁱᵗ/32ᵇⁱᵗ/64ᵇⁱᵗ offset. Only used in 𝔪𝔬𝔳 instruction and is quite similar to “normal”
//      address in legacy modes. In 64ᵇⁱᵗ x86-64 mode it's different, though, since it's the only one with support for 64ᵇⁱᵗ offset.
//      We can not just use similar trick to how we handle 𝔠𝔪𝔭𝔰/𝔦𝔫𝔰/𝔩𝔬𝔡𝔰/𝔪𝔬𝔳𝔰/𝔬𝔲𝔱𝔰/𝔰𝔠𝔞𝔰/𝔰𝔱𝔬𝔰/𝔵𝔩𝔞𝔱 because 𝔪𝔬𝔳 supports both addresses:
//          mov(𝔢𝔞𝔵, 𝔞𝔟𝔰𝔬𝔩𝔲𝔱𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰.with_offset(0)) would emit 0xa1, 0x00, 0x00, 0x00, 0x00
//          mov(𝔢𝔞𝔵, 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ.with_disp(0)) would emit 0x8b, 0x05, 0x00, 0x00, 0x00, 0x00
//   2. Legacy 16ᵇⁱᵗ 8086 address — segment, base, index and 16ᵇⁱᵗ displacement. Supported in “legacy” 16ᵇⁱᵗ/32ᵇⁱᵗ assemblers.
//      All optional except displacemet (which can be zero).
//   3. Modern 32ᵇⁱᵗ/64ᵇⁱᵗ address — segment, base, index, scale and 32ᵇⁱᵗ displacement.
//      All optional except for scale (which can be 1 and that's default) and displacement (which can be zero).
//   4. Gather address — segment, base, index, scale and 32ᵇⁱᵗ displacement.
//      Similar to previous one, but index is vector register and, more importantly, it's not optional.
//
// Also some instructions only support limited addresses:
//   𝔢𝔦𝔭/𝔯𝔦𝔭 based address in ₓ86_64 mode — segment and displacement from 𝔢𝔦𝔭/𝔯𝔦𝔭
//
//   𝔠𝔪𝔭𝔰/𝔦𝔫𝔰/𝔪𝔬𝔳𝔰/𝔰𝔠𝔞𝔰 — destination: only segment and index which must be 𝔡𝔦/𝔡𝔰𝔦/𝔡𝔰𝔦 (depending on address size)
//   𝔠𝔪𝔭𝔰/𝔩𝔬𝔡𝔰/𝔪𝔬𝔳𝔰/𝔬𝔲𝔱𝔰/𝔰𝔱𝔬𝔰 — source: only segment and index which must be 𝔰𝔦/𝔢𝔰𝔦/𝔯𝔰𝔦 (depending on address size)
//   𝔵𝔩𝔞𝔱 — only segment and base which must be 𝔟𝔵/𝔢𝔟𝔵/𝔯𝔟𝔵 (depending on address size)
//
// These have their own types used in the implementation of instructions, but use standard 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_16ᵇⁱᵗ/𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ/𝔞𝔡𝔡𝔯𝔢𝔰𝔰_64ᵇⁱᵗ
// fluent constants and machinery.
#[allow(non_upper_case_globals)]
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮: Default, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    pub 𝗈𝖿𝖿𝗌𝖾𝗍: 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮,
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮: Default> 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 0>
{
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_byte_ptr(
        self,
    ) -> 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 1> {
        𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 1> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝗈𝖿𝖿𝗌𝖾𝗍: self.𝗈𝖿𝖿𝗌𝖾𝗍}
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_word_ptr(
        self,
    ) -> 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 2> {
        𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 2> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝗈𝖿𝖿𝗌𝖾𝗍: self.𝗈𝖿𝖿𝗌𝖾𝗍}
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_dword_ptr(
        self,
    ) -> 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 4> {
        𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 4> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝗈𝖿𝖿𝗌𝖾𝗍: self.𝗈𝖿𝖿𝗌𝖾𝗍}
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_qword_ptr(
        self,
    ) -> 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 8> {
        𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 8> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝗈𝖿𝖿𝗌𝖾𝗍: self.𝗈𝖿𝖿𝗌𝖾𝗍}
    }
    // Generic memory region. Mostly for completeness because all instructions which use arbitrarily sized regions don't need that
    // for disambigution.
    #[allow(clippy::wrong_self_convention)]
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    pub fn as_sized_ptr<const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>(
        self,
    ) -> 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝗈𝖿𝖿𝗌𝖾𝗍: self.𝗈𝖿𝖿𝗌𝖾𝗍}
    }
}

#[allow(non_upper_case_globals)]
impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮: Default,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮>,
        const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize,
    > From<𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>>
for 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_ₓ86_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    #[inline(always)]
    fn from(
        new_address: 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    ) -> Self {
        Self {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_address.𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into(),
            𝗈𝖿𝖿𝗌𝖾𝗍: new_address.𝗈𝖿𝖿𝗌𝖾𝗍.into(),
        }
    }
}

#[allow(non_upper_case_globals)]
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: core::marker::PhantomData<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
    pub 𝗂𝗇𝖽𝖾𝗑: core::marker::PhantomData<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮> 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 0>
{
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_byte_ptr(
        self,
    ) -> 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 1> {
        𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 1> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑}
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_word_ptr(
        self,
    ) -> 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 2> {
        𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 2> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑}
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_dword_ptr(
        self,
    ) -> 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 4> {
        𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 4> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑}
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_qword_ptr(
        self,
    ) -> 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 8> {
        𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 8> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑}
    }
    // Generic memory region. Mostly for completeness because all instructions which use arbitrarily sized regions don't need that
    // for disambigution.
    #[allow(clippy::wrong_self_convention)]
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    pub fn as_sized_ptr<const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>(
        self,
    ) -> 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑}
    }
}

#[allow(non_upper_case_globals)]
impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮> + 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒔𝒆𝒈𝒎𝒆𝒏𝒕,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮> + 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙,
        const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize,
    > From<𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>>
for 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    #[inline(always)]
    fn from(
        _: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    ) -> Self {
        Self {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: core::marker::PhantomData,
            𝗂𝗇𝖽𝖾𝗑: core::marker::PhantomData,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮> + 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒔𝒆𝒈𝒎𝒆𝒏𝒕,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮> + 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙,
        const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize,
    > From<𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>>
for 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    #[inline(always)]
    fn from(
        _: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    ) -> Self {
        Self {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: core::marker::PhantomData,
            𝗂𝗇𝖽𝖾𝗑: core::marker::PhantomData,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮> + 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒔𝒆𝒈𝒎𝒆𝒏𝒕,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮> + 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙,
        const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize,
    > From<𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>>
for 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    #[inline(always)]
    fn from(
        _: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    ) -> Self {
        Self {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: core::marker::PhantomData,
            𝗂𝗇𝖽𝖾𝗑: core::marker::PhantomData,
        }
    }
}

pub trait 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒔𝒆𝒈𝒎𝒆𝒏𝒕 {}
// If you don't specify segment 𝔢𝔰 is used automatically. Doesn't have to be specified and in 64ᵇⁱᵗ x86-64 mode can not be specified.
impl 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒔𝒆𝒈𝒎𝒆𝒏𝒕 for 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {}
pub trait 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙 {}

#[allow(non_upper_case_globals)]
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    pub 𝗂𝗇𝖽𝖾𝗑: core::marker::PhantomData<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮> 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 0>
{
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_byte_ptr(
        self,
    ) -> 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 1> {
        𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 1> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑}
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_word_ptr(
        self,
    ) -> 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 2> {
        𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 2> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑}
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_dword_ptr(
        self,
    ) -> 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 4> {
        𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 4> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑}
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_qword_ptr(
        self,
    ) -> 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 8> {
        𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 8> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑}
    }
    // Generic memory region. Mostly for completeness because all instructions which use arbitrarily sized regions don't need that
    // for disambigution.
    #[allow(clippy::wrong_self_convention)]
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    pub fn as_sized_ptr<const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>(
        self,
    ) -> 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑}
    }
}

#[allow(non_upper_case_globals)]
impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮> + 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙,
        const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize,
    > From<𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>>
for 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    #[inline(always)]
    fn from(
        new_address: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    ) -> Self {
        Self {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_address.𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into(),
            𝗂𝗇𝖽𝖾𝗑: core::marker::PhantomData,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮> + 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙,
        const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize,
    > From<𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>>
for 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    #[inline(always)]
    fn from(
        new_address: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    ) -> Self {
        Self {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_address.𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into(),
            𝗂𝗇𝖽𝖾𝗑: core::marker::PhantomData,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮> + 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙,
        const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize,
    > From<𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>>
for 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    #[inline(always)]
    fn from(
        new_address: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    ) -> Self {
        Self {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_address.𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into(),
            𝗂𝗇𝖽𝖾𝗑: core::marker::PhantomData,
        }
    }
}

pub trait 𝒔𝒕𝒓𝒊𝒏𝒈_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙 {}

#[allow(non_upper_case_globals)]
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    pub 𝖻𝖺𝗌𝖾: core::marker::PhantomData<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮> 𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 0>
{
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_byte_ptr(
        self,
    ) -> 𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 1> {
        𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 1> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾}
    }
    // Generic memory region. Mostly for completeness because all instructions which use arbitrarily sized regions don't need that
    // for disambigution.
    #[allow(clippy::wrong_self_convention)]
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    pub fn as_sized_ptr<const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>(
        self,
    ) -> 𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍, 𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾}
    }
}

#[allow(non_upper_case_globals)]
impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮> + 𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒃𝒂𝒔𝒆,
        const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize,
    > From<𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>>
for 𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    #[inline(always)]
    fn from(
        new_address: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    ) -> Self {
        Self {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_address.𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into(),
            𝖻𝖺𝗌𝖾: core::marker::PhantomData,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮> + 𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒃𝒂𝒔𝒆,
        const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize,
    > From<𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>>
for 𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    #[inline(always)]
    fn from(
        new_address: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    ) -> Self {
        Self {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_address.𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into(),
            𝖻𝖺𝗌𝖾: core::marker::PhantomData,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮> + 𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒃𝒂𝒔𝒆,
        const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize,
    > From<𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>>
for 𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    #[inline(always)]
    fn from(
        new_address: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    ) -> Self {
        Self {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_address.𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into(),
            𝖻𝖺𝗌𝖾: core::marker::PhantomData,
        }
    }
}

pub trait 𝒙𝒍𝒂𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒃𝒂𝒔𝒆 {}

#[allow(non_upper_case_globals)]
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    pub 𝖻𝖺𝗌𝖾: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    pub 𝗂𝗇𝖽𝖾𝗑: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default>
    𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 0>
{
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_byte_ptr(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 1> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 1> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_word_ptr(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 2> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 2> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_dword_ptr(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 4> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 4> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_qword_ptr(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 8> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 8> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    // We couldn't distingush far pointer from near pointer by size since 16ᵇⁱᵗ far pointer have the same size as 32ᵇⁱᵗ near pointer
    // and they both can be used in 16ᵇⁱᵗ and 32ᵇⁱᵗ modes.  Use negative sizes instead.
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_far_ptr16(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -4isize as usize }> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -4isize as usize }> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_far_ptr32(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -6isize as usize }> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -6isize as usize }> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    // Generic memory region. Mostly for completeness because all instructions which use arbitrarily sized regions don't need that
    // for disambigution.
    #[allow(clippy::wrong_self_convention)]
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    pub fn as_sized_ptr<const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>,
        const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize,
    > From<𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>>
for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    #[inline(always)]
    fn from(
        new_address: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    ) -> Self {
        Self {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_address.𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into(),
            𝖻𝖺𝗌𝖾: new_address.𝖻𝖺𝗌𝖾.into(),
            𝗂𝗇𝖽𝖾𝗑: new_address.𝗂𝗇𝖽𝖾𝗑.into(),
            𝖽𝗂𝗌𝗉: new_address.𝖽𝗂𝗌𝗉.into(),
        }
    }
}

#[allow(non_upper_case_globals)]
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: Default, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    pub 𝖻𝖺𝗌𝖾: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    pub 𝗂𝗇𝖽𝖾𝗑: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    pub 𝗌𝖼𝖺𝗅𝖾: 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: Default, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default>
    𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 0>
{
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_byte_ptr(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 1> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 1> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_word_ptr(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 2> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 2> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_dword_ptr(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 4> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 4> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_qword_ptr(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 8> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 8> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    // We couldn't distingush far pointer from near pointer by size since 16ᵇⁱᵗ far pointer have the same size as 32ᵇⁱᵗ near pointer
    // and they both can be used in 16ᵇⁱᵗ and 32ᵇⁱᵗ modes.  Use negative sizes instead.
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_far_ptr16(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -4isize as usize }> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -4isize as usize }> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_far_ptr32(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -6isize as usize }> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -6isize as usize }> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_far_ptr64(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -10isize as usize }> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -10isize as usize }> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    // Generic memory region. Mostly for completeness because all instructions which use arbitrarily sized regions don't need that
    // for disambigution.
    #[allow(clippy::wrong_self_convention)]
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    pub fn as_sized_ptr<const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: Default,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: core::convert::Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>,
        const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize,
    >
    From<
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        >,
    >
    for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    fn from(
        new_address: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        >,
    ) -> Self {
        Self {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_address.𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into(),
            𝖻𝖺𝗌𝖾: new_address.𝖻𝖺𝗌𝖾.into(),
            𝗂𝗇𝖽𝖾𝗑: new_address.𝗂𝗇𝖽𝖾𝗑.into(),
            𝗌𝖼𝖺𝗅𝖾: new_address.𝗌𝖼𝖺𝗅𝖾.into(),
            𝖽𝗂𝗌𝗉: new_address.𝖽𝗂𝗌𝗉.into(),
        }
    }
}

#[allow(non_upper_case_globals)]
pub const 𝔞𝔟𝔰𝔬𝔩𝔲𝔱𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰: 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 0> = 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 {
    𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {}, 𝗈𝖿𝖿𝗌𝖾𝗍: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
};

#[allow(non_upper_case_globals)]
pub const 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_16ᵇⁱᵗ: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    0,
> = 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086 {
    𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
    𝖻𝖺𝗌𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ {},
    𝗂𝗇𝖽𝖾𝗑: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ {},
    𝖽𝗂𝗌𝗉: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
};

#[allow(non_upper_case_globals)]
pub const 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    0,
> = 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86 {
    𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
    𝖻𝖺𝗌𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ {},
    𝗂𝗇𝖽𝖾𝗑: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ {},
    𝗌𝖼𝖺𝗅𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
    𝖽𝗂𝗌𝗉: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
};

#[allow(non_upper_case_globals)]
pub const 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_64ᵇⁱᵗ: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    0,
> = 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86 {
    𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
    𝖻𝖺𝗌𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ {},
    𝗂𝗇𝖽𝖾𝗑: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ {},
    𝗌𝖼𝖺𝗅𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
    𝖽𝗂𝗌𝗉: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
};

// Fluent interface requires the ability to find out type from arguments: Rust doesn't try to do complex reasoning when you have
// something like 𝔞𝔡𝔡𝔯𝔢𝔰𝔰.with_base(…).with_index(…).with_disp(…).
//
// Collect all arguments into 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_{8086,ₓ86} instead and then provide conversions into proper addess.
#[allow(non_upper_case_globals)]
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    pub 𝗈𝖿𝖿𝗌𝖾𝗍: 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮,
}

#[allow(non_upper_case_globals)]
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    pub 𝖻𝖺𝗌𝖾: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    pub 𝗂𝗇𝖽𝖾𝗑: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
}

#[allow(non_upper_case_globals)]
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    pub 𝖻𝖺𝗌𝖾: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    pub 𝗂𝗇𝖽𝖾𝗑: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    pub 𝗌𝖼𝖺𝗅𝖾: 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
}

#[allow(non_upper_case_globals)]
impl<𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    #[inline(always)]
    pub fn with_segment<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>(
        self,
        new_segment: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    ) -> 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_segment,
            𝗈𝖿𝖿𝗌𝖾𝗍: self.𝗈𝖿𝖿𝗌𝖾𝗍,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
    #[inline(always)]
    pub fn with_offset<𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮>(
        self,
        new_offset: 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮,
    ) -> 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝗈𝖿𝖿𝗌𝖾𝗍: new_offset,
        }
    }
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮>
    𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 0> {
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_byte_ptr(
        self,
    ) -> 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 1> {
        𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 1> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝗈𝖿𝖿𝗌𝖾𝗍: self.𝗈𝖿𝖿𝗌𝖾𝗍,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_word_ptr(
        self,
    ) -> 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 2> {
        𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 2> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝗈𝖿𝖿𝗌𝖾𝗍: self.𝗈𝖿𝖿𝗌𝖾𝗍,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_dword_ptr(
        self,
    ) -> 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 4> {
        𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 4> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝗈𝖿𝖿𝗌𝖾𝗍: self.𝗈𝖿𝖿𝗌𝖾𝗍,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_qword_ptr(
        self,
    ) -> 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 8> {
        𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 8> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝗈𝖿𝖿𝗌𝖾𝗍: self.𝗈𝖿𝖿𝗌𝖾𝗍,
        }
    }
    // Generic memory region. Mostly for completeness because all instructions which use arbitrarily sized regions don't need that
    // for disambigution.
    #[allow(clippy::wrong_self_convention)]
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    pub fn as_sized_ptr<const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>(
        self,
    ) -> 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓸𝓯𝓯𝓼𝓮𝓽_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝗈𝖿𝖿𝗌𝖾𝗍: self.𝗈𝖿𝖿𝗌𝖾𝗍,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    #[inline(always)]
    pub fn with_segment<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>(
        self,
        new_segment: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_segment,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
    #[inline(always)]
    pub fn with_base<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>(
        self,
        new_base: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: new_base,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
    #[inline(always)]
    pub fn with_index<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>(
        self,
        new_index: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: new_index,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
    #[inline(always)]
    pub fn with_disp<𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>(
        self,
        new_disp: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: new_disp,
        }
    }
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 0> {
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_byte_ptr(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 1> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 1> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_word_ptr(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 2> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 2> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_dword_ptr(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 4> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 4> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_qword_ptr(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 8> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 8> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    // We couldn't distingush far pointer from near pointer by size since 16ᵇⁱᵗ far pointer have the same size as 32ᵇⁱᵗ near pointer
    // and they both can be used in 16ᵇⁱᵗ and 32ᵇⁱᵗ modes.  Use negative sizes instead.
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_far_ptr16(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -4isize as usize }> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -4isize as usize }> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_far_ptr32(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -6isize as usize }> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -6isize as usize }> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    // Generic memory region. Mostly for completeness because all instructions which use arbitrarily sized regions don't need that
    // for disambigution.
    #[allow(clippy::wrong_self_convention)]
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    pub fn as_sized_ptr<const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
    #[inline(always)]
    pub fn with_segment<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>(
        self,
        new_segment: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_segment,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
    #[inline(always)]
    pub fn with_base<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>(
        self,
        new_base: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: new_base,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
    #[inline(always)]
    pub fn with_base<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>(
        self,
        new_base: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: new_base,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
    #[inline(always)]
    pub fn with_index<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>(
        self,
        new_index: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: new_index,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
    #[inline(always)]
    pub fn with_index<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>(
        self,
        new_index: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: new_index,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
    #[inline(always)]
    pub fn with_scale<𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮>(
        self,
        new_scale: 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: new_scale,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
    #[inline(always)]
    pub fn with_disp<𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>(
        self,
        new_disp: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: new_disp,
        }
    }
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 0>
{
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_byte_ptr(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 1> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 1> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_word_ptr(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 2> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 2> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_dword_ptr(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 4> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 4> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_qword_ptr(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 8> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 8> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    // We couldn't distingush far pointer from near pointer by size since 16ᵇⁱᵗ far pointer have the same size as 32ᵇⁱᵗ near pointer
    // and they both can be used in 16ᵇⁱᵗ and 32ᵇⁱᵗ modes.  Use negative sizes instead.
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_far_ptr16(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -4isize as usize }> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -4isize as usize }> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_far_ptr32(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -6isize as usize }> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -6isize as usize }> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn as_far_ptr64(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -10isize as usize }> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { -10isize as usize }> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    // Generic memory region. Mostly for completeness because all instructions which use arbitrarily sized regions don't need that
    // for disambigution.
    #[allow(clippy::wrong_self_convention)]
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    pub fn as_sized_ptr<const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮 > {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
}

macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖎𝖓𝖙𝖊𝖌𝖊𝖗_𝖋𝖗𝖔𝖒 {
    ($($𝓲𝓷𝓽_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮:ident),*) => {
        $(
            impl From<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞> for $𝓲𝓷𝓽_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                #[inline(always)]
                fn from(_: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞) -> $𝓲𝓷𝓽_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                    0
                }
            }
         )*
    }
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖎𝖓𝖙𝖊𝖌𝖊𝖗_𝖋𝖗𝖔𝖒!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);

impl<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮> From<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞> for core::num::Wrapping<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>
where 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞: core::convert::Into<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>
{
    #[inline(always)]
    fn from(value: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞) -> Self {
        Self(value.into())
    }
}

// Any 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 type can be converted into the appropriate Option (and will end up as None, of course).
impl<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮: 𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆> From<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞> for Option<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮> {
    #[inline(always)]
    fn from(_: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞) -> Self {
        None
    }
}

impl<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮: 𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆> From<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ> for Option<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮> {
    #[inline(always)]
    fn from(_: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ) -> Self {
        None
    }
}

impl<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮: 𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆> From<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ> for Option<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮> {
    #[inline(always)]
    fn from(_: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ) -> Self {
        None
    }
}

impl<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮: 𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆> From<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ> for Option<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮> {
    #[inline(always)]
    fn from(_: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ) -> Self {
        None
    }
}

impl 𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆 for 𝐧𝐨_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 {}
