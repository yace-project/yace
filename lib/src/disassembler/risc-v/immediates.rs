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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct 𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧(pub i32);

impl From<𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧> for 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧(encoding): 𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧) -> 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞::new_from_encoded(encoding)
    }
}

impl From<𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧> for 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧(encoding): 𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧) -> 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞::new_from_encoded(encoding)
    }
}

impl From<𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧(encoding): 𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧) -> 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞::new_from_encoded(encoding)
    }
}

impl From<𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧> for 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧(encoding): 𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧) -> 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞::new_from_encoded(encoding)
    }
}

impl From<𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧> for 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧(encoding): 𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧) -> 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞::new_from_encoded(encoding)
    }
}

impl From<𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧(encoding): 𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧) -> 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞::new_from_encoded(encoding)
    }
}

impl From<𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧(encoding): 𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧) -> 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞::new_from_encoded(encoding)
    }
}

impl From<𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧(encoding): 𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧) -> 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞::new_from_encoded(encoding)
    }
}

impl From<𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧> for 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧(encoding): 𝐫𝐢𝐬𝐜_𝐯_𝟑𝟐𝐛𝐢𝐭_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧) -> 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞::new_from_encoded(encoding)
    }
}

use super::super::super::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::super::super::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::super::super::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::super::super::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::super::super::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::super::super::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::super::super::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::super::super::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::super::super::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝗶𝘀𝗰_𝘃::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
