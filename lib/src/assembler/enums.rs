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

macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖞𝖋𝖗𝖔𝖒_𝖋𝖔𝖗_𝖎𝖓𝖙 {
        ($𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮:ident {$𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr} {$𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr} ($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮:ident, $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮:ident)) => {
            #[cfg(test)]
            impl $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> {
                fn test_i𝓫𝓪𝓼𝓮() {
                }
            }
        };
        ($𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮:ident {$𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr} {$𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr} {$𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴:expr} ($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮:ident, $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮:ident)) => {
            𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖞𝖋𝖗𝖔𝖒_𝖋𝖔𝖗_𝖎𝖓𝖙!($𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {$𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷} {$𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷} {$𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴} $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮);
        };
        ($𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮:ident {$𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr} {$𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr} {$𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴:expr} i8 $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮:ident, $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮:ident) => {
            impl TryFrom<i8> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: i8) -> Result<Self, Self::Error> {
                     if $𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴(value as $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮) {return Err($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))}
                     // SAFETY: we are using repr($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { core::mem::transmute::<$𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value)) })
                }
            }
            impl From<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> for i8 {
                #[allow(clippy::redundant_closure_call)]
                #[inline(always)]
                fn from(value: $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) -> i8 {
                    return ($𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as i8) as i8;
                }
            }
            impl TryFrom<u8> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: u8) -> Result<Self, Self::Error> {
                     if $𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴(value) {return Err($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))}
                     // SAFETY: we are using repr($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { core::mem::transmute::<$𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value)) })
                }
            }
            impl From<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> for u8 {
                #[allow(clippy::redundant_closure_call)]
                #[inline(always)]
                fn from(value: $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) -> u8 {
                    return ($𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as u8) as u8;
                }
            }
            𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖞𝖋𝖗𝖔𝖒_𝖋𝖔𝖗_𝖎𝖓𝖙!($𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {$𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷} {$𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷} {$𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴} i16 $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮);
        };
        ($𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮:ident {$𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr} {$𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr} {$𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴:expr} i16 $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮:ident, $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮:ident) => {
            impl TryFrom<i16> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: i16) -> Result<Self, Self::Error> {
                     if $𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴(value as u16) {return Err($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))}
                     // SAFETY: we are using repr($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { core::mem::transmute::<$𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮)) })
                }
            }
            impl From<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> for i16 {
                #[allow(clippy::redundant_closure_call)]
                #[inline(always)]
                fn from(value: $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) -> i16 {
                    return ($𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as i16) as i16;
                }
            }
            impl TryFrom<u16> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: u16) -> Result<Self, Self::Error> {
                     if $𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴(value) {return Err($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))}
                     // SAFETY: we are using repr($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { core::mem::transmute::<$𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮)) })
                }
            }
            impl From<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> for u16 {
                #[allow(clippy::redundant_closure_call)]
                #[inline(always)]
                fn from(value: $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) -> u16 {
                    return ($𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as u16) as u16;
                }
            }
            𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖞𝖋𝖗𝖔𝖒_𝖋𝖔𝖗_𝖎𝖓𝖙!($𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {$𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷} {$𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷} {$𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴} i32 $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮);
        };
        ($𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮:ident {$𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr} {$𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr} {$𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴:expr} i32 $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮:ident, $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮:ident) => {
            impl TryFrom<i32> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: i32) -> Result<Self, Self::Error> {
                     if $𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴(value as u32) {return Err($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))}
                     // SAFETY: we are using repr($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { core::mem::transmute::<$𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮)) })
                }
            }
            impl From<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> for i32 {
                #[allow(clippy::redundant_closure_call)]
                #[inline(always)]
                fn from(value: $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) -> i32 {
                    return ($𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as i32) as i32;
                }
            }
            impl TryFrom<u32> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: u32) -> Result<Self, Self::Error> {
                     if $𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴(value) {return Err($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))}
                     // SAFETY: we are using repr($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { core::mem::transmute::<$𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮)) })
                }
            }
            impl From<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> for u32 {
                #[allow(clippy::redundant_closure_call)]
                #[inline(always)]
                fn from(value: $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) -> u32 {
                    return ($𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as u32) as u32;
                }
            }

            impl TryFrom<isize> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: isize) -> Result<Self, Self::Error> {
                     if $𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴(value as usize) {return Err($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))}
                     // SAFETY: we are using repr($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { core::mem::transmute::<$𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮)) })
                }
            }
            impl From<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> for isize {
                #[allow(clippy::redundant_closure_call)]
                #[inline(always)]
                fn from(value: $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) -> isize {
                    return ($𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as isize) as isize;
                }
            }
            impl TryFrom<usize> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: usize) -> Result<Self, Self::Error> {
                     if $𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴(value) {return Err($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))}
                     // SAFETY: we are using repr($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { core::mem::transmute::<$𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮)) })
                }
            }
            impl From<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> for usize {
                #[allow(clippy::redundant_closure_call)]
                #[inline(always)]
                fn from(value: $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) -> usize {
                    return ($𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as usize) as usize;
                }
            }

            𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖞𝖋𝖗𝖔𝖒_𝖋𝖔𝖗_𝖎𝖓𝖙!($𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {$𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷} {$𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷} {$𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴} i64 $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮);
        };
        ($𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮:ident {$𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr} {$𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr} {$𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴:expr} i64 $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮:ident, $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮:ident) => {
            impl TryFrom<i64> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: i64) -> Result<Self, Self::Error> {
                     if $𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴(value as u64) {return Err($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))}
                     // SAFETY: we are using repr($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { core::mem::transmute::<$𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮)) })
                }
            }
            impl From<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> for i64 {
                #[allow(clippy::redundant_closure_call)]
                #[inline(always)]
                fn from(value: $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) -> i64 {
                    return ($𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as i64) as i64;
                }
            }
            impl TryFrom<u64> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: u64) -> Result<Self, Self::Error> {
                     if $𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴(value) {return Err($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))}
                     // SAFETY: we are using repr($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { core::mem::transmute::<$𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮)) })
                }
            }
            impl From<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> for u64 {
                #[allow(clippy::redundant_closure_call)]
                #[inline(always)]
                fn from(value: $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) -> u64 {
                    return ($𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as u64) as u64;
                }
            }
            𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖞𝖋𝖗𝖔𝖒_𝖋𝖔𝖗_𝖎𝖓𝖙!($𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {$𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷} {$𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷} {$𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴} i128 $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮);
        };
        ($𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮:ident {$𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr} {$𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr} {$𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴:expr} i128 $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮:ident, $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮:ident) => {
            impl TryFrom<i128> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: i128) -> Result<Self, Self::Error> {
                     if $𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴(value as u128) {return Err($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))}
                     // SAFETY: we are using repr($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { core::mem::transmute::<$𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮)) })
                }
            }
            impl From<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> for i128 {
                #[allow(clippy::redundant_closure_call)]
                #[inline(always)]
                fn from(value: $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) -> i128 {
                    return ($𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as i128) as i128;
                }
            }
            impl TryFrom<u128> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: u128) -> Result<Self, Self::Error> {
                     if $𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴(value) {return Err($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))}
                     // SAFETY: we are using repr($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { core::mem::transmute::<$𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮)) })
                }
            }
            impl From<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> for u128 {
                #[allow(clippy::redundant_closure_call)]
                #[inline(always)]
                fn from(value: $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) -> u128 {
                    return ($𝓽𝔂𝓹𝓮_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as u128) as u128;
                }
            }

            impl<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮> TryFrom<core::num::Wrapping<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 where $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮: TryFrom<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮> {
                type Error = <$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 as TryFrom<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>>::Error;
                #[inline(always)]
                fn try_from(value: core::num::Wrapping<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>) -> Result<Self, Self::Error> {
                    Self::try_from(value.0)
                }
            }
            impl<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮> From<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> for core::num::Wrapping<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮> where 𝓲𝓷𝓽_𝓽𝔂𝓹𝓮: From<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> {
                #[inline(always)]
                fn from(value: $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) -> core::num::Wrapping<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮> {
                    core::num::Wrapping(core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(value))
                }
            }


            #[cfg(test)]
            impl $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> {
                #[allow(dead_code)]
                fn test_i𝓫𝓪𝓼𝓮() {
                    for value in $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮::MIN..=$𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮::MAX {
                      assert_eq!(core::convert::TryInto::<$crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕::<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>>::try_into(value)
                                     .ok()
                                     .map(|value| value.0),
                                 core::convert::TryInto::<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>::try_into(value).ok())
                    }
                }
            }
        }
    }

macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖊𝖓𝖚𝖒𝖘 {
        ($(
            [$({$𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴:expr})?]
            [$($𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮:ident $({$𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr})?),*]
            [$($𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮:ident {$𝓮𝓷𝓾𝓶_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴:expr} $({$𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr})?),*]
            #[derive($($𝓭𝓮𝓻𝓲𝓿𝓮𝓼:tt)*)]
            #[repr($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮:ident/$𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮:ident)]
            pub enum $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮:ident {
                $($𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮:ident = $𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓮𝓷𝓬𝓸𝓭𝓲𝓷𝓰:expr),*
            }
          )*) => {
            $(
                #[derive($($𝓭𝓮𝓻𝓲𝓿𝓮𝓼)*)]
                #[repr($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮)]
                pub enum $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮 {
                   $(
                       $𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮 = $𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓮𝓷𝓬𝓸𝓭𝓲𝓷𝓰,
                    )*
                }

                #[cfg(test)]
                impl TryFrom<$𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮> for $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮> {
                    type Error = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
                    fn try_from(value: $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮) -> Result<Self, Self::Error> {
                        match 𝗲𝗻𝘂𝗺_𝘁𝗲𝘀𝘁𝘀::adjust_int_value(value, stringify!($𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮)) {
                            $(
                                value if value == $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮::$𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮 as $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 => 
                                    Ok($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕($𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮::$𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮)),
                             )*
                            _ => Err($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
                        }
                    }
                }

                #[cfg(test)]
                impl $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮> {
                    #[allow(dead_code)]
                    fn all_from(value: $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮) -> Result<Self, $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
                        match value {
                            $(
                            value if value == $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮::$𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮 as $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 =>
                                    Ok($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕($𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮::$𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮)),
                             )*
                            _ => Err($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
                        }
                    }
                }

                𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖞𝖋𝖗𝖔𝖒_𝖋𝖔𝖗_𝖎𝖓𝖙!($𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮 {core::convert::identity} {core::convert::identity} $({$𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴})* ($𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, $𝓾𝓷𝓼𝓲𝓰𝓷𝓮𝓭_𝓽𝔂𝓹𝓮));

                $(
                    impl From<$𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮> for $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮 {
                        #[inline(always)]
                        fn from(value: $𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮) -> Self {
                            // SAFETY: we are using repr(i8) here thus conversion is safe.
                            unsafe { core::mem::transmute::<$𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>($($𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)*(value as $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮)) }
                        }
                    }
                    impl From<$𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮> for Option<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮> {
                        #[inline(always)]
                        fn from(value: $𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮) -> Self {
                            // SAFETY: we are using repr(i8) here thus conversion is safe.
                            Some(unsafe { core::mem::transmute::<$𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>($($𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)*(value as $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮)) })
                        }
                    }
                    #[cfg(test)]
                    impl From<$crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>>
                    for $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮> {
                        fn from(value: $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>) -> Self {
                            // Certain enum values should be converted non-trivially.
                            // E.g. both 𝔟𝔥 is -1 in 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 but 7 in 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086.
                            // Since this only needed that for tests we don't worry about efficiency,
                            // and it's easier to reason about when all checks are in one place,
                            // in the adjust_safe_values function below.
                            Self::all_from(𝗲𝗻𝘂𝗺_𝘁𝗲𝘀𝘁𝘀::adjust_safe_values(
                                value.0 as $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, stringify!($𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮), stringify!($𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮))).unwrap()
                        }
                    }
                 )*

                $(
                    impl TryFrom<$𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮> for $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮 {
                        // The only possible error here can be is “register doesn't belong to specific register class”.
                        //
                        // Returning Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐫𝐫𝐨𝐫(())) is enough to pass that infomation but makes Result smaller
                        // (although in real code it's almost always consumed with ok() thus we may pass some more info, but then
                        // if it's always consumed by ok() then what's the point of passing more into?).
                        type Error = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐫𝐫𝐨𝐫;

                        #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                        #[inline(always)]
                        fn try_from(value: $𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮) -> Result<Self, Self::Error> {
                            if $𝓮𝓷𝓾𝓶_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴(value as $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮) {return Err($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐫𝐫𝐨𝐫(()))}
                            // SAFETY: we are using repr(i8) here thus conversion is safe and it doesn't disable any optimizations:
                            //   Result<Self, Self::Error> is still one byte in size.
                            Ok(unsafe { core::mem::transmute::<$𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>($($𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)*(value as $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮)) })
                        }
                    }
                    #[cfg(test)]
                    impl TryFrom<$crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>>
                    for $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮> {
                        // The only possible error here can be is “register doesn't belong to specific register class”.
                        //
                        // Returning Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐫𝐫𝐨𝐫(())) is enough to pass that infomation but makes Result smaller
                        // (although in real code it's almost always consumed with ok() thus we may pass some more info, but then
                        // if it's always consumed by ok() then what's the point of passing more into?).
                        type Error = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;

                        fn try_from(value: $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>) -> Result<Self, Self::Error> {
                            // Certain enum values shouldn't be converted even if values match.
                            // E.g. both 𝔟𝔥 and 𝔢𝔦𝔷 have value -1, but they shouldn't be converted.
                            // Since this only needed that for tests we don't worry about efficiency,
                            // and it's easier to reason about when all checks are in one place,
                            // in the adjust_unsafe_values function below.
                            Self::all_from(𝗲𝗻𝘂𝗺_𝘁𝗲𝘀𝘁𝘀::adjust_unsafe_values(
                                value.0 as $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, stringify!($𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮), stringify!($𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮)))
                        }
                    }
                 )*

                #[cfg(test)]
                impl $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮> {
                    fn test_safe() {
                        $(
                            for value in $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮::MIN..=$𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮::MAX {
                                if let Ok(value) = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕::<$𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>::all_from(value) {
                                    assert_eq!($crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕::<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>::from(value).0,
                                               $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮::from(value.0))
                                }
                            }
                         )*
                    }
                }

                #[cfg(test)]
                impl $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮> {
                    fn test_unsafe() {
                        $(
                            // Count number of successfully converted enum values.  Providing conversions which may never succeed
                            // is not beneficial: it just shifts detection of problems from compile-time to runtime.
                            let mut successfully_converted = 0;
                            for value in $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮::MIN..=$𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮::MAX {
                                if let Ok(value) = $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕::<$𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>::all_from(value) {
                                    let converted_safely = value
                                        .try_into()
                                        .ok()
                                        .map(|value: $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>| value.0);
                                    let converted_unsafely = value.0.try_into().ok();
                                    assert_eq!(converted_safely, converted_unsafely);
                                    if converted_safely.is_some() {
                                        successfully_converted += 1
                                    }
                                }
                            }
                            assert!(successfully_converted > 0);
                         )*
                    }
                }
             )*

            #[cfg(test)]
            mod 𝗲𝗻𝘂𝗺_𝘁𝗲𝘀𝘁𝘀 {
                use super::*;

                pub(super) fn adjust_int_value<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮: From<i8> + core::cmp::Eq>(
                    value: 𝓲𝓷𝓽_𝓽𝔂𝓹𝓮, target_enum_name: &str) -> 𝓲𝓷𝓽_𝓽𝔂𝓹𝓮
                {
                    if value == core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(4) &&
                       (target_enum_name.starts_with("𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫") ||
                        target_enum_name.starts_with("𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫")){
                        i8::MIN.into()
                    } else {
                        value
                    }
                }

                pub(super) fn adjust_safe_values<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮: From<i8> +
                                                 core::cmp::Eq +
                                                 core::cmp::Ord +
                                                 core::ops::Add<Output = 𝓲𝓷𝓽_𝓽𝔂𝓹𝓮> +
                                                 core::ops::Sub<Output = 𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>>(
                    value: 𝓲𝓷𝓽_𝓽𝔂𝓹𝓮, target_enum_name: &str, source_enum_name: &str) -> 𝓲𝓷𝓽_𝓽𝔂𝓹𝓮
                {
                    if value > core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(3) &&
                       target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64") &&
                       (source_enum_name.eq("𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                        source_enum_name.eq("𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                        source_enum_name.eq("𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                        source_enum_name.eq("𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                        source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                        source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086")) {
                        value - 8.into()
                    } else if value > core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(3) &&
                              !target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") &&
                              !target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086") &&
                              (source_enum_name.eq("𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                               source_enum_name.eq("𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                               source_enum_name.eq("𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                               source_enum_name.eq("𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                               source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ")) {
                        value - 4.into()
                    } else if value <= core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(3) &&
                              (target_enum_name.eq("𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                               target_enum_name.eq("𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                               target_enum_name.eq("𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                               target_enum_name.eq("𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                               target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ")) &&
                               !source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") &&
                               !source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086") {
                        value + 4.into()
                    } else {
                        value
                    }
                }

                pub(super) fn adjust_unsafe_values<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮: From<i8> +
                                                   core::cmp::Eq +
                                                   core::cmp::Ord +
                                                   core::ops::Add<Output = 𝓲𝓷𝓽_𝓽𝔂𝓹𝓮> +
                                                   core::ops::Sub<Output = 𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>>(
                    value: 𝓲𝓷𝓽_𝓽𝔂𝓹𝓮, target_enum_name: &str, source_enum_name: &str) -> 𝓲𝓷𝓽_𝓽𝔂𝓹𝓮
                {
                    // First handle special conversions where 8ᵇⁱᵗ registers are involved and
                    // conversions are changing values.
                    if (value == core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(0) &&
                        (target_enum_name.eq("𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                         target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ")) &&
                        !source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086") &&
                        !source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64")) ||
                       (value == core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(4) &&
                        (target_enum_name.eq("𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                         target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ")) &&
                        source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086")) ||
                       (value == core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(-4) &&
                        (target_enum_name.eq("𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                         target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ")) &&
                        source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64")) {
                        4.into()
                    } else if (value == core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(1) &&
                               (target_enum_name.eq("𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                                target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ")) &&
                               !source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086") &&
                               !source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64")) ||
                              (value == core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(5) &&
                               (target_enum_name.eq("𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                                target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ")) &&
                               source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086")) ||
                              (value == core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(-3) &&
                               (target_enum_name.eq("𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                                target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ")) &&
                               source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64")) {
                        5.into()
                    } else if (value == core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(2) &&
                               (target_enum_name.eq("𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                                target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ")) &&
                               !source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086") &&
                               !source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64")) ||
                              (value == core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(6) &&
                               (target_enum_name.eq("𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                                target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ")) &&
                               source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086")) ||
                              (value == core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(-2) &&
                               (target_enum_name.eq("𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                                target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ")) &&
                               source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64")) {
                        6.into()
                    } else if (value == core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(3) &&
                               (target_enum_name.eq("𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                                target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ")) &&
                               !source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086") &&
                               !source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64")) ||
                              (value == core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(7) &&
                               (target_enum_name.eq("𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                                target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ")) &&
                               source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086")) ||
                              (value == core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(-1) &&
                               (target_enum_name.eq("𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                                target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ")) &&
                               source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64")) {
                        7.into()
                        // Now conversions of 8ᵇⁱᵗ registers where keeping values are incorrect.
                    } else if value > core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(3) &&
                              (target_enum_name.eq("𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                               target_enum_name.eq("𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                               target_enum_name.eq("𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                               target_enum_name.eq("𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                               target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") ||
                               target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086")) &&
                              !source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086") &&
                              !source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") {
                        i8::MIN.into()
                    } else if value > core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(3) &&
                              !target_enum_name.eq("𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") &&
                              !target_enum_name.eq("𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") &&
                              !target_enum_name.eq("𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") &&
                              !target_enum_name.eq("𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") &&
                              !target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") &&
                              !target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086") &&
                               source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086") {
                        i8::MIN.into()
                    } else if value > core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(3).into() &&
                              !target_enum_name.eq("𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") &&
                              !target_enum_name.eq("𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") &&
                              !target_enum_name.eq("𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") &&
                              !target_enum_name.eq("𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") &&
                              !target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086") &&
                               source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ") {
                        value - 4.into()
                    } else if value < core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(0) &&
                              target_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086") &&
                              source_enum_name.eq("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64") {
                        value + 8.into()
                    // And conversions between indexes and 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64.
                    } else if value == core::convert::Into::<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>::into(4) &&
                       (((target_enum_name.starts_with("𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫") ||
                          target_enum_name.starts_with("𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫")) &&
                         source_enum_name.starts_with("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫")) ||
                        (target_enum_name.starts_with("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫") &&
                         (source_enum_name.starts_with("𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫") ||
                          source_enum_name.starts_with("𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫")))) {
                        i8::MIN.into()
                    } else {
                        value
                    }
                }

                #[test]
                fn test_i𝓫𝓪𝓼𝓮() {
                    $(
                        $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕::<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>::test_i𝓫𝓪𝓼𝓮();
                     )*
                }

                #[test]
                fn test_safe() {
                    $(
                        $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕::<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>::test_safe();
                     )*
                }

                #[test]
                fn test_unsafe() {
                    $(
                        $crate::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕::<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>::test_unsafe();
                     )*
                }
            }
        };
    }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(pub(crate) ());

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐫𝐫𝐨𝐫(pub(crate) ());

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub(crate) struct 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>(pub(crate) 𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮);
