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
#![allow(uncommon_codepoints)]
#![allow(non_camel_case_types)]
// Prevent unavoidable consutions:
//     Identifier pair considered confusable between `𝑰𝒏𝒕𝒐` and `Into`. 𝑰𝒏𝒕𝒐 is 8086 instruction while Into is Rust trait.
//     Identifier pair considered confusable between `Add` and `𝑨𝒅𝒅`. 𝑨𝒅𝒅 is 8086/RISC-V instruction while Add is Rust trait.
//     Identifier pair considered confusable between `𝔞1` and `𝔞𝔩`. 𝔞1 is RISC-V register while 𝔞𝔩 is 8086 one.
// We have lots of such “nonconfuseable confusions”, but unfortunately it's crate-level lint.
#![allow(confusable_idents)]
#![cfg_attr(not(feature = "std"), no_std)]

#[path = "assembler/mod.rs"]
pub mod 𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿;

#[path = "disassembler/mod.rs"]
pub mod 𝗱𝗶𝘀𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿;
