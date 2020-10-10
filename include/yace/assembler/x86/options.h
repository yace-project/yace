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
#ifndef 𝔜𝔄ℭ𝔈_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_𝔛86_𝔒𝔓𝔗ℑ𝔒𝔑𝔖_ℌ
#define 𝔜𝔄ℭ𝔈_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_𝔛86_𝔒𝔓𝔗ℑ𝔒𝔑𝔖_ℌ

#include <cassert>

#include "yace/foundation.h"

namespace 𝘆𝗮𝗰𝗲::𝘅𝟴𝟲 {

// x86 emulation yace options
class 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 {  // NOLINT(cppcoreguidelines-pro-type-member-init, hicpp-member-init)
 public:
  enum class 𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 : 𝐢𝐧𝐭₈ { 𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16, 𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32, 𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16, 𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32, 𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32 };
  const 𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 x86_mode;
  // To write correct programs for 8086 it's important to have assembler which places 𝔯𝔢𝔭 prefix before 𝔢𝔰/𝔠𝔰/𝔰𝔰/𝔡𝔰.
  // Our assembler follows GNU Assembler convention by default and places these in 𝔢𝔰/𝔠𝔰/𝔰𝔰/𝔡𝔰 then 𝔯𝔢𝔭 order.
  // If you want the opposite order then set rep_before_segment to true.
  // More information on https://www.pcjs.org/documents/manuals/intel/8086
  //
  // Note: this only affects string instructions, not SSE instructions! SSE instructions are not available on 8086 thus it's not
  // important to put 𝔯𝔢𝔭 prefix before 𝔢𝔰/𝔠𝔰/𝔰𝔰/𝔡𝔰 for them.
  class 𝐫𝐞𝐩_𝐛𝐞𝐟𝐨𝐫𝐞_𝐬𝐞𝐠𝐦𝐞𝐧𝐭 {
   public:
    const 𝐛𝐨𝐨𝐥 rep_before_segment;
  };

  static const 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝔵86_16;
  static const 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝔵86_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32;
  static const 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝔵86_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16;
  static const 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝔵86_32;
  static const 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝔵86_64;

  static const 𝐫𝐞𝐩_𝐛𝐞𝐟𝐨𝐫𝐞_𝐬𝐞𝐠𝐦𝐞𝐧𝐭 𝔯𝔢𝔭_𝔟𝔢𝔣𝔬𝔯𝔢_𝔰𝔢𝔤𝔪𝔢𝔫𝔱;
  static const 𝐫𝐞𝐩_𝐛𝐞𝐟𝐨𝐫𝐞_𝐬𝐞𝐠𝐦𝐞𝐧𝐭 𝔯𝔢𝔭_𝔫𝔬𝔱_𝔟𝔢𝔣𝔬𝔯𝔢_𝔰𝔢𝔤𝔪𝔢𝔫𝔱;
};

inline constexpr 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔵86_16 = {.x86_mode = 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16};
inline constexpr 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔵86_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32 = {.x86_mode = 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32};
inline constexpr 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔵86_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16 = {.x86_mode = 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16};
inline constexpr 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔵86_32 = {.x86_mode = 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32};
inline constexpr 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔵86_64 = {.x86_mode = 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32};

inline constexpr 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐫𝐞𝐩_𝐛𝐞𝐟𝐨𝐫𝐞_𝐬𝐞𝐠𝐦𝐞𝐧𝐭 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔯𝔢𝔭_𝔟𝔢𝔣𝔬𝔯𝔢_𝔰𝔢𝔤𝔪𝔢𝔫𝔱 = {.rep_before_segment = true};
inline constexpr 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐫𝐞𝐩_𝐛𝐞𝐟𝐨𝐫𝐞_𝐬𝐞𝐠𝐦𝐞𝐧𝐭 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔯𝔢𝔭_𝔫𝔬𝔱_𝔟𝔢𝔣𝔬𝔯𝔢_𝔰𝔢𝔤𝔪𝔢𝔫𝔱 = {.rep_before_segment = false};

constexpr auto address_size(𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 x86_mode) -> 𝐬𝐢𝐳𝐞 {
  switch (x86_mode) {
    case 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16:
    case 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32:
      return 16;
    case 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16:
    case 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32:
      return 32;
    case 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32:
      return 64;
  }
  assert(false);
}

constexpr auto operand_size(𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 x86_mode) -> 𝐬𝐢𝐳𝐞 {
  switch (x86_mode) {
    case 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16:
    case 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16:
      return 16;
    case 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32:
    case 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32:
      return 32;
    case 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32:
      return 64;
  }
  assert(false);
}

𝖞𝖆𝖈𝖊_𝖒𝖆𝖐𝖊_𝖈𝖍𝖊𝖈𝖐_𝖛𝖆𝖗(𝔵86_𝔪𝔬𝔡𝔢, x86_mode, 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32);
𝖞𝖆𝖈𝖊_𝖒𝖆𝖐𝖊_𝖈𝖍𝖊𝖈𝖐_𝖛𝖆𝖗(𝔯𝔢𝔭_𝔟𝔢𝔣𝔬𝔯𝔢_𝔰𝔢𝔤𝔪𝔢𝔫𝔱, rep_before_segment, 𝔵86_𝔪𝔬𝔡𝔢<𝓸𝓹𝓽𝓲𝓸𝓷𝓼> == 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔵86_16);

#define 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖎𝖟𝖊𝖘(...) 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆(__VA_ARGS__), 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆_₆₄(__VA_ARGS__)
#define 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆(...) (8, __VA_ARGS__), (16, __VA_ARGS__), (32, __VA_ARGS__)
#define 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆_₆₄(...) (64, __VA_ARGS__)
#define 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(...) \
  𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆_₁₆(__VA_ARGS__), 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆_₃₂(__VA_ARGS__), \
      𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆_₆₄(__VA_ARGS__)
#define 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆_₁₆(...) (8, 16, __VA_ARGS__), (16, 16, __VA_ARGS__), (32, 16, __VA_ARGS__)
#define 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆_₃₂(...) (8, 32, __VA_ARGS__), (16, 32, __VA_ARGS__), (32, 32, __VA_ARGS__)
#define 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆_₆₄(...) \
  (64, 32, __VA_ARGS__), (8, 64, __VA_ARGS__), (16, 64, __VA_ARGS__), (32, 64, __VA_ARGS__), (64, 64, __VA_ARGS__)

}  // namespace 𝘆𝗮𝗰𝗲::𝘅𝟴𝟲

#endif  // 𝔜𝔄ℭ𝔈_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_𝔛86_𝔒𝔓𝔗ℑ𝔒𝔑𝔖_ℌ
