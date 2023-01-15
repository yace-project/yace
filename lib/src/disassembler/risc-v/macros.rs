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
                pub trait $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
                where
                    super::super::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞:
                        𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐩𝐫𝐞𝐟𝐞𝐭𝐜𝐡_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        𝒛𝒆𝒓𝒐_𝒐𝒇𝒇𝒔𝒆𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜>,
                    u32:
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐚𝐛𝐢> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐚𝐛𝐢> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐚𝐛𝐢> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐫𝐨𝐮𝐧𝐝𝐢𝐧𝐠_𝐦𝐨𝐝𝐞> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐟𝐞𝐧𝐜𝐞> +
                     
                {
                    type 𝓒𝓟𝓤_𝓽𝔂𝓹𝓮: 𝑪𝑷𝑼;
                    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;
                    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;

                    fn instruction(&mut self, instruction: 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝, operands: &[𝐨𝐩𝐞𝐫𝐚𝐧𝐝<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>]) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;

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
