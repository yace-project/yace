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
#ifndef 𝔜𝔄ℭ𝔈_𝔉𝔒𝔘𝔑𝔇𝔄𝔗ℑ𝔒𝔑_𝔗𝔜𝔓𝔈𝔖_ℌ
#define 𝔜𝔄ℭ𝔈_𝔉𝔒𝔘𝔑𝔇𝔄𝔗ℑ𝔒𝔑_𝔗𝔜𝔓𝔈𝔖_ℌ

#include <cinttypes>
#include <cstddef>
#include <limits>
#include <string>

namespace 𝘆𝗮𝗰𝗲 {

// Integer types.
using 𝐛𝐨𝐨𝐥 = bool;
#ifdef __cpp_lib_byte
using 𝐛𝐲𝐭𝐞 = std::byte;
#else
using 𝐛𝐲𝐭𝐞 = unsigned char;
#endif
#ifdef __cpp_lib_char8_t
using 𝐜𝐡𝐚𝐫 = char8_t;
using 𝐬𝐭𝐫𝐢𝐧𝐠 = std::u8string;
using 𝐬𝐭𝐫𝐢𝐧𝐠_𝐯𝐢𝐞𝐰 = std::u8string_view;
#else
using 𝐜𝐡𝐚𝐫 = unsigned char;
using 𝐬𝐭𝐫𝐢𝐧𝐠 = std::basic_string<𝐜𝐡𝐚𝐫>;
using 𝐬𝐭𝐫𝐢𝐧𝐠_𝐯𝐢𝐞𝐰 = std::basic_string_view<𝐜𝐡𝐚𝐫>;
#endif
using ℤ₂⁸ = std::int8_t;
using ℕ₂⁸ = std::uint8_t;
using 𝐢𝐧𝐭₈ = std::int8_t;
using 𝐮𝐢𝐧𝐭₈ = std::uint8_t;
using ℤ₂¹⁶ = std::int16_t;
using ℕ₂¹⁶ = std::uint16_t;
using 𝐢𝐧𝐭₁₆ = std::int16_t;
using 𝐮𝐢𝐧𝐭₁₆ = std::uint16_t;
using ℤ₂³² = std::int32_t;
using ℕ₂³² = std::uint32_t;
using 𝐢𝐧𝐭₃₂ = std::int32_t;
using 𝐮𝐢𝐧𝐭₃₂ = std::uint32_t;
using ℤ₂⁶⁴ = std::int64_t;
using ℕ₂⁶⁴ = std::uint64_t;
using 𝐢𝐧𝐭₆₄ = std::int64_t;
using 𝐮𝐢𝐧𝐭₆₄ = std::int64_t;
using 𝐢𝐧𝐭ₘₐₓ = std::intmax_t;
using 𝐮𝐢𝐧𝐭ₘₐₓ = std::uintmax_t;
#ifdef __SIZEOF_INT128__
using ℤ₂¹²⁸ = __int128_t;
using ℕ₂¹²⁸ = __uint128_t;
using 𝐢𝐧𝐭₁₂₈ = __int128_t;
using 𝐮𝐢𝐧𝐭₁₂₈ = __uint128_t;
#endif
// Pointer-related types.
using 𝐬𝐢𝐳𝐞 = std::size_t;
using 𝐩𝐭𝐫𝐝𝐢𝐟𝐟 = std::ptrdiff_t;
using 𝐧𝐮𝐥𝐥𝐩𝐭𝐫 = std::nullptr_t;

// Floating point types.
using 𝐟𝐥𝐨𝐚𝐭₃₂ = float;
static_assert(std::numeric_limits<𝐟𝐥𝐨𝐚𝐭₃₂>::is_iec559);
static_assert(sizeof(𝐟𝐥𝐨𝐚𝐭₃₂) * std::numeric_limits<𝐜𝐡𝐚𝐫>::digits == 32);
using 𝐟𝐥𝐨𝐚𝐭₆₄ = double;
static_assert(std::numeric_limits<𝐟𝐥𝐨𝐚𝐭₆₄>::is_iec559);
static_assert(sizeof(𝐟𝐥𝐨𝐚𝐭₆₄) * std::numeric_limits<𝐜𝐡𝐚𝐫>::digits == 64);

inline constexpr 𝐬𝐢𝐳𝐞 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢 = std::numeric_limits<𝐜𝐡𝐚𝐫>::digits;
// It's not clear if we may ever support platforms with non 8-bit byte. Just assert that for now.
static_assert(𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢 == 8);
template <typename 𝒯>
inline constexpr 𝐬𝐢𝐳𝐞 𝔟𝔦𝔱𝔰𝔦𝔷𝔢 = sizeof(𝒯) * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢;

#ifdef __cpp_lib_remove_cvref
template <typename 𝓣>
using 𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇 = std::remove_cvref_t<𝓣>;
#else
template <typename 𝓣>
using 𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇 = std::decay_t<𝓣>;
#endif

// Type tag to turn non-type argument into type argument.
// Could be used like this:
//   ⒯<true>
// or even like this:
//   ⒯<&Foo<𝒯₁...>::bar>
template <auto 𝒯>
class ⒯;

// Dependent true and false for static_assert.
template <typename 𝓣>
inline constexpr 𝐛𝐨𝐨𝐥 𝒕𝒓𝒖𝒆 = true;
template <typename 𝓣>
inline constexpr 𝐛𝐨𝐨𝐥 𝒇𝒂𝒍𝒔𝒆 = false;

}  // namespace 𝘆𝗮𝗰𝗲

#endif  // 𝔜𝔄ℭ𝔈_𝔉𝔒𝔘𝔑𝔇𝔄𝔗ℑ𝔒𝔑_𝔗𝔜𝔓𝔈𝔖_ℌ
