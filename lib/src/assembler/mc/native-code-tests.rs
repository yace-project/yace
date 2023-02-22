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

use super::{super::super::𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓,𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞};

struct 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec<u8>,
}

impl 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    pub const fn new() -> Self {
        𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
            𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec::<u8>::new(),
        }
    }
}

impl 𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    #[inline(always)]
    fn emit_u8(&mut self, value: u8) -> Result<(), Box<dyn std::error::Error>> {
        self.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.push(value);
        Ok(())
    }
}

#[test]
fn test_emit_labels_as_bytes_i16() {
    use super::super::𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;
    let mut machine_code = 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞::new();
    let label = machine_code.new_label();
    assert!(machine_code.set_label(label).is_ok());
    assert!(machine_code.emit_byte(label.into()).is_ok());
    assert!(machine_code.emit_2byte(label.into()).is_ok());
    assert!(machine_code.emit_4byte(label.into()).is_ok());
    assert!(machine_code.emit_8byte(label.into()).is_ok());
    assert_eq!(machine_code.finalize(0x1234i16, 10).unwrap(), 15);
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    assert!(machine_code.emit_code(&mut raw_emitter).is_ok());
    assert_eq!(
        &[0x34, 0x34, 0x12, 0x34, 0x12, 0x00, 0x00, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]);
}

#[test]
fn test_emit_labels_as_bytes_i32() {
    use super::super::𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;
    let mut machine_code = 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞::new();
    let label = machine_code.new_label();
    assert!(machine_code.set_label(label).is_ok());
    assert!(machine_code.emit_byte(label.into()).is_ok());
    assert!(machine_code.emit_2byte(label.into()).is_ok());
    assert!(machine_code.emit_4byte(label.into()).is_ok());
    assert!(machine_code.emit_8byte(label.into()).is_ok());
    assert_eq!(machine_code.finalize(0x12345678i32, 10).unwrap(), 15);
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    assert!(machine_code.emit_code(&mut raw_emitter).is_ok());
    assert_eq!(
        &[0x78, 0x78, 0x56, 0x78, 0x56, 0x34, 0x12, 0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00],
        &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]);
}

#[test]
fn test_emit_labels_as_bytes_i64() {
    use super::super::𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;
    let mut machine_code = 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞::new();
    let label = machine_code.new_label();
    assert!(machine_code.set_label(label).is_ok());
    assert!(machine_code.emit_byte(label.into()).is_ok());
    assert!(machine_code.emit_2byte(label.into()).is_ok());
    assert!(machine_code.emit_4byte(label.into()).is_ok());
    assert!(machine_code.emit_8byte(label.into()).is_ok());
    assert_eq!(machine_code.finalize(0x123456789abcdefi64, 10).unwrap(), 15);
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    assert!(machine_code.emit_code(&mut raw_emitter).is_ok());
    assert_eq!(
        &[0xef, 0xef, 0xcd, 0xef, 0xcd, 0xab, 0x89, 0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01],
        &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]);
}

#[test]
fn test_emit_labels_as_bytes_isize() {
    use super::super::𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;
    let mut machine_code = 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞::new();
    let label = machine_code.new_label();
    assert!(machine_code.set_label(label).is_ok());
    assert!(machine_code.emit_byte(label.into()).is_ok());
    assert!(machine_code.emit_2byte(label.into()).is_ok());
    assert!(machine_code.emit_4byte(label.into()).is_ok());
    assert!(machine_code.emit_8byte(label.into()).is_ok());
    #[cfg(target_pointer_width = "32")]
    assert_eq!(machine_code.finalize(0x12345678isize, 10).unwrap(), 15);
    #[cfg(target_pointer_width = "64")]
    assert_eq!(machine_code.finalize(0x123456789abcdefisize, 10).unwrap(), 15);
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    assert!(machine_code.emit_code(&mut raw_emitter).is_ok());
    #[cfg(target_pointer_width = "32")]
    assert_eq!(
        &[0x78, 0x78, 0x56, 0x78, 0x56, 0x34, 0x12, 0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00],
        &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]);
    #[cfg(target_pointer_width = "64")]
    assert_eq!(
        &[0xef, 0xef, 0xcd, 0xef, 0xcd, 0xab, 0x89, 0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01],
        &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]);
}

#[test]
fn test_emit_label_operations_i64() {
    use super::super::𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;
    let mut machine_code = 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞::new();
    let label0 = machine_code.new_label();
    let label1 = machine_code.new_label();
    assert!(machine_code.set_label(label0).is_ok());
    assert!(machine_code.emit_8byte(label0+1).is_ok());
    assert!(machine_code.emit_8byte(label0-1).is_ok());
    assert!(machine_code.emit_8byte(label0*3).is_ok());
    assert!(machine_code.emit_8byte(label0/7).is_ok());
    assert!(machine_code.emit_8byte(label0%7).is_ok());
    assert!(machine_code.emit_8byte((label0<<7)>>7).is_ok());
    assert!(machine_code.emit_8byte((label0<<7)>>7u8).is_ok());
    assert!(machine_code.emit_8byte(-label0).is_ok());
    assert!(machine_code.emit_8byte(!label0).is_ok());
    assert!(machine_code.emit_8byte(label1-label0).is_ok());
    assert!(machine_code.set_label(label1).is_ok());
    assert_eq!(machine_code.finalize(0x123456789abcdefi64, 10).unwrap(), 80);
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    assert!(machine_code.emit_code(&mut raw_emitter).is_ok());
    assert_eq!(
        &[0xf0, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01,
          0xee, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01,
          0xcd, 0x69, 0x03, 0x9d, 0x36, 0xd0, 0x69, 0x03,
          0x8f, 0x66, 0xcf, 0x5c, 0x33, 0x9c, 0x29, 0x00,
          0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0xff,
          0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01,
          0x11, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe,
          0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe,
          0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]);
}
