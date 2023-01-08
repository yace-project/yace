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

use yace_codegen::𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘;

𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖎𝖘𝖈𝖛_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖙𝖗𝖆𝖎𝖙! {
    [𝔯𝔳64𝔦]
    pub trait 𝑪𝑷𝑼 {
    }

    [𝔯𝔳64𝔦 𝔢𝔞𝔟𝔦]
    pub trait 𝑪𝑷𝑼_𝔢𝔞𝔟𝔦 {
    }
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖎𝖘𝖈𝖛_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖙𝖗𝖚𝖈𝖙! {
    [𝔯𝔳64𝔦]
    pub struct 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿_𝗴𝗲𝗻𝗲𝗿𝗶𝗰[<𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝑪𝑷𝑼 {
    }

    [𝔯𝔳64𝔦 𝔢𝔞𝔟𝔦]
    impl[<𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] 𝑪𝑷𝑼_𝔢𝔞𝔟𝔦
    for 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿_𝗴𝗲𝗻𝗲𝗿𝗶𝗰[<𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>];
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖎𝖘𝖈𝖛_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
    [𝔯𝔳64𝔦]
    impl[<𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿_𝗴𝗲𝗻𝗲𝗿𝗶𝗰
    as 𝑪𝑷𝑼;

    [𝔯𝔳64𝔦 𝔢𝔞𝔟𝔦]
    impl[<𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿_𝗴𝗲𝗻𝗲𝗿𝗶𝗰
    as 𝑪𝑷𝑼_𝔢𝔞𝔟𝔦;
}

pub use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝔞𝔡𝔡𝔯𝔢𝔰𝔰;

pub type 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿_𝗴𝗲𝗻𝗲𝗿𝗶𝗰::<𝐝𝐮𝐦𝐦𝐲_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;

use super::𝗿𝗶𝘀𝗰_𝘃;

use super::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;
use super::𝒓𝒆𝒃𝒊𝒏𝒅_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;

use super::𝗲𝗺𝗶𝘁𝘁𝗲𝗿::𝐝𝐮𝐦𝐦𝐲_𝐞𝐦𝐢𝐭𝐭𝐞𝐫;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐫𝐯𝟔𝟒;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐟𝐞𝐧𝐜𝐞_𝐨𝐩𝐞𝐫𝐚𝐧𝐝;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒂𝒅𝒅𝒓𝒆𝒔𝒔;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒄𝒔𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒔𝒉𝒊𝒇𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒛𝒆𝒓𝒐_𝒐𝒇𝒇𝒔𝒆𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆;

use yace_codegen::𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖎𝖘𝖈𝖛_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓𝖘;
𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖎𝖘𝖈𝖛_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓𝖘!{𝗿𝘃𝟲𝟰𝗶}
