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

#[path = "producer.rs"]
mod 𝗽𝗿𝗼𝗱𝘂𝗰𝗲𝗿_𝗺𝗼𝗱𝘂𝗹𝗲;

pub use 𝗽𝗿𝗼𝗱𝘂𝗰𝗲𝗿_𝗺𝗼𝗱𝘂𝗹𝗲::𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓;
pub use 𝗽𝗿𝗼𝗱𝘂𝗰𝗲𝗿_𝗺𝗼𝗱𝘂𝗹𝗲::𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓;
pub use 𝗽𝗿𝗼𝗱𝘂𝗰𝗲𝗿_𝗺𝗼𝗱𝘂𝗹𝗲::𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓;
pub use 𝗽𝗿𝗼𝗱𝘂𝗰𝗲𝗿_𝗺𝗼𝗱𝘂𝗹𝗲::𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓;

#[cfg(feature = "std")]
pub use 𝗽𝗿𝗼𝗱𝘂𝗰𝗲𝗿_𝗺𝗼𝗱𝘂𝗹𝗲::𝒅𝒚𝒏_𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓;
#[cfg(feature = "std")]
pub use 𝗽𝗿𝗼𝗱𝘂𝗰𝗲𝗿_𝗺𝗼𝗱𝘂𝗹𝗲::𝒅𝒚𝒏_𝒑𝒂𝒓𝒄𝒆𝒍_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓;
#[cfg(feature = "std")]
pub use 𝗽𝗿𝗼𝗱𝘂𝗰𝗲𝗿_𝗺𝗼𝗱𝘂𝗹𝗲::𝒅𝒚𝒏_𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒘𝒐𝒓𝒅_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓;

#[macro_use]
#[path = "risc-v/mod.rs"]
pub mod 𝗿𝗶𝘀𝗰_𝘃;

#[path = "rv32e.rs"]
pub mod 𝗿𝘃𝟯𝟮𝗲;

#[path = "rv32i.rs"]
pub mod 𝗿𝘃𝟯𝟮𝗶;

#[path = "rv64i.rs"]
pub mod 𝗿𝘃𝟲𝟰𝗶;
