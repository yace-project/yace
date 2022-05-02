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

//! Documentation doesn't exist yet but we have some doctests.
//!
//! This code will compile:
//!
//! ```
//! # #![allow(uncommon_codepoints)]
//! # #![allow(non_camel_case_types)]
//! # use yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::{𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓,𝘅𝟴𝟲_𝟲𝟰::{𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ,𝑪𝑷𝑼,𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32}};
//! #
//! # struct 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec<u8>,
//! # }
//! #
//! # impl 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     pub const fn new() -> Self {
//! #         𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #             𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec::<u8>::new(),
//! #         }
//! #     }
//! # }
//! #
//! # impl 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
//! #     type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = ();
//! #
//! #     fn emit_u8(&mut self, value: u8) -> Result<(), ()> {
//! #         self.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.push(value);
//! #         Ok(())
//! #     }
//! # }
//! #
//! # type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ =
//! #     𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
//! # let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
//! 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32::from(&mut raw_emitter)
//!     .add((
//!         𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔞𝔩,
//!         𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ
//!             .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔰𝔭)
//!             .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔯15𝔡)
//!             .with_disp(0x12345678),
//!     ))
//!     .expect("Testing assembler");
//! # assert_eq!(
//! #     &[0x67, 0x42, 0x02, 0x84, 0x3c, 0x78, 0x56, 0x34, 0x12],
//! #     &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
//! # );
//! ```
//!
//! This code works fine, too:
//!
//! ```
//! # #![allow(uncommon_codepoints)]
//! # #![allow(non_camel_case_types)]
//! # use yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::{𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓,𝘅𝟴𝟲_𝟲𝟰::{𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ,𝑪𝑷𝑼,𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32}};
//! #
//! # struct 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec<u8>,
//! # }
//! #
//! # impl 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     pub const fn new() -> Self {
//! #         𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #             𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec::<u8>::new(),
//! #         }
//! #     }
//! # }
//! #
//! # impl 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
//! #     type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = ();
//! #
//! #     fn emit_u8(&mut self, value: u8) -> Result<(), ()> {
//! #         self.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.push(value);
//! #         Ok(())
//! #     }
//! # }
//! #
//! # type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ =
//! #     𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
//! # let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
//! 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32::from(&mut raw_emitter)
//!     .add((
//!         𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔞𝔥,
//!         𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ
//!             .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔰𝔭)
//!             .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔟𝔭)
//!             .with_disp(0x12345678),
//!     ))
//!     .expect("Testing assembler");
//! # assert_eq!(
//! #     &[0x67, 0x02, 0xa4, 0x2c, 0x78, 0x56, 0x34, 0x12],
//! #     &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
//! # );
//! ```
//!
//! But that one wouldn't compile because you can't use 𝔞𝔥 and 𝔯15𝔡 in the same instruction.
//!
//! ```compile_fail,E0277
//! # #![allow(uncommon_codepoints)]
//! # #![allow(non_camel_case_types)]
//! # use yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::{𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓,𝘅𝟴𝟲_𝟲𝟰::{𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ,𝑪𝑷𝑼,𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32}};
//! #
//! # struct 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec<u8>,
//! # }
//! #
//! # impl 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     pub const fn new() -> Self {
//! #         𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #             𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec::<u8>::new(),
//! #         }
//! #     }
//! # }
//! #
//! # impl 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
//! #     type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = ();
//! #
//! #     fn emit_u8(&mut self, value: u8) -> Result<(), ()> {
//! #         self.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.push(value);
//! #         Ok(())
//! #     }
//! # }
//! #
//! # type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ =
//! #     𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
//! # let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
//! 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32::from(&mut raw_emitter)
//!     .add((
//!         𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔞𝔥,
//!         𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ
//!             .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔰𝔭)
//!             .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔯15𝔡)
//!             .with_disp(0x12345678),
//!     ))
//!     .expect("Testing assembler");
//! # assert_eq!(
//! #     &[0x67, 0x02, 0xa4, 0x3c, 0x78, 0x56, 0x34, 0x12],
//! #     &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
//! # );
//! ```

#[path = "address.rs"]
pub(crate) mod 𝗮𝗱𝗱𝗿𝗲𝘀𝘀;

#[path = "basic_assembler.rs"]
pub(crate) mod 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿;

#[path = "emit_instruction_bytes.rs"]
pub(crate) mod 𝗲𝗺𝗶𝘁_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻_𝗯𝘆𝘁𝗲𝘀;

#[macro_use]
#[path = "macros.rs"]
mod 𝗺𝗮𝗰𝗿𝗼𝘀;

#[path = "operands.rs"]
pub(crate) mod 𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀;

#[cfg(feature = "std")]
#[path = "tests.rs"]
#[cfg(test)]
mod 𝘁𝗲𝘀𝘁𝘀;

// We are implementing two-level scheme with a set of Xₓₓ traits (from 𝑨𝒂𝒂 to 𝑿𝒕𝒆𝒔𝒕) and a set of ₓₓₓ_𝒘𝒊𝒕𝒉 traits.
//
// This allows us to avoid combinatiorial explosion: there are more than dozen of types which may represent just general purpose
// register argument and for three-argument instruction it would mean there are almost two thousand variants.
//
// Xₓₓ traits uses traits from 𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀 module to convert arguments to less diverse set and then ₓₓₓ_𝒘𝒊𝒕𝒉 traits implement the
// remaining combinations.
//
// If all arguments would be handled identically, then of course, it wouldn't make much sense to even have these two levels.
// Instead some instructions use not 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕 trait, but more specialized conversion traits.
//
// E.g. shift instructions use 𝒔𝒉𝒊𝒇𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕 trait which only support i8 and 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ arguments (and they
// just accept them without conversion).
//
// Thus way we both avoid the combinatorial explosion and guarantee that incorrect registers are excluded during the compilation
// time. Not only this makes debugging easier, this also means that we can still thinking about reporting these particular errors.
//
// Note: Even with this approach we have some extra variants to implement (e.g. add have separate version for accumulator and
// 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫 and 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 because there are special version for 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫 and immediate, but overral it's less than 2x
// superfluous instructions.  This is considered acceptable.

use yace_codegen::𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓𝖘;
𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓𝖘!{}

use 𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏;
use 𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓;
use 𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒄𝒐𝒖𝒏𝒕𝒆𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅;
use 𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒔𝒕𝒓𝒊𝒏𝒈_𝒐𝒑𝒆𝒓𝒂𝒏𝒅;
use 𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒊𝒐_𝒐𝒑𝒆𝒓𝒂𝒏𝒅;
use 𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅;
use 𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒔𝒐𝒖𝒓𝒄𝒆_𝒔𝒕𝒓𝒊𝒏𝒈_𝒐𝒑𝒆𝒓𝒂𝒏𝒅;
use 𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒙𝒍𝒂𝒕_𝒐𝒑𝒆𝒓𝒂𝒏𝒅;
