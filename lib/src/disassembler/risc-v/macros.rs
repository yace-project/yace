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

macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖎𝖘𝖈𝖛_𝖉𝖎𝖘𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖙𝖗𝖆𝖎𝖙 {
    (   $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        pub trait $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident {
           $($𝓮𝔁𝓽𝓻𝓪_𝓭𝓮𝓯𝓲𝓷𝓮𝓼:tt)*
        }
      ) => {
        𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! {
            𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖎𝖘𝖈𝖛_𝖉𝖎𝖘𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖙𝖗𝖆𝖎𝖙! {
                $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
                pub trait $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮: Sized
                where
                    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞:
                        𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐩𝐫𝐞𝐟𝐞𝐭𝐜𝐡_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        𝒛𝒆𝒓𝒐_𝒐𝒇𝒇𝒔𝒆𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜>,
                    𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧:
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐛𝐫𝐚𝐧𝐜𝐡_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐣𝐮𝐦𝐩_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐩𝐫𝐞𝐟𝐞𝐭𝐜𝐡_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐬𝐡𝐢𝐟𝐭_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐮𝐩𝐩𝐞𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞>
                Ξ𝔯𝔳64[+ Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐰𝐨𝐫𝐝_𝐬𝐡𝐢𝐟𝐭_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞>],
                    i32:
                        TryInto<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐛𝐫𝐚𝐧𝐜𝐡_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        TryInto<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        TryInto<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        TryInto<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐣𝐮𝐦𝐩_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        TryInto<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐬𝐡𝐢𝐟𝐭_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        TryInto<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐮𝐩𝐩𝐞𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞>,
                    u32:
                        TryInto<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        TryInto<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐚𝐛𝐢> +
                        TryInto<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        TryInto<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐚𝐛𝐢> +
                        TryInto<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        TryInto<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐚𝐛𝐢> +
                        TryInto<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> +
                        TryInto<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐫𝐨𝐮𝐧𝐝𝐢𝐧𝐠_𝐦𝐨𝐝𝐞> +
                        TryInto<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐟𝐞𝐧𝐜𝐞>
                {
                    type 𝓒𝓟𝓤_𝓽𝔂𝓹𝓮: 𝑪𝑷𝑼;
                    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;
                    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;

                    fn instruction(self, instruction: 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝, operands: &[𝐨𝐩𝐞𝐫𝐚𝐧𝐝<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>])
                        -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
                    fn unimplemented_16bit_instruction(self, instruction: u16) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
                    fn unimplemented_32bit_instruction(self, instruction: u32) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;

                    fn decode<𝓹𝓻𝓸𝓭𝓾𝓬𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓<𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> + 𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>(self, mut producer: 𝓹𝓻𝓸𝓭𝓾𝓬𝓮𝓻_𝓽𝔂𝓹𝓮)
                        -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓹𝓻𝓸𝓭𝓾𝓬𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
                    {
                        // Long instructions use bits 12-14 and 2-6 for opcode but short instruction use at least bits 15-13 and
                        // 1-0 for it.
                        //
                        // Different instruction have differently sized opcodes, longest ones are 𝔠.𝔞𝔫𝔡, 𝔠.𝔬𝔯, 𝔠.𝔰𝔲𝔟, 𝔠.𝔵𝔬𝔯 where
                        // bits 15-10, 6-5, and 1-0 are used as opcode.
                        //
                        // Since bits 2-6 are also used as immediates in short instruction we split low 16 bits in the following
                        // parts: bits 15-10 and 1-0 are compressed_instruction_opcode while bits 6-2 are full_instruction_opcode
                        // and 6-9 becomes rd_field (although it includes only 3 bits, but you get the remaining two during
                        // processing of the compressed_instruction_opcode).

                        let parcel0: u16 = producer.get_u16()?.to_le();
                        let compressed_instruction_opcode = (((parcel0 >> 8) & 0xfc) + (parcel0 & 0x3));
                        let opcode = ((parcel0 >> 2) & 0x1f);
                        let rd_bits = ((parcel0 >> 7) & 0x07);

                        let compressed_instruction_step =
                            𝘁𝗮𝗯𝗹𝗲𝘀::Ξ𝔯𝔳32[𝔠𝔬𝔪𝔭𝔯𝔢𝔰𝔢𝔡_𝔰𝔱𝔢𝔭_𝔡𝔦𝔰𝔭𝔞𝔱𝔠𝔥_𝐫𝐯𝟑𝟐]Ξ𝔯𝔳64[𝔠𝔬𝔪𝔭𝔯𝔢𝔰𝔢𝔡_𝔰𝔱𝔢𝔭_𝔡𝔦𝔰𝔭𝔞𝔱𝔠𝔥_𝐫𝐯𝟔𝟒][compressed_instruction_opcode as usize];

                        type 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩 = 𝘁𝗮𝗯𝗹𝗲𝘀::Ξ𝔯𝔳32[𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐]Ξ𝔯𝔳64[𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒];

                        let instruction_bits = if compressed_instruction_step.1 <= 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_7 {
                            let parcel1: u16 = producer.get_u16()?.to_le();
                            #[cfg(target_endian = "big")]
                            {(parcel1 as u32) + (parcel0 as u32) << 16}
                            #[cfg(target_endian = "little")]
                            {(parcel1 as u32) << 16 + (parcel0 as u32)}
                        } else {
                            parcel0 as u32
                        };

                        'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ: {
                            match compressed_instruction_step.1 {
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_0 => {
                                    𝔻𝕖𝕔𝕠𝕕𝕖𝟛𝟚𝕓𝕚𝕥𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟(
                                        opcode,
                                        0,
                                        <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔯𝔫𝔢,
                                        rd_bits,
                                        compressed_instruction_step.0,
                                        instruction_bits);
                                    return self.unimplemented_32bit_instruction(instruction_bits);
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_1 => {
                                    𝔻𝕖𝕔𝕠𝕕𝕖𝟛𝟚𝕓𝕚𝕥𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟(
                                        opcode,
                                        1,
                                        <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔯𝔱𝔷,
                                        rd_bits,
                                        compressed_instruction_step.0,
                                        instruction_bits);
                                    return self.unimplemented_32bit_instruction(instruction_bits);
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_2 => {
                                    𝔻𝕖𝕔𝕠𝕕𝕖𝟛𝟚𝕓𝕚𝕥𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟(
                                        opcode,
                                        2,
                                        <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔯𝔡𝔫,
                                        rd_bits,
                                        compressed_instruction_step.0,
                                        instruction_bits);
                                    return self.unimplemented_32bit_instruction(instruction_bits);
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_3 => {
                                    𝔻𝕖𝕔𝕠𝕕𝕖𝟛𝟚𝕓𝕚𝕥𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟(
                                        opcode,
                                        3,
                                        <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔯𝔲𝔭,
                                        rd_bits,
                                        compressed_instruction_step.0,
                                        instruction_bits);
                                    return self.unimplemented_32bit_instruction(instruction_bits);
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_4 => {
                                    𝔻𝕖𝕔𝕠𝕕𝕖𝟛𝟚𝕓𝕚𝕥𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟(
                                        opcode,
                                        4,
                                        <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔯𝔪𝔪,
                                        rd_bits,
                                        compressed_instruction_step.0,
                                        instruction_bits);
                                    return self.unimplemented_32bit_instruction(instruction_bits);
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_5 => {
                                    𝔻𝕖𝕔𝕠𝕕𝕖𝟛𝟚𝕓𝕚𝕥𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟(
                                        opcode,
                                        5,
                                        rm,
                                        rd_bits,
                                        compressed_instruction_step.0,
                                        instruction_bits);
                                    return self.unimplemented_32bit_instruction(instruction_bits);
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_6 => {
                                    𝔻𝕖𝕔𝕠𝕕𝕖𝟛𝟚𝕓𝕚𝕥𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟(
                                        opcode,
                                        6,
                                        rm,
                                        rd_bits,
                                        compressed_instruction_step.0,
                                        instruction_bits);
                                    return self.unimplemented_32bit_instruction(instruction_bits);
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_7 => {
                                    𝔻𝕖𝕔𝕠𝕕𝕖𝟛𝟚𝕓𝕚𝕥𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟(
                                        opcode,
                                        7,
                                        <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔡𝔶𝔫,
                                        rd_bits,
                                        compressed_instruction_step.0,
                                        instruction_bits);
                                    return self.unimplemented_32bit_instruction(instruction_bits);
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔞𝔡𝔡𝔦4𝔰𝔭𝔫 => {
                                    let Ok(rd) = (𝘁𝗮𝗯𝗹𝗲𝘀::𝔯𝔡_𝔭𝔯𝔦𝔪𝔢[opcode as usize] as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let imm = ((compressed_instruction_step.0 as u8 +
                                                𝘁𝗮𝗯𝗹𝗲𝘀::𝔞𝔡𝔡𝔦4𝔰𝔭𝔫[opcode as usize]) as i32) << 2 +
                                              (rd_bits as i32) << 6;
                                    if imm == 0 {
                                        return self.unimp(());
                                    }
                                    let Ok(imm) = imm.try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.addi((rd, <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), imm));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔣𝔩𝔡 => {
                                    let Ok(rd) = (𝘁𝗮𝗯𝗹𝗲𝘀::𝔯𝔡_𝔭𝔯𝔦𝔪𝔢[opcode as usize] as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (compressed_instruction_step.0 as i32 +
                                                   𝘁𝗮𝗯𝗹𝗲𝘀::𝔩𝔡[opcode as usize] as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(rs1) = ((8 + rd_bits) as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                        return self.fld((rd, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: rs1, 𝖽𝗂𝗌𝗉: imm}));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔩𝔴 => {
                                    let Ok(rd) = (𝘁𝗮𝗯𝗹𝗲𝘀::𝔯𝔡_𝔭𝔯𝔦𝔪𝔢[opcode as usize] as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (compressed_instruction_step.0 as i32 +
                                                   𝘁𝗮𝗯𝗹𝗲𝘀::𝔩𝔴[opcode as usize] as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(rs1) = ((8 + rd_bits) as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.lw((rd, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: rs1, 𝖽𝗂𝗌𝗉: imm}));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::Ξ𝔯𝔳32[𝔠_𝔣𝔩𝔴]Ξ𝔯𝔳64[𝔠_𝔩𝔡] => {
                                    let Ok(rd) = (𝘁𝗮𝗯𝗹𝗲𝘀::𝔯𝔡_𝔭𝔯𝔦𝔪𝔢[opcode as usize] as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (compressed_instruction_step.0 as i32 +
                                                   𝘁𝗮𝗯𝗹𝗲𝘀::Ξ𝔯𝔳32[𝔩𝔴]Ξ𝔯𝔳64[𝔩𝔡][opcode as usize] as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(rs1) = ((8 + rd_bits) as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.Ξ𝔯𝔳32[flw]Ξ𝔯𝔳64[ld]((rd, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: rs1, 𝖽𝗂𝗌𝗉: imm}));
                                }
                                Ξ𝔯𝔳32[𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔞𝔡𝔡𝔴_𝔰𝔲𝔟𝔴|] 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔯𝔢𝔰𝔢𝔯𝔳𝔢𝔡 => return self.unimp(()),
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔣𝔰𝔡 => {
                                    let Ok(rs2) = (𝘁𝗮𝗯𝗹𝗲𝘀::𝔯𝔡_𝔭𝔯𝔦𝔪𝔢[opcode as usize] as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (compressed_instruction_step.0 as i32 +
                                                   𝘁𝗮𝗯𝗹𝗲𝘀::𝔩𝔡[opcode as usize] as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(rs1) = ((8 + rd_bits) as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                        return self.fsd((rs2, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: rs1, 𝖽𝗂𝗌𝗉: imm}));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔰𝔴 => {
                                    let Ok(rs2) = (𝘁𝗮𝗯𝗹𝗲𝘀::𝔯𝔡_𝔭𝔯𝔦𝔪𝔢[opcode as usize] as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (compressed_instruction_step.0 as i32 +
                                                   𝘁𝗮𝗯𝗹𝗲𝘀::𝔩𝔴[opcode as usize] as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(rs1) = ((8 + rd_bits) as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.sw((rs2, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: rs1, 𝖽𝗂𝗌𝗉: imm}));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::Ξ𝔯𝔳32[𝔠_𝔣𝔰𝔴]Ξ𝔯𝔳64[𝔠_𝔰𝔡] => {
                                    let Ok(rs2) = (𝘁𝗮𝗯𝗹𝗲𝘀::𝔯𝔡_𝔭𝔯𝔦𝔪𝔢[opcode as usize] as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (compressed_instruction_step.0 as i32 +
                                                   𝘁𝗮𝗯𝗹𝗲𝘀::Ξ𝔯𝔳32[𝔩𝔴]Ξ𝔯𝔳64[𝔩𝔡][opcode as usize] as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(rs1) = ((8 + rd_bits) as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.Ξ𝔯𝔳32[fsw]Ξ𝔯𝔳64[sd]((rs2, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: rs1, 𝖽𝗂𝗌𝗉: imm}));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00 => {
                                    if rd_bits == 0 {
                                        return self.nop(());
                                    }
                                    let Ok(rd): Result<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> =
                                        (rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (opcode as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.addi((rd.clone(), rd, imm));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00 => {
                                    if rd_bits == 0 {
                                        return self.nop(());
                                    }
                                    let Ok(rd): Result<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> =
                                        (rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (-32 + opcode as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.addi((rd.clone(), rd, imm));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 => {
                                    let Ok(rd): Result<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> =
                                        (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (opcode as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.addi((rd.clone(), rd, imm));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 => {
                                    let Ok(rd): Result<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> =
                                        (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (-32 + opcode as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.addi((rd.clone(), rd, imm));
                                }
                        Ξ𝔯𝔳32[  𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔧𝔞𝔩 => {
                                    let Ok(imm) = ((compressed_instruction_step.0 as i32) << 4 +
                                                   𝘁𝗮𝗯𝗹𝗲𝘀::𝔧_𝔥𝔦𝔤𝔥[rd_bits as usize] +
                                                   𝘁𝗮𝗯𝗹𝗲𝘀::𝔧_𝔩𝔬𝔴[opcode as usize] as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.jal((<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵1.into(), imm));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔧 => {
                                    let Ok(imm) = ((compressed_instruction_step.0 as i32) << 4 +
                                                   𝘁𝗮𝗯𝗹𝗲𝘀::𝔧_𝔥𝔦𝔤𝔥[rd_bits as usize] +
                                                   𝘁𝗮𝗯𝗹𝗲𝘀::𝔧_𝔩𝔬𝔴[opcode as usize] as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.jal((<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵0.into(), imm));
                                }]
                        Ξ𝔯𝔳64[  𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00 => {
                                    if rd_bits == 0 {
                                        return self.unimp(());
                                    }
                                    let Ok(rd): Result<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> =
                                        (rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (opcode as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.addiw((rd.clone(), rd, imm));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00 => {
                                    if rd_bits == 0 {
                                        return self.unimp(());
                                    }
                                    let Ok(rd): Result<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> =
                                        (rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (-32 + opcode as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.addiw((rd.clone(), rd, imm));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 => {
                                    let Ok(rd): Result<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> =
                                        (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (opcode as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.addiw((rd.clone(), rd, imm));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 => {
                                    let Ok(rd): Result<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> =
                                        (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (-32 + opcode as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.addiw((rd.clone(), rd, imm));
                                }]
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔩𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 => {
                                    let Ok(rd) = (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (opcode as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.addi((rd, <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵0.into(), imm));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔩𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 => {
                                    let Ok(rd) = (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (-32 + opcode as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.addi((rd, <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵0.into(), imm));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00 => {
                                    if opcode == 0 {
                                        return self.unimp(());
                                    }
                                    if rd_bits == 2 {
                                        let Ok(imm) = ((𝘁𝗮𝗯𝗹𝗲𝘀::𝔞𝔡𝔡𝔦16𝔰𝔭[opcode as usize] as i32) << 2).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                        return self.addi((<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(),
                                                          <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(),
                                                          imm));
                                    }
                                    let Ok(rd) = (rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = ((opcode as i32) << 12).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.lui((rd, imm));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00 => {
                                    if rd_bits == 2 {
                                        let Ok(imm) = (-512 + (𝘁𝗮𝗯𝗹𝗲𝘀::𝔞𝔡𝔡𝔦16𝔰𝔭[opcode as usize] as i32) << 2).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                        return self.addi((<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(),
                                                          <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(),
                                                          imm));
                                    }
                                    let Ok(rd) = (rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (-131072 + (opcode as i32) << 12).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.lui((rd, imm));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 => {
                                    if opcode == 0 {
                                        return self.unimp(());
                                    }
                                    let Ok(rd) = (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = ((opcode as i32) << 12).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.lui((rd, imm));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 => {
                                    let Ok(rd) = (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (-131072 + (opcode as i32) << 12).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.lui((rd, imm));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::Ξ𝔯𝔳32[𝔠_𝔰𝔯𝔩𝔦_𝔩𝔬𝔴]Ξ𝔯𝔳64[𝔠_𝔰𝔯𝔩𝔦] => {
                                    let Ok(rd): Result<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> =
                                        ((8 + rd_bits) as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (Ξ𝔯𝔳64[compressed_instruction_step.0 as i32 +] opcode as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.srli((rd.clone(), rd, imm));
                                }
                        Ξ𝔯𝔳32[  𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔰𝔯𝔩𝔦_𝔥𝔦𝔤𝔥 => return self.unimp(()),]
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::Ξ𝔯𝔳32[𝔠_𝔰𝔯𝔞𝔦_𝔩𝔬𝔴]Ξ𝔯𝔳64[𝔠_𝔰𝔯𝔞𝔦] => {
                                    let Ok(rd): Result<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> =
                                        ((8 + rd_bits) as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (Ξ𝔯𝔳64[compressed_instruction_step.0 as i32 +] opcode as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.srai((rd.clone(), rd, imm));
                                }
                        Ξ𝔯𝔳32[  𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔰𝔯𝔞𝔦_𝔥𝔦𝔤𝔥 => return self.unimp(()),]
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔞𝔫𝔡𝔦 => {
                                    let Ok(rd): Result<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> =
                                        ((8 + rd_bits) as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (compressed_instruction_step.0 as i32 + opcode as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.andi((rd.clone(), rd, imm));
                                }
                        Ξ𝔯𝔳32[  𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔣𝔰𝔴𝔰𝔭 => {
                                    let Ok(rs2) = (opcode as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (compressed_instruction_step.0 as i32 +
                                                   𝘁𝗮𝗯𝗹𝗲𝘀::𝔰𝔴𝔰𝔭[rd_bits as usize]).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.fsw((rs2, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), 𝖽𝗂𝗌𝗉: imm}));
                                }]
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔞𝔫𝔡_𝔬𝔯_𝔰𝔲𝔟_𝔵𝔬𝔯 => {
                                    let Ok(rs2) = (𝘁𝗮𝗯𝗹𝗲𝘀::𝔯𝔡_𝔭𝔯𝔦𝔪𝔢[opcode as usize] as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(rd): Result<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> =
                                        ((8 + rd_bits) as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return if opcode & 16 == 0 {
                                        if opcode & 8 == 0 {
                                            self.sub((rd.clone(), rd, rs2))
                                        } else {
                                            self.xor((rd.clone(), rd, rs2))
                                        }
                                    } else {
                                        if opcode & 8 == 0 {
                                            self.or((rd.clone(), rd, rs2))
                                        } else {
                                            self.and((rd.clone(), rd, rs2))
                                        }
                                    }
                                }
                        Ξ𝔯𝔳64[  𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔞𝔡𝔡𝔴_𝔰𝔲𝔟𝔴 => {
                                    let Ok(rs2) = (𝘁𝗮𝗯𝗹𝗲𝘀::𝔯𝔡_𝔭𝔯𝔦𝔪𝔢[opcode as usize] as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(rd): Result<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> =
                                        ((8 + rd_bits) as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return if opcode & 16 == 0 {
                                        if opcode & 8 == 0 {
                                            self.subw((rd.clone(), rd, rs2))
                                        } else {
                                            self.addw((rd.clone(), rd, rs2))
                                        }
                                    } else {
                                        self.unimp(())
                                    }
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔧 => {
                                    let Ok(imm) = ((compressed_instruction_step.0 as i32) << 4 +
                                                   𝘁𝗮𝗯𝗹𝗲𝘀::𝔧_𝔥𝔦𝔤𝔥[rd_bits as usize] +
                                                   𝘁𝗮𝗯𝗹𝗲𝘀::𝔧_𝔩𝔬𝔴[opcode as usize] as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.jal((<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵0.into(), imm));
                                }]
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔟𝔢𝔮𝔷 => {
                                    let Ok(rd) = ((8 + rd_bits) as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (((compressed_instruction_step.0 as i32) +
                                                    𝘁𝗮𝗯𝗹𝗲𝘀::𝔟𝔢𝔮𝔷[opcode as usize] as i32) << 1).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.beq((rd, <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵0.into(), imm));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔟𝔫𝔢𝔷 => {
                                    let Ok(rd) = ((8 + rd_bits) as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (((compressed_instruction_step.0 as i32) +
                                                    𝘁𝗮𝗯𝗹𝗲𝘀::𝔟𝔢𝔮𝔷[opcode as usize] as i32) << 1).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.bne((rd, <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵0.into(), imm));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔰𝔩𝔩𝔦_𝔩𝔬𝔴 => {
                                    let Ok(rd): Result<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> =
                                        (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (opcode as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.slli((rd.clone(), rd, imm));
                                }
                        Ξ𝔯𝔳32[  𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔰𝔩𝔩𝔦_𝔥𝔦𝔤𝔥 => return self.unimp(()),]
                        Ξ𝔯𝔳64[  𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔰𝔩𝔩𝔦_𝔥𝔦𝔤𝔥 => {
                                    let Ok(rd): Result<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> =
                                        (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (32 + opcode as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.slli((rd.clone(), rd, imm));
                                }]
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔣𝔩𝔡𝔰𝔭_𝔩𝔬𝔴 => {
                                    let Ok(rd) = (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = ((𝘁𝗮𝗯𝗹𝗲𝘀::𝔩𝔡𝔰𝔭[opcode as usize] as i32) << 3).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.fld((rd, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), 𝖽𝗂𝗌𝗉: imm}));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔣𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥 => {
                                    let Ok(rd) = (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (32 + ((𝘁𝗮𝗯𝗹𝗲𝘀::𝔩𝔡𝔰𝔭[opcode as usize] as i32) << 3)).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.fld((rd, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), 𝖽𝗂𝗌𝗉: imm}));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴_00 => {
                                    if rd_bits == 0 {
                                        return self.unimp(());
                                    }
                                    let Ok(rd) = (rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (𝘁𝗮𝗯𝗹𝗲𝘀::𝔩𝔴𝔰𝔭[opcode as usize] as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.lw((rd, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), 𝖽𝗂𝗌𝗉: imm}));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥_00 => {
                                    if rd_bits == 0 {
                                        return self.unimp(());
                                    }
                                    let Ok(rd) = (rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (32 + 𝘁𝗮𝗯𝗹𝗲𝘀::𝔩𝔴𝔰𝔭[opcode as usize] as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.lw((rd, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), 𝖽𝗂𝗌𝗉: imm}));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴 => {
                                    let Ok(rd) = (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (𝘁𝗮𝗯𝗹𝗲𝘀::𝔩𝔴𝔰𝔭[opcode as usize] as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.lw((rd, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), 𝖽𝗂𝗌𝗉: imm}));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥 => {
                                    let Ok(rd) = (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (32 + 𝘁𝗮𝗯𝗹𝗲𝘀::𝔩𝔴𝔰𝔭[opcode as usize] as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.lw((rd, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), 𝖽𝗂𝗌𝗉: imm}));
                                }
                        Ξ𝔯𝔳32[  𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔣𝔩𝔴𝔰𝔭_𝔩𝔬𝔴 => {
                                    let Ok(rd) = (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (𝘁𝗮𝗯𝗹𝗲𝘀::𝔩𝔴𝔰𝔭[opcode as usize] as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.flw((rd, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), 𝖽𝗂𝗌𝗉: imm}));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔣𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥 => {
                                    let Ok(rd) = (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (32 + 𝘁𝗮𝗯𝗹𝗲𝘀::𝔩𝔴𝔰𝔭[opcode as usize] as i32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.flw((rd, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), 𝖽𝗂𝗌𝗉: imm}));
                                }]
                        Ξ𝔯𝔳64[  𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔩𝔡𝔰𝔭_𝔩𝔬𝔴_00 => {
                                    if rd_bits == 0 {
                                        return self.unimp(());
                                    }
                                    let Ok(rd) = (rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = ((𝘁𝗮𝗯𝗹𝗲𝘀::𝔩𝔡𝔰𝔭[opcode as usize] as i32) << 3).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.ld((rd, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), 𝖽𝗂𝗌𝗉: imm}));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥_00 => {
                                    if rd_bits == 0 {
                                        return self.unimp(());
                                    }
                                    let Ok(rd) = (rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (32 + ((𝘁𝗮𝗯𝗹𝗲𝘀::𝔩𝔡𝔰𝔭[opcode as usize] as i32) << 3)).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.ld((rd, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), 𝖽𝗂𝗌𝗉: imm}));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔩𝔡𝔰𝔭_𝔩𝔬𝔴 => {
                                    let Ok(rd) = (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = ((𝘁𝗮𝗯𝗹𝗲𝘀::𝔩𝔡𝔰𝔭[opcode as usize] as i32) << 3).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.ld((rd, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), 𝖽𝗂𝗌𝗉: imm}));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥 => {
                                    let Ok(rd) = (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (32 + ((𝘁𝗮𝗯𝗹𝗲𝘀::𝔩𝔡𝔰𝔭[opcode as usize] as i32) << 3)).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.ld((rd, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), 𝖽𝗂𝗌𝗉: imm}));
                                }]
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔪𝔳_00 => {
                                    let Ok(rd) = (rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    if opcode == 0 {
                                        if rd_bits == 0 {
                                            return self.unimp(());
                                        }
                                        return self.jalr((<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵0.into(),
                                                          𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: rd, 𝖽𝗂𝗌𝗉: Default::default()}));
                                    }
                                    let Ok(rs2) = (opcode as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.add((rd, <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵0.into(), rs2));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔪𝔳 => {
                                    let Ok(rd) = (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    if opcode == 0 {
                                        return self.jalr((<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵0.into(),
                                                          𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: rd, 𝖽𝗂𝗌𝗉: Default::default()}));
                                    }
                                    let Ok(rs2) = (opcode as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.add((rd, <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵0.into(), rs2));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔞𝔡𝔡_00 => {
                                    let Ok(rd) = (rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    if opcode == 0 {
                                        if rd_bits == 0 {
                                            return self.ebreak(());
                                        }
                                        return self.jalr((<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵1.into(),
                                                          𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: rd, 𝖽𝗂𝗌𝗉: Default::default()}));
                                    }
                                    let Ok(rs2) = (opcode as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.add((rd.clone(), rd, rs2));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔞𝔡𝔡 => {
                                    let Ok(rd) = (compressed_instruction_step.0 as u32 + rd_bits as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    if opcode == 0 {
                                        return self.jalr((<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵1.into(),
                                                          𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: rd, 𝖽𝗂𝗌𝗉: Default::default()}));
                                    }
                                    let Ok(rs2) = (opcode as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.add((rd.clone(), rd, rs2));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔣𝔰𝔡𝔰𝔭 => {
                                    let Ok(rs2) = (opcode as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (compressed_instruction_step.0 as i32 + ((rd_bits as i32) << 6)).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.fsd((rs2, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), 𝖽𝗂𝗌𝗉: imm}));
                                }
                                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔰𝔴𝔰𝔭 => {
                                    let Ok(rs2) = (opcode as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (compressed_instruction_step.0 as i32 +
                                                   𝘁𝗮𝗯𝗹𝗲𝘀::𝔰𝔴𝔰𝔭[rd_bits as usize]).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.sw((rs2, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), 𝖽𝗂𝗌𝗉: imm}));
                                }
                        Ξ𝔯𝔳64[  𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔠_𝔰𝔡𝔰𝔭 => {
                                    let Ok(rs2) = (opcode as u32).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    let Ok(imm) = (compressed_instruction_step.0 as i32 + ((rd_bits as i32) << 6)).try_into() else { break 'ᵘⁿⁱᵐᵖˡᵉᵐᵉⁿᵗᵉᵈ };
                                    return self.sd((rs2, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾: <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝔵2.into(), 𝖽𝗂𝗌𝗉: imm}));
                                }]
                            }
                        }
                        self.unimplemented_16bit_instruction(parcel0)
                    }

                    𝔻𝕚𝕤𝕒𝕤𝕤𝕖𝕞𝕓𝕝𝕖𝕣𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟𝕤

                    $($𝓮𝔁𝓽𝓻𝓪_𝓭𝓮𝓯𝓲𝓷𝓮𝓼)*
                }
            }
        }
    };
    ($( $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        pub trait $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident {
           $($𝓮𝔁𝓽𝓻𝓪_𝓭𝓮𝓯𝓲𝓷𝓮𝓼:tt)*
        }
      )*) => {
        $(  𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖎𝖘𝖈𝖛_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖙𝖗𝖆𝖎𝖙! {
                $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
                pub trait $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮 {
                    $($𝓮𝔁𝓽𝓻𝓪_𝓭𝓮𝓯𝓲𝓷𝓮𝓼)*
                }
            }
         )*
    };
    (   $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        pub trait $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        $($𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼_𝓪𝓷𝓭_𝓭𝓮𝓯𝓲𝓷𝓮𝓼:tt)*
      ) => {
        #[allow(non_upper_case_globals)]
        pub trait $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
        $($𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼_𝓪𝓷𝓭_𝓭𝓮𝓯𝓲𝓷𝓮𝓼)*
    };
}
