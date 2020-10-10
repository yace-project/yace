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
#ifndef 𝔜𝔄ℭ𝔈_𝔉𝔒𝔘𝔑𝔇𝔄𝔗ℑ𝔒𝔑_𝔒𝔓𝔗ℑ𝔒𝔑𝔖_ℌ
#define 𝔜𝔄ℭ𝔈_𝔉𝔒𝔘𝔑𝔇𝔄𝔗ℑ𝔒𝔑_𝔒𝔓𝔗ℑ𝔒𝔑𝔖_ℌ

#include <memory>
#include <type_traits>

#include "yace/foundation/types.h"

namespace 𝘆𝗮𝗰𝗲 {

// Basic yace options.
class 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 {
 public:
  // Note: if sanitity_checks are not enabled, exceptions should be disabled also.
  const 𝐛𝐨𝐨𝐥 sanity_checks;
  // Note: this replaces exceptions with asserts, it doesn't suppress exceptions in code supplied by user.
  const 𝐛𝐨𝐨𝐥 no_exceptions;

  static const 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝔡𝔢𝔣𝔞𝔲𝔩𝔱;
  static const 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝔞𝔰𝔰𝔢𝔯𝔱;
  static const 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝔢𝔵𝔠𝔢𝔭𝔱𝔦𝔬𝔫𝔰;
  static const 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝔫𝔬_𝔢𝔵𝔠𝔢𝔭𝔱𝔦𝔬𝔫𝔰;
  static const 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝔫𝔬𝔠𝔥𝔢𝔠𝔨𝔰;
};

inline constexpr 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔡𝔢𝔣𝔞𝔲𝔩𝔱 = {.sanity_checks = true, .no_exceptions = true};
inline constexpr 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔞𝔰𝔰𝔢𝔯𝔱 = {.sanity_checks = true, .no_exceptions = true};
inline constexpr 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔢𝔵𝔠𝔢𝔭𝔱𝔦𝔬𝔫𝔰 = {.sanity_checks = true, .no_exceptions = false};
inline constexpr 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔫𝔬_𝔢𝔵𝔠𝔢𝔭𝔱𝔦𝔬𝔫𝔰 = {.sanity_checks = true, .no_exceptions = true};
inline constexpr 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔫𝔬𝔠𝔥𝔢𝔠𝔨𝔰 = {.sanity_checks = false, .no_exceptions = false};

// Visual Stuido doesn't really have a working SFINAE but you can use a different, somewhat related trick:
// https://stackoverflow.com/questions/17201329/c11-ways-of-finding-if-a-type-has-member-function-or-supports-operator#17215386
// We are using decltype instead of std::enable_if_t here to make sure it works with Visual Studio and Clang.
// We have to use “auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼” and “auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼” because otherwise Visual Stuido poor SFINAE support leads to
// “variable template has already been defined” error — but that doesn't work with clang (but works with GCC).
#if defined(_MSC_VER) && !defined(__clang__)
#define 𝖞𝖆𝖈𝖊_𝖒𝖆𝖐𝖊_𝖈𝖍𝖊𝖈𝖐_𝖛𝖆𝖗(𝓿𝓪𝓻_𝓷𝓪𝓶𝓮, 𝓯𝓲𝓮𝓵𝓭_𝓷𝓪𝓶𝓮, 𝓭𝓮𝓯𝓪𝓾𝓵𝓽_𝓿𝓪𝓵𝓾𝓮) \
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, typename = void> \
  inline constexpr auto 𝓿𝓪𝓻_𝓷𝓪𝓶𝓮 = 𝓭𝓮𝓯𝓪𝓾𝓵𝓽_𝓿𝓪𝓵𝓾𝓮; \
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼> \
  inline constexpr auto 𝓿𝓪𝓻_𝓷𝓪𝓶𝓮<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, decltype((𝓸𝓹𝓽𝓲𝓸𝓷𝓼.𝓯𝓲𝓮𝓵𝓭_𝓷𝓪𝓶𝓮, void()))> = 𝓸𝓹𝓽𝓲𝓸𝓷𝓼.𝓯𝓲𝓮𝓵𝓭_𝓷𝓪𝓶𝓮; \
  template <auto* 𝓸𝓹𝓽𝓲𝓸𝓷𝓼> \
  inline constexpr auto 𝓿𝓪𝓻_𝓷𝓪𝓶𝓮<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, decltype((𝓸𝓹𝓽𝓲𝓸𝓷𝓼->𝓯𝓲𝓮𝓵𝓭_𝓷𝓪𝓶𝓮, void()))> = 𝓸𝓹𝓽𝓲𝓸𝓷𝓼->𝓯𝓲𝓮𝓵𝓭_𝓷𝓪𝓶𝓮
#else
#define 𝖞𝖆𝖈𝖊_𝖒𝖆𝖐𝖊_𝖈𝖍𝖊𝖈𝖐_𝖛𝖆𝖗(𝓿𝓪𝓻_𝓷𝓪𝓶𝓮, 𝓯𝓲𝓮𝓵𝓭_𝓷𝓪𝓶𝓮, 𝓭𝓮𝓯𝓪𝓾𝓵𝓽_𝓿𝓪𝓵𝓾𝓮) \
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, typename = void> \
  inline constexpr auto 𝓿𝓪𝓻_𝓷𝓪𝓶𝓮 = 𝓭𝓮𝓯𝓪𝓾𝓵𝓽_𝓿𝓪𝓵𝓾𝓮; \
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼> \
  inline constexpr auto 𝓿𝓪𝓻_𝓷𝓪𝓶𝓮<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, decltype((𝓸𝓹𝓽𝓲𝓸𝓷𝓼.𝓯𝓲𝓮𝓵𝓭_𝓷𝓪𝓶𝓮, void()))> = 𝓸𝓹𝓽𝓲𝓸𝓷𝓼.𝓯𝓲𝓮𝓵𝓭_𝓷𝓪𝓶𝓮; \
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼> \
  inline constexpr auto 𝓿𝓪𝓻_𝓷𝓪𝓶𝓮<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, decltype((𝓸𝓹𝓽𝓲𝓸𝓷𝓼->𝓯𝓲𝓮𝓵𝓭_𝓷𝓪𝓶𝓮, void()))> = 𝓸𝓹𝓽𝓲𝓸𝓷𝓼->𝓯𝓲𝓮𝓵𝓭_𝓷𝓪𝓶𝓮
#endif

𝖞𝖆𝖈𝖊_𝖒𝖆𝖐𝖊_𝖈𝖍𝖊𝖈𝖐_𝖛𝖆𝖗(𝔰𝔞𝔫𝔦𝔱𝔶_𝔠𝔥𝔢𝔠𝔨𝔰, sanity_checks, false);
𝖞𝖆𝖈𝖊_𝖒𝖆𝖐𝖊_𝖈𝖍𝖊𝖈𝖐_𝖛𝖆𝖗(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱, no_exceptions, true);
𝖞𝖆𝖈𝖊_𝖒𝖆𝖐𝖊_𝖈𝖍𝖊𝖈𝖐_𝖛𝖆𝖗(𝔣𝔞𝔰𝔱_𝔬𝔴𝔫𝔢𝔯, fast_owner, 𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>);

template <auto... 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
class 𝑜𝑝𝑡𝑖𝑜𝑛𝑠;

template <>
class 𝑜𝑝𝑡𝑖𝑜𝑛𝑠<> {};

#if __cpp_nontype_template_args >= 201911

template <auto 𝓸𝓹𝓽𝓲𝓸𝓷>
class 𝑜𝑝𝑡𝑖𝑜𝑛𝑠<𝓸𝓹𝓽𝓲𝓸𝓷> : public decltype(𝓸𝓹𝓽𝓲𝓸𝓷) {
  using 𝐨𝐩𝐭𝐢𝐨𝐧 = decltype(𝓸𝓹𝓽𝓲𝓸𝓷);

 public:
  constexpr 𝑜𝑝𝑡𝑖𝑜𝑛𝑠() : 𝐨𝐩𝐭𝐢𝐨𝐧(𝓸𝓹𝓽𝓲𝓸𝓷) {}
};

template <auto* 𝓸𝓹𝓽𝓲𝓸𝓷>
class 𝑜𝑝𝑡𝑖𝑜𝑛𝑠<𝓸𝓹𝓽𝓲𝓸𝓷> : public 𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(*𝓸𝓹𝓽𝓲𝓸𝓷)> {
  using 𝐨𝐩𝐭𝐢𝐨𝐧 = 𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(*𝓸𝓹𝓽𝓲𝓸𝓷)>;

 public:
  constexpr 𝑜𝑝𝑡𝑖𝑜𝑛𝑠() : 𝐨𝐩𝐭𝐢𝐨𝐧(*𝓸𝓹𝓽𝓲𝓸𝓷) {}
};

#else

template <auto 𝓸𝓹𝓽𝓲𝓸𝓷>
class 𝑜𝑝𝑡𝑖𝑜𝑛𝑠<𝓸𝓹𝓽𝓲𝓸𝓷> : public 𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(*𝓸𝓹𝓽𝓲𝓸𝓷)> {
  using 𝐨𝐩𝐭𝐢𝐨𝐧 = 𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(*𝓸𝓹𝓽𝓲𝓸𝓷)>;

 public:
  constexpr 𝑜𝑝𝑡𝑖𝑜𝑛𝑠() : 𝐨𝐩𝐭𝐢𝐨𝐧(*𝓸𝓹𝓽𝓲𝓸𝓷) {}
};

#endif

template <auto 𝓸𝓹𝓽𝓲𝓸𝓷, auto... 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
class 𝑜𝑝𝑡𝑖𝑜𝑛𝑠<𝓸𝓹𝓽𝓲𝓸𝓷, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼...> : public 𝑜𝑝𝑡𝑖𝑜𝑛𝑠<𝓸𝓹𝓽𝓲𝓸𝓷>, public 𝑜𝑝𝑡𝑖𝑜𝑛𝑠<𝓸𝓹𝓽𝓲𝓸𝓷𝓼...> {};

template <auto 𝓸𝓹𝓽𝓲𝓸𝓷, auto... 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
inline constexpr 𝑜𝑝𝑡𝑖𝑜𝑛𝑠<𝓸𝓹𝓽𝓲𝓸𝓷, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼...> 𝔬𝔭𝔱𝔦𝔬𝔫𝔰;

// Owner type for passing arguments.  Note: std::unique_ptr is inefficient because of Itanium C++ ABI, but is needed when exceptions
// are enabled.
template <typename 𝓣, auto 𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼 = &::𝘆𝗮𝗰𝗲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔡𝔢𝔣𝔞𝔲𝔩𝔱>
using 𝒐𝒘𝒏𝒆𝒓 = std::
    enable_if_t<std::is_pointer_v<𝓣>, std::conditional_t<𝔣𝔞𝔰𝔱_𝔬𝔴𝔫𝔢𝔯<𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝓣, std::unique_ptr<std::remove_pointer_t<𝓣>>>>;

namespace 𝗼𝘄𝗻𝗲𝗿 {

template <typename 𝓣>
class 𝒐𝒘𝒏𝒆𝒓_𝒕𝒚𝒑𝒆;

template <typename 𝓣>
class 𝒐𝒘𝒏𝒆𝒓_𝒕𝒚𝒑𝒆<𝓣*> {
 public:
  using 𝐭𝐲𝐩𝐞 = 𝓣;
};

template <typename 𝓣>
class 𝒐𝒘𝒏𝒆𝒓_𝒕𝒚𝒑𝒆<std::unique_ptr<𝓣>> {
 public:
  using 𝐭𝐲𝐩𝐞 = 𝓣;
};

}  // namespace 𝗼𝘄𝗻𝗲𝗿

template <typename 𝓣>
using 𝒐𝒘𝒏𝒆𝒓_𝒕𝒚𝒑𝒆 = typename 𝗼𝘄𝗻𝗲𝗿::𝒐𝒘𝒏𝒆𝒓_𝒕𝒚𝒑𝒆<𝓣>::𝓣;

#define 𝖞𝖆𝖈𝖊_𝖍𝖔𝖑𝖉_𝖔𝖜𝖓𝖊𝖗_𝖕𝖆𝖗𝖆𝖒𝖆𝖙𝖊𝖗(𝓱𝓸𝓵𝓭𝓮𝓻, 𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻) \
  std::unique_ptr<::𝘆𝗮𝗰𝗲::𝒐𝒘𝒏𝒆𝒓_𝒕𝒚𝒑𝒆<::𝘆𝗮𝗰𝗲::𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻)>>> 𝓱𝓸𝓵𝓭𝓮𝓻(std::move(𝓹𝓪𝓻𝓪𝓶𝓮𝓽𝓮𝓻))

#if __cplusplus >= 202002L
#define 𝖞𝖆𝖈𝖊_𝖕𝖆𝖘𝖘_𝖔𝖜𝖓𝖊𝖗_𝖆𝖗𝖌𝖚𝖒𝖊𝖓𝖙(𝓱𝓸𝓵𝓭𝓮𝓻, ...) \
  ::𝘆𝗮𝗰𝗲::𝒐𝒘𝒏𝒆𝒓<::𝘆𝗮𝗰𝗲::𝒐𝒘𝒏𝒆𝒓_𝒕𝒚𝒑𝒆<::𝘆𝗮𝗰𝗲::𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(𝓱𝓸𝓵𝓭𝓮𝓻)>> * __VA_OPT__(, ) __VA_ARGS__>(𝓱𝓸𝓵𝓭𝓮𝓻.release())
#elif defined(__GNUC__) || defined(_MSC_VER)
#define 𝖞𝖆𝖈𝖊_𝖕𝖆𝖘𝖘_𝖔𝖜𝖓𝖊𝖗_𝖆𝖗𝖌𝖚𝖒𝖊𝖓𝖙(𝓱𝓸𝓵𝓭𝓮𝓻, ...) \
  ::𝘆𝗮𝗰𝗲::𝒐𝒘𝒏𝒆𝒓<::𝘆𝗮𝗰𝗲::𝗼𝘄𝗻𝗲𝗿::𝒐𝒘𝒏𝒆𝒓_𝒕𝒚𝒑𝒆<::𝘆𝗮𝗰𝗲::𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(𝓱𝓸𝓵𝓭𝓮𝓻)>>*, ##__VA_ARGS__>(𝓱𝓸𝓵𝓭𝓮𝓻.release())
#else
#error Unknown compiler.
#endif

enum 𝐜𝐨𝐧𝐬𝐭𝐞𝐱𝐩𝐫_𝐟𝐫𝐢𝐞𝐧𝐝𝐥𝐲 { 𝔠𝔬𝔫𝔰𝔱𝔢𝔵𝔭𝔯_𝔲𝔫𝔣𝔯𝔦𝔢𝔫𝔡𝔩𝔶, 𝔠𝔬𝔫𝔰𝔱𝔢𝔵𝔭𝔯_𝔣𝔯𝔦𝔢𝔫𝔡𝔩𝔶 };

}  // namespace 𝘆𝗮𝗰𝗲

#endif  // 𝔜𝔄ℭ𝔈_𝔉𝔒𝔘𝔑𝔇𝔄𝔗ℑ𝔒𝔑_𝔒𝔓𝔗ℑ𝔒𝔑𝔖_ℌ
