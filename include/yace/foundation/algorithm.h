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
#ifndef 𝔜𝔄ℭ𝔈_𝔉𝔒𝔘𝔑𝔇𝔄𝔗ℑ𝔒𝔑_𝔄𝔏𝔊𝔒ℜℑ𝔗ℌ𝔐_ℌ
#define 𝔜𝔄ℭ𝔈_𝔉𝔒𝔘𝔑𝔇𝔄𝔗ℑ𝔒𝔑_𝔄𝔏𝔊𝔒ℜℑ𝔗ℌ𝔐_ℌ

#include <algorithm>
#include <array>
#if __has_include(<bit>)
#include <bit>
#endif
#ifndef __cpp_lib_bit_cast
#include <cstring>
#endif
#include <tuple>

#include "yace/foundation/defines.h"
#include "yace/foundation/types.h"

namespace 𝘆𝗮𝗰𝗲 {

// Since std::bit_cast is C++20 we provide a suitable alternative for C++17 compilers.
#ifdef __cpp_lib_bit_cast
template <typename 𝓽𝓸, typename 𝓯𝓻𝓸𝓶>
𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑏𝑖𝑡_𝑐𝑎𝑠𝑡(const 𝓯𝓻𝓸𝓶& from) noexcept -> 𝓽𝓸 {
  return std::bit_cast(from);
}
#else
template <typename 𝓽𝓸, typename 𝓯𝓻𝓸𝓶>
𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 auto 𝑏𝑖𝑡_𝑐𝑎𝑠𝑡(const 𝓯𝓻𝓸𝓶& from) noexcept -> 𝓽𝓸 {
  𝓽𝓸 to;
  static_assert(sizeof to == sizeof from, "bit_cast: source and destination must be of same size");
  static_assert(std::is_trivially_copyable_v<𝓽𝓸>, "bit_cast: destination must be trivially copyable");
  static_assert(std::is_trivially_copyable_v<𝓯𝓻𝓸𝓶>, "bit_cast: source must be trivially copyable");
  std::memcpy(&to, &from, sizeof to);
  return to;
}
#endif

template <typename 𝓽𝓾𝓹𝓵𝓮>
𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(𝓽𝓾𝓹𝓵𝓮&& tuple) {
  return std::apply(
      [](auto&&... element) { return std::array{std::forward<decltype(element)>(element)...}; }, std::forward<𝓽𝓾𝓹𝓵𝓮>(tuple));
}

template <typename... 𝓽𝓾𝓹𝓵𝓮>
𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(𝓽𝓾𝓹𝓵𝓮&&... tuple) {
  return std::apply(
      [](auto&&... element) { return std::array{std::forward<decltype(element)>(element)...}; },
      std::tuple_cat(std::forward<𝓽𝓾𝓹𝓵𝓮>(tuple)...));
}

template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮, typename 𝓲𝓽𝓮𝓻𝓪𝓽𝓸𝓻>
𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(𝓲𝓽𝓮𝓻𝓪𝓽𝓸𝓻 it) -> std::array<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(*it)>, 𝓼𝓲𝔃𝓮> {
  std::array<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(*it)>, 𝓼𝓲𝔃𝓮> result{};
  if constexpr (𝓼𝓲𝔃𝓮 > 0) {  // This is mostly to make GCC (and other compilers) not see "0 < 0" comparison below.
#ifdef __cpp_lib_constexpr_algorithms
    std::copy(it, it + 𝓼𝓲𝔃𝓮, std::begin(result));
#else
    // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
    for (𝐬𝐢𝐳𝐞 counter = 0; counter < 𝓼𝓲𝔃𝓮; ++it, ++counter) {
      result[counter] = *it;  // NOLINT(cppcoreguidelines-pro-bounds-constant-array-index)
    }
#endif
  }
  return result;
}

// This is mostly for 𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓 - we couldn't determine size of the required array in the 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦 function since argument
// of a constexpr function is not constexpr!
#define 𝖞𝖆𝖈𝖊_𝖙𝖔_𝖆𝖗𝖗𝖆𝖞(𝓽) 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦<size(𝓽)>(𝓽)

#if __cpp_nontype_template_args >= 201911
template <auto 𝓸𝓫𝓳𝓮𝓬𝓽>
inline constexpr auto 𝔞𝔯𝔯𝔞𝔶 = 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦<size(𝓸𝓫𝓳𝓮𝓬𝓽)>(𝓸𝓫𝓳𝓮𝓬𝓽);
#endif

}  // namespace 𝘆𝗮𝗰𝗲

#endif  // 𝔜𝔄ℭ𝔈_𝔉𝔒𝔘𝔑𝔇𝔄𝔗ℑ𝔒𝔑_𝔄𝔏𝔊𝔒ℜℑ𝔗ℌ𝔐_ℌ
