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
pub trait 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 {
    fn get_u8(&mut self) -> Result<u8, Box<dyn std::error::Error>>;
    #[inline(always)]
    fn get_i8(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        self.get_u8().map(|value| value as i8)
    }
    #[inline(always)]
    fn get_u16(&mut self) -> Result<u16, Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_u8()? as u16) << 8 | (self.get_u8()? as u16))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_u8()? as u16) | (self.get_u8()? as u16) << 8)}
    }
    #[inline(always)]
    fn get_i16(&mut self) -> Result<i16, Box<dyn std::error::Error>> {
        self.get_u16().map(|value| value as i16)
    }
    #[inline(always)]
    fn get_u32(&mut self) -> Result<u32, Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_u16()? as u32) << 16 | (self.get_u16()? as u32))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_u16()? as u32) | (self.get_u16()? as u32) << 16)}
    }
    #[inline(always)]
    fn get_i32(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        self.get_u32().map(|value| value as i32)
    }
    #[inline(always)]
    fn get_u64(&mut self) -> Result<u64, Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_u32()? as u64) << 32 | (self.get_u32()? as u64))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_u32()? as u64) | (self.get_u32()? as u64) << 32)}
    }
    #[inline(always)]
    fn get_i64(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        self.get_u64().map(|value| value as i64)
    }
    #[inline(always)]
    fn get_u128(&mut self) -> Result<u128, Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_u64()? as u128) << 64 | (self.get_u64()? as u128))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_u64()? as u128) | (self.get_u64()? as u128) << 64)}
    }
    #[inline(always)]
    fn get_i128(&mut self) -> Result<i128, Box<dyn std::error::Error>> {
        self.get_u128().map(|value| value as i128)
    }
}

#[cfg(feature = "std")]
impl<'ᵖʳᵒᵈᵘᶜᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ> 𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 for dyn 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 + 'ᵖʳᵒᵈᵘᶜᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ {
    fn get_u8(&mut self) -> Result<u8, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_u8(self)
    }
    #[inline(always)]
    fn get_i8(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_i8(self)
    }
    #[inline(always)]
    fn get_bytes_u16(&mut self) -> Result<u16, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_u16(self)
    }
    #[inline(always)]
    fn get_bytes_i16(&mut self) -> Result<i16, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_i16(self)
    }
    #[inline(always)]
    fn get_bytes_u32(&mut self) -> Result<u32, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_u32(self)
    }
    #[inline(always)]
    fn get_bytes_i32(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_i32(self)
    }
    #[inline(always)]
    fn get_bytes_u64(&mut self) -> Result<u64, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_u64(self)
    }
    #[inline(always)]
    fn get_bytes_i64(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_i64(self)
    }
    #[inline(always)]
    fn get_bytes_u128(&mut self) -> Result<u128, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_u128(self)
    }
    #[inline(always)]
    fn get_bytes_i128(&mut self) -> Result<i128, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_i128(self)
    }
}

#[cfg(feature = "std")]
impl<'ᵖʳᵒᵈᵘᶜᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ> 𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 for dyn 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 + 'ᵖʳᵒᵈᵘᶜᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ {
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = Box<dyn std::error::Error>;
}

#[cfg(feature = "std")]
pub trait 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 {
    fn get_u16(&mut self) -> Result<u16, Box<dyn std::error::Error>>;
    #[inline(always)]
    fn get_i16(&mut self) -> Result<i16, Box<dyn std::error::Error>> {
        self.get_u16().map(|value| value as i16)
    }
    #[inline(always)]
    fn get_u32(&mut self) -> Result<u32, Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_u16()? as u32) << 16 | (self.get_u16()? as u32))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_u16()? as u32) | (self.get_u16()? as u32) << 16)}
    }
    #[inline(always)]
    fn get_i32(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        self.get_u32().map(|value| value as i32)
    }
    #[inline(always)]
    fn get_u64(&mut self) -> Result<u64, Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_u32()? as u64) << 32 | (self.get_u32()? as u64))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_u32()? as u64) | (self.get_u32()? as u64) << 32)}
    }
    #[inline(always)]
    fn get_i64(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        self.get_u64().map(|value| value as i64)
    }
    #[inline(always)]
    fn get_u128(&mut self) -> Result<u128, Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_u64()? as u128) << 64 | (self.get_u64()? as u128))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_u64()? as u128) | (self.get_u64()? as u128) << 64)}
    }
    #[inline(always)]
    fn get_i128(&mut self) -> Result<i128, Box<dyn std::error::Error>> {
        self.get_u128().map(|value| value as i128)
    }
}

#[cfg(feature = "std")]
impl<'ᵖʳᵒᵈᵘᶜᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ> 𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 for dyn 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 + 'ᵖʳᵒᵈᵘᶜᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ {
    #[inline(always)]
    fn get_u16(&mut self) -> Result<u16, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_u16(self)
    }
    #[inline(always)]
    fn get_i16(&mut self) -> Result<i16, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_i16(self)
    }
    #[inline(always)]
    fn get_parcels_u32(&mut self) -> Result<u32, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_u32(self)
    }
    #[inline(always)]
    fn get_parcels_i32(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_i32(self)
    }
    #[inline(always)]
    fn get_parcels_u64(&mut self) -> Result<u64, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_u64(self)
    }
    #[inline(always)]
    fn get_parcels_i64(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_i64(self)
    }
    #[inline(always)]
    fn get_parcels_u128(&mut self) -> Result<u128, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_u128(self)
    }
    #[inline(always)]
    fn get_parcels_i128(&mut self) -> Result<i128, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_i128(self)
    }
}

#[cfg(feature = "std")]
impl<'ᵖʳᵒᵈᵘᶜᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ> 𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 for dyn 𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 + 'ᵖʳᵒᵈᵘᶜᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ {
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = Box<dyn std::error::Error>;
}

#[cfg(feature = "std")]
pub trait 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 {
    fn get_u32(&mut self) -> Result<u32, Box<dyn std::error::Error>>;
    #[inline(always)]
    fn get_i32(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        self.get_u32().map(|value| value as i32)
    }
    #[inline(always)]
    fn get_u64(&mut self) -> Result<u64, Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_u32()? as u64) << 32 | (self.get_u32()? as u64))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_u32()? as u64) | (self.get_u32()? as u64) << 32)}
    }
    #[inline(always)]
    fn get_i64(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        self.get_u64().map(|value| value as i64)
    }
    #[inline(always)]
    fn get_u128(&mut self) -> Result<u128, Box<dyn std::error::Error>> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_u64()? as u128) << 64 | (self.get_u64()? as u128))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_u64()? as u128) | (self.get_u64()? as u128) << 64)}
    }
    #[inline(always)]
    fn get_i128(&mut self) -> Result<i128, Box<dyn std::error::Error>> {
        self.get_u128().map(|value| value as i128)
    }
}

#[cfg(feature = "std")]
impl<'ᵖʳᵒᵈᵘᶜᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ> 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 for dyn 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 + 'ᵖʳᵒᵈᵘᶜᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ {
    #[inline(always)]
    fn get_u32(&mut self) -> Result<u32, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_u32(self)
    }
    #[inline(always)]
    fn get_i32(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_i32(self)
    }
    #[inline(always)]
    fn get_u64(&mut self) -> Result<u64, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_u64(self)
    }
    #[inline(always)]
    fn get_i64(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_i64(self)
    }
    #[inline(always)]
    fn get_u128(&mut self) -> Result<u128, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_u128(self)
    }
    #[inline(always)]
    fn get_i128(&mut self) -> Result<i128, Box<dyn std::error::Error>> {
        <Self as 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_i128(self)
    }
}

#[cfg(feature = "std")]
impl<'ᵖʳᵒᵈᵘᶜᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ> 𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 for dyn 𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 + 'ᵖʳᵒᵈᵘᶜᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ {
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = Box<dyn std::error::Error>;
}

pub trait 𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓: 𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 {
    fn get_u8(&mut self) -> Result<u8, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
    #[inline(always)]
    fn get_i8(&mut self) -> Result<i8, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_u8().map(|value| value as i8)
    }
    #[inline(always)]
    fn get_bytes_u16(&mut self) -> Result<u16, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_u8()? as u16) << 8 | (self.get_u8()? as u16))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_u8()? as u16) | (self.get_u8()? as u16) << 8)}
    }
    #[inline(always)]
    fn get_bytes_i16(&mut self) -> Result<i16, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_bytes_u16().map(|value| value as i16)
    }
    #[inline(always)]
    fn get_bytes_u32(&mut self) -> Result<u32, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_bytes_u16()? as u32) << 16 | (self.get_bytes_u16()? as u32))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_bytes_u16()? as u32) | (self.get_bytes_u16()? as u32) << 16)}
    }
    #[inline(always)]
    fn get_bytes_i32(&mut self) -> Result<i32, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_bytes_u32().map(|value| value as i32)
    }
    #[inline(always)]
    fn get_bytes_u64(&mut self) -> Result<u64, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_bytes_u32()? as u64) << 32 | (self.get_bytes_u32()? as u64))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_bytes_u32()? as u64) | (self.get_bytes_u32()? as u64) << 32)}
    }
    #[inline(always)]
    fn get_bytes_i64(&mut self) -> Result<i64, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_bytes_u64().map(|value| value as i64)
    }
    #[inline(always)]
    fn get_bytes_u128(&mut self) -> Result<u128, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_bytes_u64()? as u128) << 64 | (self.get_bytes_u64()? as u128))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_bytes_u64()? as u128) | (self.get_bytes_u64()? as u128) << 64)}
    }
    #[inline(always)]
    fn get_bytes_i128(&mut self) -> Result<i128, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_bytes_u128().map(|value| value as i128)
    }
}

impl<𝓟𝓻𝓸𝓭𝓾𝓬𝓮𝓻: 𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 + ?Sized> 𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 for 𝓟𝓻𝓸𝓭𝓾𝓬𝓮𝓻 {
    #[inline(always)]
    fn get_u16(&mut self) -> Result<u16, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        <Self as 𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_bytes_u16(self)
    }
    #[inline(always)]
    fn get_i16(&mut self) -> Result<i16, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        <Self as 𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_bytes_i16(self)
    }
    #[inline(always)]
    fn get_parcels_u32(&mut self) -> Result<u32, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        <Self as 𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_bytes_u32(self)
    }
    #[inline(always)]
    fn get_parcels_i32(&mut self) -> Result<i32, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        <Self as 𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_bytes_i32(self)
    }
    #[inline(always)]
    fn get_parcels_u64(&mut self) -> Result<u64, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        <Self as 𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_bytes_u64(self)
    }
    #[inline(always)]
    fn get_parcels_i64(&mut self) -> Result<i64, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        <Self as 𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_bytes_i64(self)
    }
    #[inline(always)]
    fn get_parcels_u128(&mut self) -> Result<u128, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        <Self as 𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_bytes_u128(self)
    }
    #[inline(always)]
    fn get_parcels_i128(&mut self) -> Result<i128, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        <Self as 𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_bytes_i128(self)
    }
}

pub trait 𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓: 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 {
    fn get_u16(&mut self) -> Result<u16, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
    #[inline(always)]
    fn get_i16(&mut self) -> Result<i16, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_u16().map(|value| value as i16)
    }
    #[inline(always)]
    fn get_parcels_u32(&mut self) -> Result<u32, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_u16()? as u32) << 16 | (self.get_u16()? as u32))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_u16()? as u32) | (self.get_u16()? as u32) << 16)}
    }
    #[inline(always)]
    fn get_parcels_i32(&mut self) -> Result<i32, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_parcels_u32().map(|value| value as i32)
    }
    #[inline(always)]
    fn get_parcels_u64(&mut self) -> Result<u64, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_parcels_u32()? as u64) << 32 | (self.get_u32()? as u64))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_parcels_u32()? as u64) | (self.get_u32()? as u64) << 32)}
    }
    #[inline(always)]
    fn get_parcels_i64(&mut self) -> Result<i64, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_parcels_u64().map(|value| value as i64)
    }
    #[inline(always)]
    fn get_parcels_u128(&mut self) -> Result<u128, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_parcels_u64()? as u128) << 64 | (self.get_u64()? as u128))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_parcels_u64()? as u128) | (self.get_u64()? as u128) << 64)}
    }
    #[inline(always)]
    fn get_parcels_i128(&mut self) -> Result<i128, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_parcels_u128().map(|value| value as i128)
    }
}

impl<𝓟𝓻𝓸𝓭𝓾𝓬𝓮𝓻: 𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 + ?Sized> 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 for 𝓟𝓻𝓸𝓭𝓾𝓬𝓮𝓻 {
    #[inline(always)]
    fn get_u32(&mut self) -> Result<u32, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        <Self as 𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_parcels_u32(self)
    }
    #[inline(always)]
    fn get_i32(&mut self) -> Result<i32, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        <Self as 𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_parcels_i32(self)
    }
    #[inline(always)]
    fn get_u64(&mut self) -> Result<u64, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        <Self as 𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_parcels_u64(self)
    }
    #[inline(always)]
    fn get_i64(&mut self) -> Result<i64, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        <Self as 𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_parcels_i64(self)
    }
    #[inline(always)]
    fn get_u128(&mut self) -> Result<u128, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        <Self as 𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_parcels_u128(self)
    }
    #[inline(always)]
    fn get_i128(&mut self) -> Result<i128, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        <Self as 𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓>::get_parcels_i128(self)
    }
}

pub trait 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓: 𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 {
    fn get_u32(&mut self) -> Result<u32, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
    #[inline(always)]
    fn get_i32(&mut self) -> Result<i32, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_u32().map(|value| value as i32)
    }
    #[inline(always)]
    fn get_u64(&mut self) -> Result<u64, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_u32()? as u64) << 32 | (self.get_u32()? as u64))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_u32()? as u64) | (self.get_u32()? as u64) << 32)}
    }
    #[inline(always)]
    fn get_i64(&mut self) -> Result<i64, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_u64().map(|value| value as i64)
    }
    #[inline(always)]
    fn get_u128(&mut self) -> Result<u128, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        #[cfg(target_endian = "big")]
        {Ok((self.get_u64()? as u128) << 64 | (self.get_u64()? as u128))}
        #[cfg(target_endian = "little")]
        {Ok((self.get_u64()? as u128) | (self.get_u64()? as u128) << 64)}
    }
    #[inline(always)]
    fn get_i128(&mut self) -> Result<i128, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_u128().map(|value| value as i128)
    }
}

pub trait 𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 {
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;
}
