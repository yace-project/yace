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

#[cfg(feature = "std")]
pub trait 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 {
    fn emit_u8(&mut self, value: u8) -> Result<(), Box<dyn std::error::Error>>;
    #[inline(always)]
    fn emit_i8(&mut self, value: i8) -> Result<(), Box<dyn std::error::Error>> {
        self.emit_u8(value as u8)
    }
    #[inline(always)]
    fn emit_u16(&mut self, value: u16) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {   self.emit_u8((value >> 8) as u8)?;
            Ok(self.emit_u8(value as u8)?)}
        #[cfg(target_endian = "little")]
        {   self.emit_u8(value as u8)?;
            Ok(self.emit_u8((value >> 8) as u8)?)}
    }
    #[inline(always)]
    fn emit_i16(&mut self, value: i16) -> Result<(), Box<dyn std::error::Error>> {
        self.emit_u16(value as u16)
    }
    #[inline(always)]
    fn emit_u32(&mut self, value: u32) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {   self.emit_u16((value >> 16) as u16)?;
            Ok(self.emit_u16(value as u16)?)}
        #[cfg(target_endian = "little")]
        {   self.emit_u16(value as u16)?;
            Ok(self.emit_u16((value >> 16) as u16)?)}
    }
    #[inline(always)]
    fn emit_i32(&mut self, value: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.emit_u32(value as u32)
    }
    #[inline(always)]
    fn emit_u64(&mut self, value: u64) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {   self.emit_u32((value >> 32) as u32)?;
            Ok(self.emit_u32(value as u32)?)}
        #[cfg(target_endian = "little")]
        {   self.emit_u32(value as u32)?;
            Ok(self.emit_u32((value >> 32) as u32)?)}
    }
    #[inline(always)]
    fn emit_i64(&mut self, value: i64) -> Result<(), Box<dyn std::error::Error>> {
        self.emit_u64(value as u64)
    }
    #[inline(always)]
    fn emit_u128(&mut self, value: u128) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {   self.emit_u64((value >> 64) as u64)?;
            Ok(self.emit_u64(value as u64)?)}
        #[cfg(target_endian = "little")]
        {   self.emit_u64(value as u64)?;
            Ok(self.emit_u64((value >> 64) as u64)?)}
    }
    #[inline(always)]
    fn emit_i128(&mut self, value: i128) -> Result<(), Box<dyn std::error::Error>> {
        self.emit_u128(value as u128)
    }
    #[inline(always)]
    fn emit_u8_slice(&mut self, 𝗌𝗅𝗂𝖼𝖾: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let mut index = 0;
        while index + 16 <= 𝗌𝗅𝗂𝖼𝖾.len() {
            let mut 𝖺𝗋𝗋𝖺𝗒 = [0u8; 16];
            𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝗌𝗅𝗂𝖼𝖾[index..index+16]);
            self.emit_u128(u128::from_ne_bytes(𝖺𝗋𝗋𝖺𝗒))?;
            index += 16;
        }
        if index + 8 <= 𝗌𝗅𝗂𝖼𝖾.len() {
            let mut 𝖺𝗋𝗋𝖺𝗒 = [0u8; 8];
            𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝗌𝗅𝗂𝖼𝖾[index..index+8]);
            self.emit_u64(u64::from_ne_bytes(𝖺𝗋𝗋𝖺𝗒))?;
            index += 8;
        }
        if index + 4 <= 𝗌𝗅𝗂𝖼𝖾.len() {
            let mut 𝖺𝗋𝗋𝖺𝗒 = [0u8; 4];
            𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝗌𝗅𝗂𝖼𝖾[index..index+4]);
            self.emit_u32(u32::from_ne_bytes(𝖺𝗋𝗋𝖺𝗒))?;
            index += 4;
        }
        if index + 2 <= 𝗌𝗅𝗂𝖼𝖾.len() {
            let mut 𝖺𝗋𝗋𝖺𝗒 = [0u8; 2];
            𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝗌𝗅𝗂𝖼𝖾[index..index+2]);
            self.emit_u16(u16::from_ne_bytes(𝖺𝗋𝗋𝖺𝗒))?;
            index += 2;
        }
        if index + 1 <= 𝗌𝗅𝗂𝖼𝖾.len() {
            let mut 𝖺𝗋𝗋𝖺𝗒 = [0u8; 1];
            𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝗌𝗅𝗂𝖼𝖾[index..index+1]);
            self.emit_u8(u8::from_ne_bytes(𝖺𝗋𝗋𝖺𝗒))?;
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ> 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for dyn 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 + 'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ
where
    Self: 𝒆𝒎𝒊𝒕𝒕𝒆𝒓<𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = (), 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = Box<dyn std::error::Error>>
{
    fn emit_u8(&mut self, value: u8) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_u8(self, value)
    }
    #[inline(always)]
    fn emit_i8(&mut self, value: i8) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_i8(self, value)
    }
    #[inline(always)]
    fn emit_bytes_u16(&mut self, value: u16) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_u16(self, value)
    }
    #[inline(always)]
    fn emit_bytes_i16(&mut self, value: i16) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_i16(self, value)
    }
    #[inline(always)]
    fn emit_bytes_u32(&mut self, value: u32) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_u32(self, value)
    }
    #[inline(always)]
    fn emit_bytes_i32(&mut self, value: i32) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_i32(self, value)
    }
    #[inline(always)]
    fn emit_bytes_u64(&mut self, value: u64) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_u64(self, value)
    }
    #[inline(always)]
    fn emit_bytes_i64(&mut self, value: i64) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_i64(self, value)
    }
    #[inline(always)]
    fn emit_bytes_u128(&mut self, value: u128) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_u128(self, value)
    }
    #[inline(always)]
    fn emit_bytes_i128(&mut self, value: i128) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_i128(self, value)
    }
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    fn emit_u8_array<const 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮: usize>(&mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮])
        -> Result<(), Box<dyn std::error::Error>>
    where Self: 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, { 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮 }> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_u8_slice(self, &𝖺𝗋𝗋𝖺𝗒[..])
    }
    #[inline(always)]
    fn emit_u8_slice(&mut self, 𝗌𝗅𝗂𝖼𝖾: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_u8_slice(self, 𝗌𝗅𝗂𝖼𝖾)
    }
}

#[cfg(feature = "std")]
impl<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ> 𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for dyn 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 + 'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ {
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = Box<dyn std::error::Error>;
    fn combine_results(_: &mut (), _: ()) {
    }
}

#[cfg(feature = "std")]
pub trait 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 {
    fn emit_u16(&mut self, value: u16) -> Result<(), Box<dyn std::error::Error>>;
    #[inline(always)]
    fn emit_i16(&mut self, value: i16) -> Result<(), Box<dyn std::error::Error>> {
        self.emit_u16(value as u16)
    }
    #[inline(always)]
    fn emit_u32(&mut self, value: u32) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {   self.emit_u16((value >> 16) as u16)?;
            Ok(self.emit_u16(value as u16)?)}
        #[cfg(target_endian = "little")]
        {   self.emit_u16(value as u16)?;
            Ok(self.emit_u16((value >> 16) as u16)?)}
    }
    #[inline(always)]
    fn emit_i32(&mut self, value: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.emit_u32(value as u32)
    }
    #[inline(always)]
    fn emit_u64(&mut self, value: u64) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {   self.emit_u32((value >> 32) as u32)?;
            Ok(self.emit_u32(value as u32)?)}
        #[cfg(target_endian = "little")]
        {   self.emit_u32(value as u32)?;
            Ok(self.emit_u32((value >> 32) as u32)?)}
    }
    #[inline(always)]
    fn emit_i64(&mut self, value: i64) -> Result<(), Box<dyn std::error::Error>> {
        self.emit_u64(value as u64)
    }
    #[inline(always)]
    fn emit_u128(&mut self, value: u128) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {   self.emit_u64((value >> 64) as u64)?;
            Ok(self.emit_u64(value as u64)?)}
        #[cfg(target_endian = "little")]
        {   self.emit_u64(value as u64)?;
            Ok(self.emit_u64((value >> 64) as u64)?)}
    }
    #[inline(always)]
    fn emit_i128(&mut self, value: i128) -> Result<(), Box<dyn std::error::Error>> {
        self.emit_u128(value as u128)
    }
}

#[cfg(feature = "std")]
impl<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ> 𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for dyn 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 + 'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ {
    #[inline(always)]
    fn emit_u16(&mut self, value: u16) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_u16(self, value)
    }
    #[inline(always)]
    fn emit_i16(&mut self, value: i16) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_i16(self, value)
    }
    #[inline(always)]
    fn emit_parcels_u32(&mut self, value: u32) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_u32(self, value)
    }
    #[inline(always)]
    fn emit_parcels_i32(&mut self, value: i32) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_i32(self, value)
    }
    #[inline(always)]
    fn emit_parcels_u64(&mut self, value: u64) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_u64(self, value)
    }
    #[inline(always)]
    fn emit_parcels_i64(&mut self, value: i64) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_i64(self, value)
    }
    #[inline(always)]
    fn emit_parcels_u128(&mut self, value: u128) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_u128(self, value)
    }
    #[inline(always)]
    fn emit_parcels_i128(&mut self, value: i128) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_i128(self, value)
    }
}

#[cfg(feature = "std")]
impl<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ> 𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for dyn 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 + 'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ {
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = Box<dyn std::error::Error>;
    fn combine_results(_: &mut (), _: ()) {
    }
}

#[cfg(feature = "std")]
pub trait 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 {
    fn emit_u32(&mut self, value: u32) -> Result<(), Box<dyn std::error::Error>>;
    #[inline(always)]
    fn emit_i32(&mut self, value: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.emit_u32(value as u32)
    }
    #[inline(always)]
    fn emit_u64(&mut self, value: u64) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {   self.emit_u32((value >> 32) as u32)?;
            Ok(self.emit_u32(value as u32)?)}
        #[cfg(target_endian = "little")]
        {   self.emit_u32(value as u32)?;
            Ok(self.emit_u32((value >> 32) as u32)?)}
    }
    #[inline(always)]
    fn emit_i64(&mut self, value: i64) -> Result<(), Box<dyn std::error::Error>> {
        self.emit_u64(value as u64)
    }
    #[inline(always)]
    fn emit_u128(&mut self, value: u128) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {   self.emit_u64((value >> 64) as u64)?;
            Ok(self.emit_u64(value as u64)?)}
        #[cfg(target_endian = "little")]
        {   self.emit_u64(value as u64)?;
            Ok(self.emit_u64((value >> 64) as u64)?)}
    }
    #[inline(always)]
    fn emit_i128(&mut self, value: i128) -> Result<(), Box<dyn std::error::Error>> {
        self.emit_u128(value as u128)
    }
}

#[cfg(feature = "std")]
impl<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ> 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for dyn 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 + 'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ {
    #[inline(always)]
    fn emit_u32(&mut self, value: u32) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_u32(self, value)
    }
    #[inline(always)]
    fn emit_i32(&mut self, value: i32) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_i32(self, value)
    }
    #[inline(always)]
    fn emit_u64(&mut self, value: u64) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_u64(self, value)
    }
    #[inline(always)]
    fn emit_i64(&mut self, value: i64) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_i64(self, value)
    }
    #[inline(always)]
    fn emit_u128(&mut self, value: u128) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_u128(self, value)
    }
    #[inline(always)]
    fn emit_i128(&mut self, value: i128) -> Result<(), Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::emit_i128(self, value)
    }
}

#[cfg(feature = "std")]
impl<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ> 𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for dyn 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 + 'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ {
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = Box<dyn std::error::Error>;
    fn combine_results(_: &mut (), _: ()) {
    }
}

pub trait 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓: 𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 {
    fn emit_u8(&mut self, value: u8) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
    #[inline(always)]
    fn emit_i8(&mut self, value: i8) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_u8(value as u8)
    }
    #[inline(always)]
    fn emit_bytes_u16(&mut self, value: u16) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {   let mut result = self.emit_u8((value >> 8) as u8)?;
            Self::combine_results(&mut result, self.emit_u8(value as u8)?);
            Ok(result)}
        #[cfg(target_endian = "little")]
        {   let mut result = self.emit_u8(value as u8)?;
            Self::combine_results(&mut result, self.emit_u8((value >> 8) as u8)?);
            Ok(result)}
    }
    #[inline(always)]
    fn emit_bytes_i16(&mut self, value: i16) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_bytes_u16(value as u16)
    }
    #[inline(always)]
    fn emit_bytes_u32(&mut self, value: u32) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {   let mut result = self.emit_bytes_u16((value >> 16) as u16)?;
            Self::combine_results(&mut result, self.emit_bytes_u16(value as u16)?);
            Ok(result)}
        #[cfg(target_endian = "little")]
        {   let mut result = self.emit_bytes_u16(value as u16)?;
            Self::combine_results(&mut result, self.emit_bytes_u16((value >> 16) as u16)?);
            Ok(result)}
    }
    #[inline(always)]
    fn emit_bytes_i32(&mut self, value: i32) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_bytes_u32(value as u32)
    }
    #[inline(always)]
    fn emit_bytes_u64(&mut self, value: u64) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {   let mut result = self.emit_bytes_u32((value >> 32) as u32)?;
            Self::combine_results(&mut result, self.emit_bytes_u32(value as u32)?);
            Ok(result)}
        #[cfg(target_endian = "little")]
        {   let mut result = self.emit_bytes_u32(value as u32)?;
            Self::combine_results(&mut result, self.emit_bytes_u32((value >> 32) as u32)?);
            Ok(result)}
    }
    #[inline(always)]
    fn emit_bytes_i64(&mut self, value: i64) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_bytes_u64(value as u64)
    }
    #[inline(always)]
    fn emit_bytes_u128(&mut self, value: u128) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {   let mut result = self.emit_bytes_u64((value >> 64) as u64)?;
            Self::combine_results(&mut result, self.emit_bytes_u64(value as u64)?);
            Ok(result)}
        #[cfg(target_endian = "little")]
        {   let mut result = self.emit_bytes_u64(value as u64)?;
            Self::combine_results(&mut result, self.emit_bytes_u64((value >> 64) as u64)?);
            Ok(result)}
    }
    #[inline(always)]
    fn emit_bytes_i128(&mut self, value: i128) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_bytes_u128(value as u128)
    }
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    fn emit_u8_array<const 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮: usize>(&mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮])
        -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
    where Self: 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, { 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮 }> {
        <Self as 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, { 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮 }>>::emit_array(self, 𝖺𝗋𝗋𝖺𝗒)
    }
    #[inline(always)]
    fn emit_u8_slice(&mut self, 𝗌𝗅𝗂𝖼𝖾: &[u8]) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut index = 0;
        let mut result = Default::default();
        while index + 16 <= 𝗌𝗅𝗂𝖼𝖾.len() {
            let mut 𝖺𝗋𝗋𝖺𝗒 = [0u8; 16];
            𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝗌𝗅𝗂𝖼𝖾[index..index+16]);
            Self::combine_results(&mut result, self.emit_bytes_u128(u128::from_ne_bytes(𝖺𝗋𝗋𝖺𝗒))?);
            index += 16;
        }
        if index + 8 <= 𝗌𝗅𝗂𝖼𝖾.len() {
            let mut 𝖺𝗋𝗋𝖺𝗒 = [0u8; 8];
            𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝗌𝗅𝗂𝖼𝖾[index..index+8]);
            Self::combine_results(&mut result, self.emit_bytes_u64(u64::from_ne_bytes(𝖺𝗋𝗋𝖺𝗒))?);
            index += 8;
        }
        if index + 4 <= 𝗌𝗅𝗂𝖼𝖾.len() {
            let mut 𝖺𝗋𝗋𝖺𝗒 = [0u8; 4];
            𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝗌𝗅𝗂𝖼𝖾[index..index+4]);
            Self::combine_results(&mut result, self.emit_bytes_u32(u32::from_ne_bytes(𝖺𝗋𝗋𝖺𝗒))?);
            index += 4;
        }
        if index + 2 <= 𝗌𝗅𝗂𝖼𝖾.len() {
            let mut 𝖺𝗋𝗋𝖺𝗒 = [0u8; 2];
            𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝗌𝗅𝗂𝖼𝖾[index..index+2]);
            Self::combine_results(&mut result, self.emit_bytes_u16(u16::from_ne_bytes(𝖺𝗋𝗋𝖺𝗒))?);
            index += 2;
        }
        if index + 1 <= 𝗌𝗅𝗂𝖼𝖾.len() {
            Self::combine_results(&mut result, self.emit_u8(𝗌𝗅𝗂𝖼𝖾[index])?);
        }
        Ok(result)
    }
}

pub trait 𝒓𝒆𝒃𝒊𝒏𝒅_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 {
    type 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻<𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>;
    fn rebind<𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>(
        new_emitter: 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮
    ) -> Self::𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻<𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>;
}

impl<𝓔𝓶𝓲𝓽𝓽𝓮𝓻: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 + ?Sized> 𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝓔𝓶𝓲𝓽𝓽𝓮𝓻 {
    #[inline(always)]
    fn emit_u16(&mut self, value: u16) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_bytes_u16(value)
    }
    #[inline(always)]
    fn emit_i16(&mut self, value: i16) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_bytes_i16(value)
    }
    #[inline(always)]
    fn emit_parcels_u32(&mut self, value: u32) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_bytes_u32(value)
    }
    #[inline(always)]
    fn emit_parcels_i32(&mut self, value: i32) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_bytes_i32(value)
    }
    #[inline(always)]
    fn emit_parcels_u64(&mut self, value: u64) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_bytes_u64(value)
    }
    #[inline(always)]
    fn emit_parcels_i64(&mut self, value: i64) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_bytes_i64(value)
    }
    #[inline(always)]
    fn emit_parcels_u128(&mut self, value: u128) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_bytes_u128(value)
    }
    #[inline(always)]
    fn emit_parcels_i128(&mut self, value: i128) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_bytes_i128(value)
    }
}

pub trait 𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓: 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 {
    fn emit_u16(&mut self, value: u16) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
    #[inline(always)]
    fn emit_i16(&mut self, value: i16) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_u16(value as u16)
    }
    #[inline(always)]
    fn emit_parcels_u32(&mut self, value: u32) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {   let mut result = self.emit_u16((value >> 16) as u16)?;
            Self::combine_results(&mut result, self.emit_u16(value as u16)?);
            Ok(result)}
        #[cfg(target_endian = "little")]
        {   let mut result = self.emit_u16(value as u16)?;
            Self::combine_results(&mut result, self.emit_u16((value >> 16) as u16)?);
            Ok(result)}
    }
    #[inline(always)]
    fn emit_parcels_i32(&mut self, value: i32) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_parcels_u32(value as u32)
    }
    #[inline(always)]
    fn emit_parcels_u64(&mut self, value: u64) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {   let mut result = self.emit_parcels_u32((value >> 32) as u32)?;
            Self::combine_results(&mut result, self.emit_parcels_u32(value as u32)?);
            Ok(result)}
        #[cfg(target_endian = "little")]
        {   let mut result = self.emit_parcels_u32(value as u32)?;
            Self::combine_results(&mut result, self.emit_parcels_u32((value >> 32) as u32)?);
            Ok(result)}
    }
    #[inline(always)]
    fn emit_parcels_i64(&mut self, value: i64) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_parcels_u64(value as u64)
    }
    #[inline(always)]
    fn emit_parcels_u128(&mut self, value: u128) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {   let mut result = self.emit_parcels_u64((value >> 64) as u64)?;
            Self::combine_results(&mut result, self.emit_parcels_u64(value as u64)?);
            Ok(result)}
        #[cfg(target_endian = "little")]
        {   let mut result = self.emit_parcels_u64(value as u64)?;
            Self::combine_results(&mut result, self.emit_parcels_u64((value >> 64) as u64)?);
            Ok(result)}
    }
    #[inline(always)]
    fn emit_parcels_i128(&mut self, value: i128) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_parcels_u128(value as u128)
    }
}

pub trait 𝒓𝒆𝒃𝒊𝒏𝒅_𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 {
    type 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻<𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>;
    fn rebind<𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>(
        new_emitter: 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮
    ) -> Self::𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻<𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>;
}

impl<𝓔𝓶𝓲𝓽𝓽𝓮𝓻: 𝒑𝒂𝒓𝒄𝒆𝒍_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 + ?Sized> 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝓔𝓶𝓲𝓽𝓽𝓮𝓻 {
    #[inline(always)]
    fn emit_u32(&mut self, value: u32) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_parcels_u32(value)
    }
    #[inline(always)]
    fn emit_i32(&mut self, value: i32) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_parcels_i32(value)
    }
    #[inline(always)]
    fn emit_u64(&mut self, value: u64) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_parcels_u64(value)
    }
    #[inline(always)]
    fn emit_i64(&mut self, value: i64) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_parcels_i64(value)
    }
    #[inline(always)]
    fn emit_u128(&mut self, value: u128) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_parcels_u128(value)
    }
    #[inline(always)]
    fn emit_i128(&mut self, value: i128) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_parcels_i128(value)
    }
}

pub trait 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒆𝒎𝒊𝒕𝒕𝒆𝒓: 𝒆𝒎𝒊𝒕𝒕𝒆𝒓 {
    fn emit_u32(&mut self, value: u32) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
    #[inline(always)]
    fn emit_i32(&mut self, value: i32) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_u32(value as u32)
    }
    #[inline(always)]
    fn emit_u64(&mut self, value: u64) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {   let mut result = self.emit_u32((value >> 32) as u32)?;
            Self::combine_results(&mut result, self.emit_u32(value as u32)?);
            Ok(result)}
        #[cfg(target_endian = "little")]
        {   let mut result = self.emit_u32(value as u32)?;
            Self::combine_results(&mut result, self.emit_u32((value >> 32) as u32)?);
            Ok(result)}
    }
    #[inline(always)]
    fn emit_i64(&mut self, value: i64) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_u64(value as u64)
    }
    #[inline(always)]
    fn emit_u128(&mut self, value: u128) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {   let mut result = self.emit_u64((value >> 64) as u64)?;
            Self::combine_results(&mut result, self.emit_u64(value as u64)?);
            Ok(result)}
        #[cfg(target_endian = "little")]
        {   let mut result = self.emit_u64(value as u64)?;
            Self::combine_results(&mut result, self.emit_u64((value >> 64) as u64)?);
            Ok(result)}
    }
    #[inline(always)]
    fn emit_i128(&mut self, value: i128) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_u128(value as u128)
    }
}

pub trait 𝒓𝒆𝒃𝒊𝒏𝒅_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 {
    type 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻<𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>;
    fn rebind<𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>(
        new_emitter: 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮
    ) -> Self::𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻<𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>;
}

pub trait 𝒆𝒎𝒊𝒕𝒕𝒆𝒓 {
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞: Default;
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;
    fn combine_results(x: &mut Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, y: Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞);
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum 𝐝𝐮𝐦𝐦𝐲_𝐞𝐦𝐢𝐭𝐭𝐞𝐫{}

impl 𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐝𝐮𝐦𝐦𝐲_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = core::convert::Infallible;
    #[inline(always)]
    fn combine_results(_: &mut (), _: ()) {
    }
}

impl 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐝𝐮𝐦𝐦𝐲_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    #[inline(always)]
    fn emit_i8(&mut self, _: i8) -> Result<(), core::convert::Infallible> {
        Ok(())
    }
    #[inline(always)]
    fn emit_u8(&mut self, _: u8) -> Result<(), core::convert::Infallible> {
        Ok(())
    }
    #[inline(always)]
    fn emit_bytes_i16(&mut self, _: i16) -> Result<(), core::convert::Infallible> {
        Ok(())
    }
    #[inline(always)]
    fn emit_bytes_u16(&mut self, _: u16) -> Result<(), core::convert::Infallible> {
        Ok(())
    }
    #[inline(always)]
    fn emit_bytes_i32(&mut self, _: i32) -> Result<(), core::convert::Infallible> {
        Ok(())
    }
    #[inline(always)]
    fn emit_bytes_u32(&mut self, _: u32) -> Result<(), core::convert::Infallible> {
        Ok(())
    }
    #[inline(always)]
    fn emit_bytes_i64(&mut self, _: i64) -> Result<(), core::convert::Infallible> {
        Ok(())
    }
    #[inline(always)]
    fn emit_bytes_u64(&mut self, _: u64) -> Result<(), core::convert::Infallible> {
        Ok(())
    }
    #[inline(always)]
    fn emit_bytes_i128(&mut self, _: i128) -> Result<(), core::convert::Infallible> {
        Ok(())
    }
    #[inline(always)]
    fn emit_bytes_u128(&mut self, _: u128) -> Result<(), core::convert::Infallible> {
        Ok(())
    }
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    fn emit_u8_array<const 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮: usize>(&mut self, _𝖺𝗋𝗋𝖺𝗒: [u8; 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮])
        -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
    where Self: 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, { 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮 }> {
        Ok(())
    }
}

// Simple 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 which doesn't emit enything but just counts size of emitted code.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝐝𝐲𝐧_𝐜𝐨𝐝𝐞_𝐬𝐢𝐳𝐞_𝐜𝐨𝐮𝐧𝐭𝐞𝐫(usize);

impl 𝐝𝐲𝐧_𝐜𝐨𝐝𝐞_𝐬𝐢𝐳𝐞_𝐜𝐨𝐮𝐧𝐭𝐞𝐫 {
    pub fn accumulated_size(self) -> usize {
        self.0
    }
}

impl 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐝𝐲𝐧_𝐜𝐨𝐝𝐞_𝐬𝐢𝐳𝐞_𝐜𝐨𝐮𝐧𝐭𝐞𝐫 {
    #[inline(always)]
    fn emit_i8(&mut self, _: i8) -> Result<(), Box<dyn std::error::Error>> {
        self.0 += 1;
        Ok(())
    }
    #[inline(always)]
    fn emit_u8(&mut self, _: u8) -> Result<(), Box<dyn std::error::Error>> {
        self.0 += 1;
        Ok(())
    }
    #[inline(always)]
    fn emit_i16(&mut self, _: i16) -> Result<(), Box<dyn std::error::Error>> {
        self.0 += 2;
        Ok(())
    }
    #[inline(always)]
    fn emit_u16(&mut self, _: u16) -> Result<(), Box<dyn std::error::Error>> {
        self.0 += 2;
        Ok(())
    }
    #[inline(always)]
    fn emit_i32(&mut self, _: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.0 += 4;
        Ok(())
    }
    #[inline(always)]
    fn emit_u32(&mut self, _: u32) -> Result<(), Box<dyn std::error::Error>> {
        self.0 += 4;
        Ok(())
    }
    #[inline(always)]
    fn emit_i64(&mut self, _: i64) -> Result<(), Box<dyn std::error::Error>> {
        self.0 += 8;
        Ok(())
    }
    #[inline(always)]
    fn emit_u64(&mut self, _: u64) -> Result<(), Box<dyn std::error::Error>> {
        self.0 += 8;
        Ok(())
    }
    #[inline(always)]
    fn emit_i128(&mut self, _: i128) -> Result<(), Box<dyn std::error::Error>> {
        self.0 += 16;
        Ok(())
    }
    #[inline(always)]
    fn emit_u128(&mut self, _: u128) -> Result<(), Box<dyn std::error::Error>> {
        self.0 += 16;
        Ok(())
    }
    #[inline(always)]
    fn emit_u8_slice(&mut self, 𝗌𝗅𝗂𝖼𝖾: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.0 += 𝗌𝗅𝗂𝖼𝖾.len();
        Ok(())
    }
}

// Simple 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 which doesn't emit enything but just counts size of emitted code.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝐜𝐨𝐝𝐞_𝐬𝐢𝐳𝐞_𝐜𝐨𝐮𝐧𝐭𝐞𝐫;

impl 𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐜𝐨𝐝𝐞_𝐬𝐢𝐳𝐞_𝐜𝐨𝐮𝐧𝐭𝐞𝐫 {
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = usize;
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = core::convert::Infallible;
    fn combine_results(x: &mut usize, y: usize) {
        *x += y
    }
}

impl 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐜𝐨𝐝𝐞_𝐬𝐢𝐳𝐞_𝐜𝐨𝐮𝐧𝐭𝐞𝐫 {
    #[inline(always)]
    fn emit_i8(&mut self, _: i8) -> Result<usize, core::convert::Infallible> {
        Ok(1)
    }
    #[inline(always)]
    fn emit_u8(&mut self, _: u8) -> Result<usize, core::convert::Infallible> {
        Ok(1)
    }
    #[inline(always)]
    fn emit_bytes_i16(&mut self, _: i16) -> Result<usize, core::convert::Infallible> {
        Ok(2)
    }
    #[inline(always)]
    fn emit_bytes_u16(&mut self, _: u16) -> Result<usize, core::convert::Infallible> {
        Ok(2)
    }
    #[inline(always)]
    fn emit_bytes_i32(&mut self, _: i32) -> Result<usize, core::convert::Infallible> {
        Ok(4)
    }
    #[inline(always)]
    fn emit_bytes_u32(&mut self, _: u32) -> Result<usize, core::convert::Infallible> {
        Ok(4)
    }
    #[inline(always)]
    fn emit_bytes_i64(&mut self, _: i64) -> Result<usize, core::convert::Infallible> {
        Ok(8)
    }
    #[inline(always)]
    fn emit_bytes_u64(&mut self, _: u64) -> Result<usize, core::convert::Infallible> {
        Ok(8)
    }
    #[inline(always)]
    fn emit_bytes_i128(&mut self, _: i128) -> Result<usize, core::convert::Infallible> {
        Ok(16)
    }
    #[inline(always)]
    fn emit_bytes_u128(&mut self, _: u128) -> Result<usize, core::convert::Infallible> {
        Ok(16)
    }
    #[inline(always)]
    fn emit_u8_slice(&mut self, 𝗌𝗅𝗂𝖼𝖾: &[u8]) -> Result<usize, core::convert::Infallible> {
        Ok(𝗌𝗅𝗂𝖼𝖾.len())
    }
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    fn emit_u8_array<const 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮: usize>(&mut self, _𝖺𝗋𝗋𝖺𝗒: [u8; 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮]) -> Result<usize, core::convert::Infallible>
    where Self: 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, { 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮 }> {
        Ok(𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮)
    }
}

#[allow(non_upper_case_globals)]
pub trait 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮, const 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮: usize>: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 {
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [𝓲𝓷𝓽_𝓽𝔂𝓹𝓮; 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮]
    ) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 1> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 1]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_u8(𝖺𝗋𝗋𝖺𝗒[0])
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 2> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 2]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_bytes_u16(u16::from_ne_bytes(𝖺𝗋𝗋𝖺𝗒))
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 3> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 3]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut result = self.emit_u8(𝖺𝗋𝗋𝖺𝗒[0])?;

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 2];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[1..3]);
        Self::combine_results(&mut result, self.emit_bytes_u16(u16::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        Ok(result)
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 4> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 4]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_bytes_u32(u32::from_ne_bytes(𝖺𝗋𝗋𝖺𝗒))
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 5> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 5]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut result = self.emit_u8(𝖺𝗋𝗋𝖺𝗒[0])?;

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 4];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[1..5]);
        Self::combine_results(&mut result, self.emit_bytes_u32(u32::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        Ok(result)
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 6> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 6]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 2];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[0..2]);
        let mut result = self.emit_u16(u16::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?;

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 4];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[2..6]);
        Self::combine_results(&mut result, self.emit_bytes_u32(u32::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        Ok(result)
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 7> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 7]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut result = self.emit_u8(𝖺𝗋𝗋𝖺𝗒[0])?;

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 2];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[1..3]);
        Self::combine_results(&mut result, self.emit_u16(u16::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 4];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[3..7]);
        Self::combine_results(&mut result, self.emit_bytes_u32(u32::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        Ok(result)
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 8> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 8]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_bytes_u64(u64::from_ne_bytes(𝖺𝗋𝗋𝖺𝗒))
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 9> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 9]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut result = self.emit_u8(𝖺𝗋𝗋𝖺𝗒[0])?;

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 8];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[1..9]);
        Self::combine_results(&mut result, self.emit_bytes_u64(u64::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        Ok(result)
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 10> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 10]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 2];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[0..2]);
        let mut result = self.emit_u16(u16::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?;

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 8];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[2..10]);
        Self::combine_results(&mut result, self.emit_bytes_u64(u64::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        Ok(result)
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 11> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 11]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut result = self.emit_u8(𝖺𝗋𝗋𝖺𝗒[0])?;

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 2];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[1..3]);
        Self::combine_results(&mut result, self.emit_u16(u16::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 8];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[3..11]);
        Self::combine_results(&mut result, self.emit_bytes_u64(u64::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        Ok(result)
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 12> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 12]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 4];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[0..4]);
        let mut result = self.emit_u32(u32::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?;

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 8];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[4..12]);
        Self::combine_results(&mut result, self.emit_bytes_u64(u64::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        Ok(result)
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 13> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 13]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut result = self.emit_u8(𝖺𝗋𝗋𝖺𝗒[0])?;

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 4];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[1..5]);
        Self::combine_results(&mut result, self.emit_u32(u32::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 8];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[5..13]);
        Self::combine_results(&mut result, self.emit_bytes_u64(u64::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        Ok(result)
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 14> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 14]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 2];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[0..2]);
        let mut result = self.emit_u16(u16::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?;

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 4];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[2..6]);
        Self::combine_results(&mut result, self.emit_u32(u32::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 8];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[6..14]);
        Self::combine_results(&mut result, self.emit_bytes_u64(u64::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        Ok(result)
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 15> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 15]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut result = self.emit_u8(𝖺𝗋𝗋𝖺𝗒[0])?;

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 2];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[1..3]);
        Self::combine_results(&mut result, self.emit_u16(u16::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 4];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[3..7]);
        Self::combine_results(&mut result, self.emit_u32(u32::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 8];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[7..15]);
        Self::combine_results(&mut result, self.emit_bytes_u64(u64::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        Ok(result)
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 16> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 16]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_bytes_u128(u128::from_ne_bytes(𝖺𝗋𝗋𝖺𝗒))
    }
}

// Note: currently the longest known ₓ86_64 instruction is 𝔵𝔯𝔢𝔩𝔢𝔞𝔰𝔢 𝔩𝔬𝔠𝔨 𝔞𝔡𝔡 %𝔣𝔰:𝔮𝔴𝔬𝔯𝔡 𝔭𝔱𝔯 [1234 + %𝔯8𝔡 + %𝔯9𝔡], 1234 — it's
// 16 bytes long (and thus is rejected by most CPUs).
//
// But there are certain impossible combinations of prefixes/options which are longer.
//
// Thus we need to provide 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 17>..=𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 21>.
impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 17> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 17]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut result = self.emit_u8(𝖺𝗋𝗋𝖺𝗒[0])?;

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 16];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[1..17]);
        Self::combine_results(&mut result, self.emit_bytes_u128(u128::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        Ok(result)
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 18> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 18]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 2];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[0..2]);
        let mut result = self.emit_u16(u16::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?;

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 16];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[2..18]);
        Self::combine_results(&mut result, self.emit_bytes_u128(u128::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        Ok(result)
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 19> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 19]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut result = self.emit_u8(𝖺𝗋𝗋𝖺𝗒[0])?;

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 2];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[1..3]);
        Self::combine_results(&mut result, self.emit_u16(u16::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 16];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[3..19]);
        Self::combine_results(&mut result, self.emit_bytes_u128(u128::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        Ok(result)
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 20> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 20]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 4];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[0..4]);
        let mut result = self.emit_u32(u32::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?;

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 16];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[4..20]);
        Self::combine_results(&mut result, self.emit_bytes_u128(u128::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        Ok(result)
    }
}

impl<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒊𝒏𝒕_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓<u8, 21> for 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮 {
    #[inline(always)]
    fn emit_array(
        &mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 21]
    ) -> Result<𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let mut result = self.emit_u8(𝖺𝗋𝗋𝖺𝗒[0])?;

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 4];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[0..4]);
        Self::combine_results(&mut result, self.emit_u32(u32::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        let mut 𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒 = [0u8; 16];
        𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒.copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[4..20]);
        Self::combine_results(&mut result, self.emit_bytes_u128(u128::from_ne_bytes(𝗌𝗎𝖻𝖺𝗋𝗋𝖺𝗒))?);

        Ok(result)
    }
}
