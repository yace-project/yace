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
#ifndef 𝔜𝔄ℭ𝔈_𝔉𝔒𝔘𝔑𝔇𝔄𝔗ℑ𝔒𝔑_𝔖𝔄𝔑ℑ𝔗𝔜_ℭℌ𝔈ℭ𝔎𝔖_ℌ
#define 𝔜𝔄ℭ𝔈_𝔉𝔒𝔘𝔑𝔇𝔄𝔗ℑ𝔒𝔑_𝔖𝔄𝔑ℑ𝔗𝔜_ℭℌ𝔈ℭ𝔎𝔖_ℌ

#include <cassert>
#include <stdexcept>

#include "yace/foundation/defines.h"
#include "yace/foundation/options.h"
#include "yace/foundation/types.h"

namespace 𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀 {

// clang-format off
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖊𝖖𝖚𝖆𝖑(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑒𝑞𝑢𝑎𝑙(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖓𝖔𝖙_𝖊𝖖𝖚𝖆𝖑(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑛𝑜𝑡_𝑒𝑞𝑢𝑎𝑙(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖆𝖇𝖔𝖛𝖊(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑎𝑏𝑜𝑣𝑒(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖓𝖔𝖙_𝖆𝖇𝖔𝖛𝖊(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑛𝑜𝑡_𝑎𝑏𝑜𝑣𝑒(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖆𝖇𝖔𝖛𝖊_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑎𝑏𝑜𝑣𝑒_𝑜𝑟_𝑒𝑞𝑢𝑎𝑙(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖓𝖔𝖙_𝖆𝖇𝖔𝖛𝖊_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑛𝑜𝑡_𝑎𝑏𝑜𝑣𝑒_𝑜𝑟_𝑒𝑞𝑢𝑎𝑙(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖇𝖊𝖑𝖔𝖜(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑏𝑒𝑙𝑜𝑤(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖓𝖔𝖙_𝖇𝖊𝖑𝖔𝖜(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑛𝑜𝑡_𝑏𝑒𝑙𝑜𝑤(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖇𝖊𝖑𝖔𝖜_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑏𝑒𝑙𝑜𝑤_𝑜𝑟_𝑒𝑞𝑢𝑎𝑙(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖓𝖔𝖙_𝖇𝖊𝖑𝖔𝖜_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑛𝑜𝑡_𝑏𝑒𝑙𝑜𝑤_𝑜𝑟_𝑒𝑞𝑢𝑎𝑙(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖔𝖓𝖊_𝖔𝖋(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, ...) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑖𝑠_𝑜𝑛𝑒_𝑜𝑓(𝓪𝓬𝓽𝓾𝓪𝓵, __VA_ARGS__))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖎𝖓_𝖗𝖆𝖓𝖌𝖊(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓿𝓪𝓵𝓾𝓮, 𝓶𝓲𝓷𝓲𝓶𝓾𝓶, 𝓶𝓪𝔁𝓲𝓶𝓾𝓶) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑖𝑠_𝑖𝑛_𝑟𝑎𝑛𝑔𝑒(𝓿𝓪𝓵𝓾𝓮, 𝓶𝓲𝓷𝓲𝓶𝓾𝓶, 𝓶𝓪𝔁𝓲𝓶𝓾𝓶))

#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖊𝖖𝖚𝖆𝖑_𝖋𝖔𝖑𝖉(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, (::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑒𝑞𝑢𝑎𝑙(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) && ...))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖓𝖔𝖙_𝖊𝖖𝖚𝖆𝖑_𝖋𝖔𝖑𝖉(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, (::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑛𝑜𝑡_𝑒𝑞𝑢𝑎𝑙(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) && ...))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖆𝖇𝖔𝖛𝖊_𝖋𝖔𝖑𝖉(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, (::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑎𝑏𝑜𝑣𝑒(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) && ...))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖓𝖔𝖙_𝖆𝖇𝖔𝖛𝖊_𝖋𝖔𝖑𝖉(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, (::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑛𝑜𝑡_𝑎𝑏𝑜𝑣𝑒(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) && ...))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖆𝖇𝖔𝖛𝖊_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑_𝖋𝖔𝖑𝖉(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, (::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑎𝑏𝑜𝑣𝑒_𝑜𝑟_𝑒𝑞𝑢𝑎𝑙(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) && ...))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖓𝖔𝖙_𝖆𝖇𝖔𝖛𝖊_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑_𝖋𝖔𝖑𝖉(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, (::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑛𝑜𝑡_𝑎𝑏𝑜𝑣𝑒_𝑜𝑟_𝑒𝑞𝑢𝑎𝑙(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) && ...))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖇𝖊𝖑𝖔𝖜_𝖋𝖔𝖑𝖉(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, (::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑏𝑒𝑙𝑜𝑤(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) && ...))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖓𝖔𝖙_𝖇𝖊𝖑𝖔𝖜_𝖋𝖔𝖑𝖉(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, (::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑛𝑜𝑡_𝑏𝑒𝑙𝑜𝑤(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) && ...))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖇𝖊𝖑𝖔𝖜_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑_𝖋𝖔𝖑𝖉(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, (::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑏𝑒𝑙𝑜𝑤_𝑜𝑟_𝑒𝑞𝑢𝑎𝑙(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) && ...))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖓𝖔𝖙_𝖇𝖊𝖑𝖔𝖜_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑_𝖋𝖔𝖑𝖉(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, (::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑛𝑜𝑡_𝑏𝑒𝑙𝑜𝑤_𝑜𝑟_𝑒𝑞𝑢𝑎𝑙(𝓪𝓬𝓽𝓾𝓪𝓵, 𝓮𝔁𝓹𝓮𝓬𝓽𝓮𝓭) && ...))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖔𝖓𝖊_𝖔𝖋_𝖋𝖔𝖑𝖉(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓪𝓬𝓽𝓾𝓪𝓵, ...) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, (::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑖𝑠_𝑜𝑛𝑒_𝑜𝑓(𝓪𝓬𝓽𝓾𝓪𝓵, __VA_ARGS__) && ...))
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖎𝖓_𝖗𝖆𝖓𝖌𝖊_𝖋𝖔𝖑𝖉(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓿𝓪𝓵𝓾𝓮, 𝓶𝓲𝓷𝓲𝓶𝓾𝓶, 𝓶𝓪𝔁𝓲𝓶𝓾𝓶) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, (::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑖𝑠_𝑖𝑛_𝑟𝑎𝑛𝑔𝑒(𝓿𝓪𝓵𝓾𝓮, 𝓶𝓲𝓷𝓲𝓶𝓾𝓶, 𝓶𝓪𝔁𝓲𝓶𝓾𝓶) && ...))
// clang-format on

#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓬𝓱𝓮𝓬𝓴) \
  do { \
    if constexpr (𝔰𝔞𝔫𝔦𝔱𝔶_𝔠𝔥𝔢𝔠𝔨𝔰<𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) { \
      if constexpr (𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) { \
        /* This is strange code to circumvent the fact that MSVC doesn't allow use of assert in costexpr functions. */ \
        if (not(𝓬𝓱𝓮𝓬𝓴)) { \
          assert(𝓬𝓱𝓮𝓬𝓴); \
        } \
      } else { \
        if (not(𝓬𝓱𝓮𝓬𝓴)) { \
          𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖙𝖍𝖗𝖔𝖜(std::out_of_range("YACE: unequal value")); \
        } \
      } \
    } \
  } while (false)

#ifdef __GNUC__
#ifdef __EXCEPTIONS
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖙𝖍𝖗𝖔𝖜(𝓮𝔁𝓹𝓻𝓮𝓼𝓼𝓲𝓸𝓷) throw(𝓮𝔁𝓹𝓻𝓮𝓼𝓼𝓲𝓸𝓷)
#else
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖙𝖍𝖗𝖔𝖜(𝓮𝔁𝓹𝓻𝓮𝓼𝓼𝓲𝓸𝓷) static_assert(𝒇𝒂𝒍𝒔𝒆<𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼>)
#endif
#else
#define 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖙𝖍𝖗𝖔𝖜(𝓮𝔁𝓹𝓻𝓮𝓼𝓼𝓲𝓸𝓷) throw(𝓮𝔁𝓹𝓻𝓮𝓼𝓼𝓲𝓸𝓷)
#endif

// Sanity check options.
template <𝐛𝐨𝐨𝐥 𝓼𝓪𝓷𝓲𝓽𝔂_𝓬𝓱𝓮𝓬𝓴𝓼, 𝐛𝐨𝐨𝐥 𝓷𝓸_𝓮𝔁𝓬𝓮𝓹𝓽𝓲𝓸𝓷𝓼>
class 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 {
  static_assert(𝓼𝓪𝓷𝓲𝓽𝔂_𝓬𝓱𝓮𝓬𝓴𝓼 or 𝓷𝓸_𝓮𝔁𝓬𝓮𝓹𝓽𝓲𝓸𝓷𝓼, "If sanitity_checks are not enabled, exceptions should be disabled also.");

 public:
  // Note: if sanitity_checks are not enabled, exceptions should be disabled also.
  const 𝐛𝐨𝐨𝐥 sanity_checks = 𝓼𝓪𝓷𝓲𝓽𝔂_𝓬𝓱𝓮𝓬𝓴𝓼;
  // Note: this replaces exceptions with asserts, it doesn't suppress exceptions in code supplied by user.
  const 𝐛𝐨𝐨𝐥 no_exceptions = 𝓷𝓸_𝓮𝔁𝓬𝓮𝓹𝓽𝓲𝓸𝓷𝓼;

  static const 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝔬𝔭𝔱𝔦𝔬𝔫𝔰;
};

template <>
inline constexpr 𝐨𝐩𝐭𝐢𝐨𝐧𝐬<true, true> 𝐨𝐩𝐭𝐢𝐨𝐧𝐬<true, true>::𝔬𝔭𝔱𝔦𝔬𝔫𝔰 = {};
template <>
inline constexpr 𝐨𝐩𝐭𝐢𝐨𝐧𝐬<true, false> 𝐨𝐩𝐭𝐢𝐨𝐧𝐬<true, false>::𝔬𝔭𝔱𝔦𝔬𝔫𝔰 = {};
template <>
inline constexpr 𝐨𝐩𝐭𝐢𝐨𝐧𝐬<false, true> 𝐨𝐩𝐭𝐢𝐨𝐧𝐬<false, true>::𝔬𝔭𝔱𝔦𝔬𝔫𝔰 = {};

template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
inline constexpr auto* 𝔬𝔭𝔱𝔦𝔬𝔫𝔰 = &𝐨𝐩𝐭𝐢𝐨𝐧𝐬<𝔰𝔞𝔫𝔦𝔱𝔶_𝔠𝔥𝔢𝔠𝔨𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>>::𝔬𝔭𝔱𝔦𝔬𝔫𝔰;

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖎𝖒𝖕𝖑𝖊_𝖘𝖆𝖓𝖎𝖙𝖞_𝖈𝖍𝖊𝖈𝖐(𝓷𝓪𝓶𝓮, 𝓸𝓹𝓮𝓻𝓪𝓽𝓸𝓻) \
  template <typename 𝒯₁, typename 𝒯₂> \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝓷𝓪𝓶𝓮(𝒯₁&& actual, 𝒯₂&& expected) noexcept->𝐛𝐨𝐨𝐥 { \
    static_assert(std::is_same_v<decltype(std::declval<𝒯₁> == std::declval<𝒯₁>), 𝐛𝐨𝐨𝐥>); \
    return actual 𝓸𝓹𝓮𝓻𝓪𝓽𝓸𝓻 expected; \
  } \
  template <typename 𝒯₁, typename 𝒯₂> \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑛𝑜𝑡_##𝓷𝓪𝓶𝓮(𝒯₁&& actual, 𝒯₂&& expected) noexcept->𝐛𝐨𝐨𝐥 { \
    static_assert(std::is_same_v<decltype(std::declval<𝒯₁> == std::declval<𝒯₁>), 𝐛𝐨𝐨𝐥>); \
    return not(actual 𝓸𝓹𝓮𝓻𝓪𝓽𝓸𝓻 expected); \
  }
𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖎𝖒𝖕𝖑𝖊_𝖘𝖆𝖓𝖎𝖙𝖞_𝖈𝖍𝖊𝖈𝖐, (𝑒𝑞𝑢𝑎𝑙, ==), (𝑎𝑏𝑜𝑣𝑒, >), (𝑏𝑒𝑙𝑜𝑤, <), (𝑎𝑏𝑜𝑣𝑒_𝑜𝑟_𝑒𝑞𝑢𝑎𝑙, >=), (𝑏𝑒𝑙𝑜𝑤_𝑜𝑟_𝑒𝑞𝑢𝑎𝑙, <=))
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖎𝖒𝖕𝖑𝖊_𝖘𝖆𝖓𝖎𝖙𝖞_𝖈𝖍𝖊𝖈𝖐

template <typename 𝒯₁, typename... 𝒯ₓ>
𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑖𝑠_𝑜𝑛𝑒_𝑜𝑓(𝒯₁ actual, 𝒯ₓ... expected) noexcept -> 𝐛𝐨𝐨𝐥 {
  static_assert((std::is_same_v<decltype(std::declval<𝒯₁> == std::declval<𝒯ₓ>), 𝐛𝐨𝐨𝐥> and ...));
  return ((expected == actual) or ...);
}

template <typename 𝒯₁, typename 𝒯₂, typename 𝒯₃>
𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑖𝑠_𝑖𝑛_𝑟𝑎𝑛𝑔𝑒(𝒯₁ value, 𝒯₂ minimum, 𝒯₃ maximum) noexcept -> 𝐛𝐨𝐨𝐥 {
  return minimum <= value and value < maximum;
}

}  // namespace 𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀

#endif  // 𝔜𝔄ℭ𝔈_𝔉𝔒𝔘𝔑𝔇𝔄𝔗ℑ𝔒𝔑_𝔖𝔄𝔑ℑ𝔗𝔜_ℭℌ𝔈ℭ𝔎𝔖_ℌ
