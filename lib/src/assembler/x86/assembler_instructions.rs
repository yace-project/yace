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

// We are implementing two-level scheme with a set of ₓₓₓ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔 traits and a set of ₓₓₓ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏 traits.
//
// This allows us to avoid combinatiorial explosion: there are more than dozen of types which may represent just general purpose
// register argument and for three-argument instruction it would mean there are almost two thousand variants.
//
// ₓₓₓ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔 uses traits below to convert arguments to less diverse set and then ₓₓₓ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏 implement the remaining
// combinations.
//
// If all arguments would be handled identically, then of course, it wouldn't make much sense to even have these two levels.
// Instead some instructions use not 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕 trait, but more specialized conversion traits.
//
// E.g. shift instructions use 𝒔𝒉𝒊𝒇𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕 trait which only support i8 and 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ arguments (and they
// just accept them without conversion).
//
// Thus way we both avoid the combinatorial explosion and guarantee that incorrect registers are excluded during the compilation
// time. Not only this makes debugging easier, this also means that we can stol thinking about reporting these particular errors.
//
// Note: Even with this approach we have some extra variants to implement (e.g. add have separate version for accumulator and
// 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫 and 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 because there are special version for 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫 and immediate, but overral it's less than 2x
// superfluous instructions.  This is considered acceptable.

use super::𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏;
use super::𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓;

pub trait 𝒂𝒅𝒅_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮> {
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;
    fn add_implementation(
        &mut self,
        arguments: 𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮,
    ) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
}

pub trait 𝒂𝒅𝒅_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮> {
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;
    fn add_forwarder(
        &mut self,
        arguments: 𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻_𝓽𝓾𝓹𝓵𝓮,
    ) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
}

impl<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻1_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻2_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒂𝒅𝒅_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
        (<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻1_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
         <𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻2_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭)>>
𝒂𝒅𝒅_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔<(𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻1_𝓽𝔂𝓹𝓮, 𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻2_𝓽𝔂𝓹𝓮)>
for 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮
{
    #[allow(clippy::type_complexity)]
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = <𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as 𝒂𝒅𝒅_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
        (<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻1_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
         <𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻2_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭)>>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;
    #[allow(clippy::type_complexity)]
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = <𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as 𝒂𝒅𝒅_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
        (<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻1_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
         <𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻2_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭)>>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;
    #[inline(always)]
    fn add_forwarder(&mut self, (parameter1, parameter2): (𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻1_𝓽𝔂𝓹𝓮, 𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻2_𝓽𝔂𝓹𝓮)) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.add_implementation((
            Into::<<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻1_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>::into(parameter1),
            Into::<<𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻2_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>::into(parameter2)))
    }
}
