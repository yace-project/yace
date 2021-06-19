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
//! # use yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝘅𝟴𝟲::{𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏,
//! #                            𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ,
//! #                            𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓};
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
//! #     &[0, 0, 1, 4, 1, 15, 1, 0x78, 0x56, 0x34, 0x12],
//! #     &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
//! # );
//! ```
//!
//! This code works fine, too:
//!
//! ```
//! # #![allow(uncommon_codepoints)]
//! # #![allow(non_camel_case_types)]
//! # use yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝘅𝟴𝟲::{𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏,
//! #                            𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ,
//! #                            𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓};
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
//! #     &[4, 0, 1, 4, 1, 5, 1, 0x78, 0x56, 0x34, 0x12],
//! #     &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
//! # );
//! ```
//!
//! But that one wouldn't compile because you can't use 𝔞𝔥 and 𝔯15𝔡 in the same instruction.
//!
//! ```compile_fail,E0277
//! # #![allow(uncommon_codepoints)]
//! # #![allow(non_camel_case_types)]
//! # use yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝘅𝟴𝟲::{𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏,
//! #                            𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ,
//! #                            𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓};
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
//! #     &[4, 0, 1, 4, 1, 15, 1, 0x78, 0x56, 0x34, 0x12],
//! #     &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
//! # );
//! ```

use yace_codegen::𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘;

#[path = "assembler_instructions.rs"]
pub mod 𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻𝘀;

#[path = "basic_assembler.rs"]
pub mod 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿;

#[path = "implementation.rs"]
pub mod 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻;

#[macro_use]
#[path = "macros.rs"]
pub mod 𝗺𝗮𝗰𝗿𝗼𝘀;

#[cfg(feature = "std")]
#[path = "tests.rs"]
#[cfg(test)]
mod 𝘁𝗲𝘀𝘁𝘀;

pub use 𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻𝘀::𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏;

pub use 𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻𝘀::𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏;

𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖙𝖗𝖆𝖎𝖙! {
    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝒘𝒊𝒕𝒉_𝒆𝒊𝒛_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝒘𝒊𝒕𝒉_𝒆𝒊𝒛_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝒘𝒊𝒕𝒉_𝒆𝒊𝒛_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝒘𝒊𝒕𝒉_𝒆𝒊𝒛_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝒘𝒊𝒕𝒉_𝒓𝒊𝒛_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₐᵥₓ512]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷 ₐᵥₓ512]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝒘𝒊𝒕𝒉_𝒓𝒊𝒛_𝔡𝔞𝔱𝔞32_ₐᵥₓ512 {
    }
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖙𝖗𝖚𝖈𝖙! {
    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞16[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝒘𝒊𝒕𝒉_𝒆𝒊𝒛_𝔡𝔞𝔱𝔞16
    for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞16[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>];

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝒘𝒊𝒕𝒉_𝒆𝒊𝒛_𝔡𝔞𝔱𝔞32
    for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>];

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞16[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝒘𝒊𝒕𝒉_𝒆𝒊𝒛_𝔡𝔞𝔱𝔞16
    for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞16[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>];

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝒘𝒊𝒕𝒉_𝒆𝒊𝒛_𝔡𝔞𝔱𝔞32
    for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>];

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₐᵥₓ512]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝒘𝒊𝒕𝒉_𝒓𝒊𝒛_𝔡𝔞𝔱𝔞32
    for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>];

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷 ₐᵥₓ512]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳_𝔡𝔞𝔱𝔞32_ₐᵥₓ512[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷 ₓ𝔦𝔷 ₐᵥₓ512]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝒘𝒊𝒕𝒉_𝒓𝒊𝒛_𝔡𝔞𝔱𝔞32_ₐᵥₓ512
    for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳_𝔡𝔞𝔱𝔞32_ₐᵥₓ512[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>];
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16;

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞16
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16;

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16;

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞16
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16;

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₐᵥₓ512]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512;

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷 ₐᵥₓ512]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳_𝔡𝔞𝔱𝔞32_ₐᵥₓ512
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝒘𝒊𝒕𝒉_𝒓𝒊𝒛_𝔡𝔞𝔱𝔞32_ₐᵥₓ512;
}

pub use 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝔞𝔡𝔡𝔯𝔢𝔰𝔰_16ᵇⁱᵗ;
pub use 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ;
pub use 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝔞𝔡𝔡𝔯𝔢𝔰𝔰_64ᵇⁱᵗ;
