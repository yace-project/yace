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

#ifndef 𝔜𝔄ℭ𝔈_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_𝔛86_ℭ𝔒𝔑𝔖𝔗𝔈𝔛𝔓ℜ_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_ℌ
#define 𝔜𝔄ℭ𝔈_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_𝔛86_ℭ𝔒𝔑𝔖𝔗𝔈𝔛𝔓ℜ_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_ℌ

#if __has_include(<span>)
#include <span>
#endif

#include "yace/foundation.h"

#include "yace/assembler/x86/byte_emit_assembler.h"
#include "yace/assembler/x86/options.h"

namespace 𝘆𝗮𝗰𝗲::𝘅𝟴𝟲 {

// Assembler capable of producing constexpr array of bytes.  Used mostly for testing.
//
// Note: we only provide 𝑒𝑚𝑖𝑡ₐᵣᵣ function and delete all other ones for simplicity. Only arrays up to 7 elements are supported
// and we store number of actual elements in the first element of an array.  This is done because many x86 instructions can have
// different sizes: 𝔯𝔢𝔵 could be omitted, 𝔳𝔢𝔵 could be 2 or 3 bytes, 𝔪𝔬𝔡𝔯𝔪 (wih 𝔰𝔦𝔟 and 𝔡𝔦𝔰𝔭) can take between one byte and 6
// and so on.  In all these cases it's important to return the same type, otherwise it's impossible to handle functions like
// 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑚𝑒𝑚𝑜𝑟𝑦_32𝑏𝑖𝑡_𝑎𝑑𝑑𝑟𝑒𝑠𝑠.
//
// Additionally we have special hack for 8-byte array: if we need to do that we go back “one step” and output one byte in the
// previous “slice”. This works because only one x86 instructions needs that: 𝔪𝔬𝔳 - and then only version without 𝔪𝔬𝔡𝔯𝔪.
template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼 = &::𝘆𝗮𝗰𝗲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔡𝔢𝔣𝔞𝔲𝔩𝔱, 𝐬𝐢𝐳𝐞 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 = 0>
class 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓;

template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝐬𝐢𝐳𝐞 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>
class 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓 : public 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼> {
  using ⒮ = 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>;

 public:
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 friend constexpr auto size(const 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓& assembler) noexcept -> 𝐬𝐢𝐳𝐞 {
    𝐬𝐢𝐳𝐞 result = 0;
    for (const auto& slice : assembler.𝗌𝗅𝗂𝖼𝖾𝗌) {
      result += slice[0];
    }
    return result;
  }

  // Note: C++17 makes us use an awkward 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦<size(𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻)>(𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻) form. In C++20 𝔞𝔯𝔯𝔞𝔶<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻> is available.
  template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮>
  friend constexpr auto 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(const 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓& assembler) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>)
      -> const std::array<𝐮𝐢𝐧𝐭₈, 𝓼𝓲𝔃𝓮> {
    𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖊𝖖𝖚𝖆𝖑(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓼𝓲𝔃𝓮, size(assembler));
    std::array<𝐮𝐢𝐧𝐭₈, 𝓼𝓲𝔃𝓮> result{};
    auto it = std::begin(result);
    for (const auto& slice : assembler.𝗌𝗅𝗂𝖼𝖾𝗌) {
      for (𝐬𝐢𝐳𝐞 pos = 1; pos <= slice[0]; ++pos) {
        *it++ = slice[pos];  // NOLINT(cppcoreguidelines-pro-bounds-constant-array-index)
      }
    }
    return result;
  }

 protected:
  template <typename 𝓼𝓶𝓪𝓵𝓵𝓮𝓻_𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝐬𝐢𝐳𝐞 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓(
      const 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓼𝓶𝓪𝓵𝓵𝓮𝓻_𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1>& base,
      const std::array<𝐮𝐢𝐧𝐭₈, 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>& array) noexcept
      : 𝗌𝗅𝗂𝖼𝖾𝗌(𝑚𝑎𝑘𝑒_𝑠𝑙𝑖𝑐𝑒𝑠(base, array)) {}

 private:
  constexpr auto emit₈(𝐮𝐢𝐧𝐭₈ value) -> void = delete;    // NOLINT(hicpp-use-equals-delete,modernize-use-equals-delete)
  constexpr auto emit₁₆(𝐮𝐢𝐧𝐭₁₆ value) -> void = delete;  // NOLINT(hicpp-use-equals-delete,modernize-use-equals-delete)
  constexpr auto emit₃₂(𝐮𝐢𝐧𝐭₃₂ value) -> void = delete;  // NOLINT(hicpp-use-equals-delete,modernize-use-equals-delete)
  constexpr auto emit₆₄(𝐮𝐢𝐧𝐭₆₄ value) -> void = delete;  // NOLINT(hicpp-use-equals-delete,modernize-use-equals-delete)

  template <typename 𝓼𝓶𝓪𝓵𝓵𝓮𝓻_𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, auto 𝓼𝓶𝓪𝓵𝓵𝓮𝓻_𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
  friend class 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓;
  template <𝐬𝐢𝐳𝐞 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑒𝑚𝑖𝑡ₐᵣᵣ(const std::array<𝐮𝐢𝐧𝐭₈, 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>& array) noexcept {
    return typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template ⒭<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 + 1>{*this, array};
  }

  template <typename 𝓫𝓲𝓰𝓰𝓮𝓻_𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, auto 𝓫𝓲𝓰𝓰𝓮𝓻_𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝐬𝐢𝐳𝐞 𝓫𝓲𝓰𝓰𝓮𝓻_𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>
  friend class 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓;
  template <typename 𝓼𝓶𝓪𝓵𝓵𝓮𝓻_𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝐬𝐢𝐳𝐞 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 static constexpr auto 𝑚𝑎𝑘𝑒_𝑠𝑙𝑖𝑐𝑒𝑠(
      const 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓼𝓶𝓪𝓵𝓵𝓮𝓻_𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1>& base,
      const std::array<𝐮𝐢𝐧𝐭₈, 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>& array) noexcept {
    std::array<std::array<𝐮𝐢𝐧𝐭₈, 8>, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮> result{};
    static_assert(𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 > 0);
    static_assert(𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 <= 8);
    if constexpr (𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 > 1) {
#ifdef __cpp_lib_constexpr_algorithms
      std::copy(std::begin(base.𝗌𝗅𝗂𝖼𝖾𝗌), std::end(base.𝗌𝗅𝗂𝖼𝖾𝗌), std::begin(result));
#else
      // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
      for (𝐬𝐢𝐳𝐞 counter = 0; counter < 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1; ++counter) {
        result[counter] = base.𝗌𝗅𝗂𝖼𝖾𝗌[counter];  // NOLINT(cppcoreguidelines-pro-bounds-constant-array-index)
      }
#endif
    }
    if constexpr (𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 < 8) {
      result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1][0] = 𝐮𝐢𝐧𝐭₈(𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱);
      if constexpr (𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 >= 1) {
        result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1][1] = array[0];
      }
      if constexpr (𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 >= 2) {
        result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1][2] = array[1];
      }
      if constexpr (𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 >= 3) {
        result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1][3] = array[2];
      }
      if constexpr (𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 >= 4) {
        result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1][4] = array[3];
      }
      if constexpr (𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 >= 5) {
        result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1][5] = array[4];
      }
      if constexpr (𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 >= 6) {
        result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1][6] = array[5];
      }
      if constexpr (𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 >= 7) {
        result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1][7] = array[6];
      }
    } else {
      static_assert(𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 > 1);
      result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 2][result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 2][0]++] = array[0];
      result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1][0] = 7;
      result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1][1] = array[1];
      result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1][2] = array[2];
      result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1][3] = array[3];
      result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1][4] = array[4];
      result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1][5] = array[5];
      result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1][6] = array[6];
      result[𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1][7] = array[7];
    }
    return result;
  }
  // It's actually an internal detail of 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓 implementation and is not supposed to be used by anything else
  // than 𝔞𝔯𝔯𝔞𝔶 (below), but C++20 requirements force us to make it public. Please don't access it directly outside of
  // 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓! Thankfully it's const thus at least it couldn't be [easily] modified.
 public:
  const std::array<std::array<𝐮𝐢𝐧𝐭₈, 8>, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮> 𝗌𝗅𝗂𝖼𝖾𝗌;  // NOLINT(misc-non-private-member-variables-in-classes)
};

template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
class 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 0> : public 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼> {
  using ⒮ = 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>;

 public:
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓() noexcept = default;

  template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮>
  friend constexpr auto 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(const 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓& assembler) noexcept -> const std::array<𝐮𝐢𝐧𝐭₈, 𝓼𝓲𝔃𝓮> {
    static_assert(𝓼𝓲𝔃𝓮 == 0);
    return {};
  }

 private:
  constexpr auto emit₈(𝐮𝐢𝐧𝐭₈ value) -> void = delete;    // NOLINT(hicpp-use-equals-delete,modernize-use-equals-delete)
  constexpr auto emit₁₆(𝐮𝐢𝐧𝐭₁₆ value) -> void = delete;  // NOLINT(hicpp-use-equals-delete,modernize-use-equals-delete)
  constexpr auto emit₃₂(𝐮𝐢𝐧𝐭₃₂ value) -> void = delete;  // NOLINT(hicpp-use-equals-delete,modernize-use-equals-delete)
  constexpr auto emit₆₄(𝐮𝐢𝐧𝐭₆₄ value) -> void = delete;  // NOLINT(hicpp-use-equals-delete,modernize-use-equals-delete)
  friend class 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>;

  template <𝐬𝐢𝐳𝐞 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑒𝑚𝑖𝑡ₐᵣᵣ(const std::array<𝐮𝐢𝐧𝐭₈, 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>& array) {
    return typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template ⒭<1>{*this, array};
  }
};

// Concrete implementation of assembler 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓.
template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼 = &::𝘆𝗮𝗰𝗲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔡𝔢𝔣𝔞𝔲𝔩𝔱, 𝐬𝐢𝐳𝐞 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 = 0>
class 𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓
    : public 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮> {
  using ⒮ = 𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>;

 private:
  template <𝐬𝐢𝐳𝐞 𝓷𝓮𝔀_𝓼𝓲𝔃𝓮>
  using ⒭ = 𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓷𝓮𝔀_𝓼𝓲𝔃𝓮>;

  template <𝐬𝐢𝐳𝐞 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>
  constexpr 𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓(
      const 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1>, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1>& base,
      const std::array<𝐮𝐢𝐧𝐭₈, 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>& array)
      : 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>(base, array) {}

  friend 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1>, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 - 1>;

  friend 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>;
};

template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
class 𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 0> : public 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 0>, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 0> {
  using ⒮ = 𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>;

 public:
  constexpr 𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓() = default;

 private:
  template <𝐬𝐢𝐳𝐞 𝓷𝓮𝔀_𝓼𝓲𝔃𝓮>
  using ⒭ = 𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓷𝓮𝔀_𝓼𝓲𝔃𝓮>;
  friend 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 0>, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 0>;
};

}  // namespace 𝘆𝗮𝗰𝗲::𝘅𝟴𝟲

#endif  // 𝔜𝔄ℭ𝔈_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_𝔛86_ℭ𝔒𝔑𝔖𝔗𝔈𝔛𝔓ℜ_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_ℌ
