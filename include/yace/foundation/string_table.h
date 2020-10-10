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
#ifndef 𝔜𝔄ℭ𝔈_𝔉𝔒𝔘𝔑𝔇𝔄𝔗ℑ𝔒𝔑_𝔖𝔗ℜℑ𝔑𝔊_𝔗𝔄𝔅𝔏𝔈_ℌ
#define 𝔜𝔄ℭ𝔈_𝔉𝔒𝔘𝔑𝔇𝔄𝔗ℑ𝔒𝔑_𝔖𝔗ℜℑ𝔑𝔊_𝔗𝔄𝔅𝔏𝔈_ℌ

#include <algorithm>
#include <string_view>

#include "yace/foundation/defines.h"
#include "yace/foundation/types.h"

namespace 𝘆𝗮𝗰𝗲 {

// Read https://www.akkadia.org/drepper/dsohowto.pdf to see why it's better to
// work with string tables in this manner.  Note: with C++ we don't need any
// preprocessor tricks here.
template <𝐬𝐢𝐳𝐞 𝓽𝓪𝓫𝓵𝓮_𝓼𝓲𝔃𝓮, 𝐬𝐢𝐳𝐞 𝓬𝓸𝓷𝓽𝓮𝓷𝓽_𝓼𝓲𝔃𝓮, typename 𝓬𝓱𝓪𝓻>
class 𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆 final {
 public:
  template <𝐬𝐢𝐳𝐞... 𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮>
  // NOLINTNEXTLINE(cppcoreguidelines-avoid-c-arrays,hicpp-avoid-c-arrays,modernize-avoid-c-arrays)
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆(const 𝓬𝓱𝓪𝓻 (&... string)[𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮]) : 𝗍𝖺𝖻𝗅𝖾(𝑚𝑎𝑘𝑒_𝑡𝑎𝑏𝑙𝑒(string...)) {}

  template <typename 𝒯>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator[](𝒯 index) const noexcept -> std::basic_string_view<const 𝓬𝓱𝓪𝓻> {
    static_assert(std::numeric_limits<𝒯>::is_integer);
    auto 𝗂𝗇𝖽𝖾𝗑 = 𝐬𝐢𝐳𝐞(index);
    // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-constant-array-index)
    return {𝗍𝖺𝖻𝗅𝖾.content.data() + 𝗍𝖺𝖻𝗅𝖾.offsets[𝗂𝗇𝖽𝖾𝗑], 𝐬𝐢𝐳𝐞(𝗍𝖺𝖻𝗅𝖾.offsets[𝗂𝗇𝖽𝖾𝗑 + 1]) - 𝐬𝐢𝐳𝐞(𝗍𝖺𝖻𝗅𝖾.offsets[𝗂𝗇𝖽𝖾𝗑]) - 1};
  }

  template <auto 𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, typename 𝒯>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑎𝑡(𝒯 index) const noexcept(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼->no_exceptions) -> std::basic_string_view<const 𝓬𝓱𝓪𝓻> {
    static_assert(std::numeric_limits<𝒯>::is_integer);
    𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖎𝖓_𝖗𝖆𝖓𝖌𝖊(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 0, 𝓽𝓪𝓫𝓵𝓮_𝓼𝓲𝔃𝓮);
    return operator[](index);
  }

  [[nodiscard]] auto size() const noexcept -> 𝐬𝐢𝐳𝐞 { return 𝓽𝓪𝓫𝓵𝓮_𝓼𝓲𝔃𝓮; }

 private:
  template <𝐬𝐢𝐳𝐞... 𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮>
  // NOLINTNEXTLINE(cppcoreguidelines-avoid-c-arrays,hicpp-avoid-c-arrays,modernize-avoid-c-arrays)
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 static constexpr auto 𝑚𝑎𝑘𝑒_𝑡𝑎𝑏𝑙𝑒(const 𝓬𝓱𝓪𝓻 (&... string)[𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮]) {
    decltype(𝗍𝖺𝖻𝗅𝖾) result = {};
    𝐬𝐢𝐳𝐞 output_offset = 0;
    𝐬𝐢𝐳𝐞 index_offset = 0;
#ifdef __cpp_lib_constexpr_algorithms
    ((std::copy(
          std::begin(string),
          std::end(string),
          std::begin(result.content) + (result.offsets[index_offset++] = 𝐨𝐟𝐟𝐬𝐞𝐭(output_offset))),
      output_offset += 𝐬𝐢𝐳𝐞(std::end(string) - std::begin(string))),
     ...);
#else
    ((
         [](auto input_begin, auto input_end, auto output) {
           // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
           for (const auto* input = input_begin; input != input_end; ++input, ++output) {
             *output = *input;
           }
         }(std::begin(string),
           std::end(string),
           // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-constant-array-index)
           std::begin(result.content) + (result.offsets[index_offset++] = 𝐨𝐟𝐟𝐬𝐞𝐭(output_offset))),
         output_offset += 𝐬𝐢𝐳𝐞(std::end(string) - std::begin(string))),
     ...);
#endif
    result.offsets[𝓽𝓪𝓫𝓵𝓮_𝓼𝓲𝔃𝓮] = 𝐨𝐟𝐟𝐬𝐞𝐭(output_offset);
    return result;
  }

  // Kludge for https://gcc.gnu.org/PR96716
#if defined(__GNUC__) && !defined(__clang__)
 public:
#endif
  using 𝐨𝐟𝐟𝐬𝐞𝐭 = std::conditional_t<
      std::numeric_limits<𝐮𝐢𝐧𝐭₈>::max() >= 𝓽𝓪𝓫𝓵𝓮_𝓼𝓲𝔃𝓮,
      𝐮𝐢𝐧𝐭₈,
      std::conditional_t<
          std::numeric_limits<𝐮𝐢𝐧𝐭₁₆>::max() >= 𝓽𝓪𝓫𝓵𝓮_𝓼𝓲𝔃𝓮,
          𝐮𝐢𝐧𝐭₁₆,
          std::conditional_t<std::numeric_limits<𝐮𝐢𝐧𝐭₃₂>::max() >= 𝓽𝓪𝓫𝓵𝓮_𝓼𝓲𝔃𝓮, 𝐮𝐢𝐧𝐭₃₂, 𝐮𝐢𝐧𝐭₆₄>>>;
  struct {
    std::array<𝐨𝐟𝐟𝐬𝐞𝐭, 𝓽𝓪𝓫𝓵𝓮_𝓼𝓲𝔃𝓮 + 1> offsets;
    std::array<𝓬𝓱𝓪𝓻, 𝓬𝓸𝓷𝓽𝓮𝓷𝓽_𝓼𝓲𝔃𝓮> content;
  } 𝗍𝖺𝖻𝗅𝖾;
};

template <𝐬𝐢𝐳𝐞... 𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮, typename 𝓬𝓱𝓪𝓻>
// NOLINTNEXTLINE(cppcoreguidelines-avoid-c-arrays,hicpp-avoid-c-arrays,modernize-avoid-c-arrays)
𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆(const 𝓬𝓱𝓪𝓻 (&... string)[𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮])->𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆<sizeof...(string), (𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮 + ...), 𝓬𝓱𝓪𝓻>;

template <𝐬𝐢𝐳𝐞 𝓽𝓪𝓫𝓵𝓮_𝓼𝓲𝔃𝓮, 𝐬𝐢𝐳𝐞 𝓵𝓲𝓷𝓮_𝓼𝓲𝔃𝓮, typename 𝓬𝓱𝓪𝓻 = 𝐜𝐡𝐚𝐫>
class 𝒔𝒉𝒐𝒓𝒕_𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆 final {
  // We don't support lines longer then what char may support. Note: we are using 𝐜𝐡𝐚𝐫, not 𝓬𝓱𝓪𝓻 to make sure it's unsigned.
  static_assert(𝓵𝓲𝓷𝓮_𝓼𝓲𝔃𝓮 + 1 <= std::numeric_limits<𝐜𝐡𝐚𝐫>::max());

 public:
  template <𝐬𝐢𝐳𝐞... 𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮>
  // NOLINTNEXTLINE(cppcoreguidelines-avoid-c-arrays,hicpp-avoid-c-arrays,modernize-avoid-c-arrays)
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒔𝒉𝒐𝒓𝒕_𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆(const 𝓬𝓱𝓪𝓻 (&... string)[𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮]) : 𝗍𝖺𝖻𝗅𝖾(𝑚𝑎𝑘𝑒_𝑡𝑎𝑏𝑙𝑒(string...)) {
    static_assert(((𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮 > 0) and ...));
    static_assert(((𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮 <= 256) and ...));
  }

  template <typename 𝒯>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator[](𝒯 index) const noexcept -> std::basic_string_view<const 𝓬𝓱𝓪𝓻> {
    static_assert(std::numeric_limits<𝒯>::is_integer);
    // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-constant-array-index)
    return {𝗍𝖺𝖻𝗅𝖾.data() + 𝐬𝐢𝐳𝐞(index) * (𝓵𝓲𝓷𝓮_𝓼𝓲𝔃𝓮 + 1) + 1, 𝐜𝐡𝐚𝐫(𝗍𝖺𝖻𝗅𝖾[𝐬𝐢𝐳𝐞(index) * (𝓵𝓲𝓷𝓮_𝓼𝓲𝔃𝓮 + 1)])};
  }

  template <auto 𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, typename 𝒯>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑎𝑡(𝒯 index) const noexcept(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼->no_exceptions) -> std::basic_string_view<const 𝓬𝓱𝓪𝓻> {
    static_assert(std::numeric_limits<𝒯>::is_integer);
    𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖎𝖓_𝖗𝖆𝖓𝖌𝖊(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 0, 𝓽𝓪𝓫𝓵𝓮_𝓼𝓲𝔃𝓮);
    return operator[](index);
  }

  [[nodiscard]] auto size() const noexcept -> 𝐬𝐢𝐳𝐞 { return 𝓽𝓪𝓫𝓵𝓮_𝓼𝓲𝔃𝓮; }

 private:
  template <𝐬𝐢𝐳𝐞... 𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮>
  // NOLINTNEXTLINE(cppcoreguidelines-avoid-c-arrays,hicpp-avoid-c-arrays,modernize-avoid-c-arrays)
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 static constexpr auto 𝑚𝑎𝑘𝑒_𝑡𝑎𝑏𝑙𝑒(const 𝓬𝓱𝓪𝓻 (&... string)[𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮])
      -> std::array<𝓬𝓱𝓪𝓻, 𝓽𝓪𝓫𝓵𝓮_𝓼𝓲𝔃𝓮*(𝓵𝓲𝓷𝓮_𝓼𝓲𝔃𝓮 + 1)> {
    static_assert(((𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮 <= 𝓵𝓲𝓷𝓮_𝓼𝓲𝔃𝓮) and ...));
    std::array<𝓬𝓱𝓪𝓻, 𝓽𝓪𝓫𝓵𝓮_𝓼𝓲𝔃𝓮*(𝓵𝓲𝓷𝓮_𝓼𝓲𝔃𝓮 + 1)> result = {};
    𝐬𝐢𝐳𝐞 output_offset = 0;
#ifdef __cpp_lib_constexpr_algorithms
    // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-constant-array-index)
    ((result[output_offset++] = 𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮 - 1,
      std::copy(std::begin(string), std::end(string), std::begin(result) + output_offset),
      output_offset += 𝓵𝓲𝓷𝓮_𝓼𝓲𝔃𝓮),
     ...);
#else
    ((
         // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-constant-array-index)
         result[output_offset++] = 𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮 - 1,
         [](auto input_begin, auto input_end, auto output) {
           // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
           for (const auto* input = input_begin; input != input_end; ++input, ++output) {
             *output = *input;
           }
         }(std::begin(string), std::end(string), std::begin(result) + output_offset),
         output_offset += 𝓵𝓲𝓷𝓮_𝓼𝓲𝔃𝓮),
     ...);
#endif
    return result;
  }

  // Kludge for https://gcc.gnu.org/PR96716
#if defined(__GNUC__) && !defined(__clang__)
 public:
#endif
  const std::array<𝓬𝓱𝓪𝓻, 𝓽𝓪𝓫𝓵𝓮_𝓼𝓲𝔃𝓮*(𝓵𝓲𝓷𝓮_𝓼𝓲𝔃𝓮 + 1)> 𝗍𝖺𝖻𝗅𝖾;
};

template <𝐬𝐢𝐳𝐞... 𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮, typename 𝓬𝓱𝓪𝓻>
// NOLINTNEXTLINE(cppcoreguidelines-avoid-c-arrays,hicpp-avoid-c-arrays,modernize-avoid-c-arrays)
𝒔𝒉𝒐𝒓𝒕_𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆(const 𝓬𝓱𝓪𝓻 (&... string)[𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮])->𝒔𝒉𝒐𝒓𝒕_𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆<sizeof...(string), std::max({𝓼𝓽𝓻𝓲𝓷𝓰_𝓼𝓲𝔃𝓮...}), 𝓬𝓱𝓪𝓻>;

}  // namespace 𝘆𝗮𝗰𝗲

#endif  // 𝔜𝔄ℭ𝔈_𝔉𝔒𝔘𝔑𝔇𝔄𝔗ℑ𝔒𝔑_𝔖𝔗ℜℑ𝔑𝔊_𝔗𝔄𝔅𝔏𝔈_ℌ
