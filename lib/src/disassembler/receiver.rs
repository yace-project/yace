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

pub trait 𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓 {
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;
    fn get_u8(&mut self) -> Result<u8, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
    #[inline(always)]
    fn get_i8(&mut self) -> Result<i8, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_u8().map(|value| value as i8)
    }
    #[inline(always)]
    fn get_u16(&mut self) -> Result<u16, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        Ok((self.get_u8()? as u16) | (self.get_u8()? as u16) << 8)
    }
    #[inline(always)]
    fn get_i16(&mut self) -> Result<i16, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_u16().map(|value| value as i16)
    }
    #[inline(always)]
    fn get_u32(&mut self) -> Result<u32, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        Ok((self.get_u16()? as u32) | (self.get_u16()? as u32) << 16)
    }
    #[inline(always)]
    fn get_i32(&mut self) -> Result<i32, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_u32().map(|value| value as i32)
    }
    #[inline(always)]
    fn get_u64(&mut self) -> Result<u64, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        Ok((self.get_u32()? as u64) | (self.get_u32()? as u64) << 32)
    }
    #[inline(always)]
    fn get_i64(&mut self) -> Result<i64, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_u64().map(|value| value as i64)
    }
    #[cfg(has_i128)]
    #[inline(always)]
    fn get_u128(&mut self) -> Result<u128, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        Ok((self.get_u64()? as u128) | (self.get_u64()? as u128) << 64)
    }
    #[cfg(has_i128)]
    #[inline(always)]
    fn get_i128(&mut self) -> Result<i128, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.get_u128().map(|value| value as i128)
    }
}
