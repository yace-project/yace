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

// Machine code for the native code executed in the address space of Rust program.
#[derive(Clone, Debug, Default)]
pub struct 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞 {
    𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄: 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤,
    // New label starts equal to 0, after set_label or set_data_label is called it becomes equal to !0usize and it receives actual
    // address in memory when “freeze” is called.
    𝗅𝖺𝖻𝖾𝗅𝗌: Vec<usize>,
}

impl 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆 for 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞 {
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬;
    type 𝐥𝐚𝐛𝐞𝐥 = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭>;
    type 𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧;
    type 𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭 = isize;
    type 𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤 = 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤;
    type 𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾> = 𝐧𝐚𝐭𝐢𝐯𝐞_𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾>
         where Self: 'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ;

    fn new_label(&mut self) -> Self::𝐥𝐚𝐛𝐞𝐥 {
        let lbl = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥::<Self::𝐝𝐢𝐬𝐩𝐥𝐚𝐜𝐞𝐦𝐞𝐧𝐭>(self.𝗅𝖺𝖻𝖾𝗅𝗌.len() as isize);
        self.𝗅𝖺𝖻𝖾𝗅𝗌.push(0);
        lbl
    }
    fn set_label(&mut self, lbl: Self::𝐥𝐚𝐛𝐞𝐥) -> Result<(), Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let label_id = lbl.0 as usize;
        if self.𝗅𝖺𝖻𝖾𝗅𝗌.len() <= label_id {
            Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔦𝔡_𝔬𝔲𝔱_𝔬𝔣_𝔯𝔞𝔫𝔤𝔢)
        } else if self.𝗅𝖺𝖻𝖾𝗅𝗌[label_id] != 0 {
            Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔞𝔩𝔯𝔢𝔞𝔡𝔶_𝔞𝔱𝔱𝔞𝔠𝔥𝔢𝔡)
        } else {
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.close_bytes_fragment();
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖼𝗈𝖽𝖾.extend_from_slice(&0usize.to_ne_bytes());
            self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖼𝗈𝖽𝖾.extend_from_slice(&label_id.to_ne_bytes());
            self.𝗅𝖺𝖻𝖾𝗅𝗌[label_id] = !0usize;
            Ok(())
        }
    }

    fn new_code_block(&mut self) -> Self::𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤 {
        Default::default()
    }
    fn with_code_block<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾>(self: &'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ mut Self,
                                                       𝖻𝗅𝗈𝖼𝗄: &'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾 mut Self::𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤)
        -> Self::𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾>
    {
        𝐧𝐚𝐭𝐢𝐯𝐞_𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 { 𝖼𝗈𝖽𝖾: self, 𝖻𝗅𝗈𝖼𝗄 }
    }
    fn attach_code_block_contents(&mut self, 𝖻𝗅𝗈𝖼𝗄: &mut Self::𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤) -> Result<(),Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.close_bytes_fragment();
        𝖻𝗅𝗈𝖼𝗄.close_bytes_fragment();
        self.𝗆𝖺𝗂𝗇_𝖻𝗅𝗈𝖼𝗄.𝖼𝗈𝖽𝖾.extend_from_slice(&𝖻𝗅𝗈𝖼𝗄.𝖼𝗈𝖽𝖾);
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤 {
    // Vector contains collected assembler code. Code is collected in chunks.
    //
    // First size of chunk in ne_chunk format is specified then sequence of instructions follow.
    // Last chunk is usually not “closed”, it's length is kept in a 𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾.
    //
    // If instruction which uses label operands is stored then top bit is set, low byte includes number of labels to be calculated
    // and provided for the emitter function, next byte includes size of data needed besides labels, after that pointer of function
    // to call follows. 
    //
    // Size of chunk equal to zero (if chunk is closed) means that next usize bytes inlcude label number to set.
    𝖼𝗈𝖽𝖾: Vec<u8>,
    // Note: initial value is zero which means that there no unclodes code chunk yet.
    𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾: usize,
}

impl 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤 {
    fn close_bytes_fragment(&mut self) {
        if self.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 != 0 {
             let size_position = self.𝖼𝗈𝖽𝖾.len() - self.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 - core::mem::size_of::<isize>();
             if size_position > self.𝖼𝗈𝖽𝖾.len() {
                 // SAFETY: 𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 and grows with vector, overflow is impossible because Vec::push would panic.
                 unsafe { core::hint::unreachable_unchecked() };
             }
             self.𝖼𝗈𝖽𝖾[size_position..size_position+core::mem::size_of::<isize>()]
                 .copy_from_slice(&self.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾.to_ne_bytes());
             self.𝗅𝖺𝗌𝗍_𝖼𝗈𝖽𝖾_𝖼𝗁𝗎𝗇𝗄_𝗌𝗂𝗓𝖾 = 0;
        }
    }
}

#[derive(Debug)]
pub struct 𝐧𝐚𝐭𝐢𝐯𝐞_𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾> {
    𝖼𝗈𝖽𝖾: &'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ mut 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞,
    𝖻𝗅𝗈𝖼𝗄: &'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾 mut 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐛𝐥𝐨𝐜𝐤,
}

impl<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾> 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐧𝐚𝐭𝐢𝐯𝐞_𝐜𝐨𝐝𝐞_𝐞𝐦𝐢𝐭𝐭𝐞𝐫<'ᵐᵃᶜʰⁱⁿᵉ_ᶜᵒᵈᵉ, 'ᵇˡᵒᶜᵏ_𝗅𝗂𝖿𝖾𝗍𝗂𝗆𝖾> {
    type 𝐜𝐨𝐝𝐞 = 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞;

    fn new_label(&mut self) -> <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞 as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥 {
        self.𝖼𝗈𝖽𝖾.new_label() 
    }
    fn set_label(&mut self, lbl: <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞 as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐥𝐚𝐛𝐞𝐥)
        -> Result<(), <𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞 as 𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
    {
        let label_id = lbl.0 as usize;
        if self.𝖼𝗈𝖽𝖾.𝗅𝖺𝖻𝖾𝗅𝗌.len() <= label_id {
            Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔦𝔡_𝔬𝔲𝔱_𝔬𝔣_𝔯𝔞𝔫𝔤𝔢)
        } else if self.𝖼𝗈𝖽𝖾.𝗅𝖺𝖻𝖾𝗅𝗌[label_id] != 0 {
            Err(𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬::𝔩𝔞𝔟𝔢𝔩_𝔞𝔩𝔯𝔢𝔞𝔡𝔶_𝔞𝔱𝔱𝔞𝔠𝔥𝔢𝔡)
        } else {
            self.𝖻𝗅𝗈𝖼𝗄.close_bytes_fragment();
            self.𝖻𝗅𝗈𝖼𝗄.𝖼𝗈𝖽𝖾.extend_from_slice(&0usize.to_ne_bytes());
            self.𝖻𝗅𝗈𝖼𝗄.𝖼𝗈𝖽𝖾.extend_from_slice(&label_id.to_ne_bytes());
            self.𝖼𝗈𝖽𝖾.𝗅𝖺𝖻𝖾𝗅𝗌[label_id] = !0usize;
            Ok(())
        }
    }
}

#[non_exhaustive]
pub enum 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞_𝐞𝐫𝐫𝐨𝐫𝐬 {
    𝔩𝔞𝔟𝔢𝔩_𝔦𝔡_𝔬𝔲𝔱_𝔬𝔣_𝔯𝔞𝔫𝔤𝔢,
    𝔩𝔞𝔟𝔢𝔩_𝔞𝔩𝔯𝔢𝔞𝔡𝔶_𝔞𝔱𝔱𝔞𝔠𝔥𝔢𝔡,
}

use super::𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆;
use super::𝒎𝒂𝒄𝒉𝒊𝒏𝒆_𝒄𝒐𝒅𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;

use super::𝗻𝘂𝗺𝗯𝗲𝗿𝗲𝗱_𝗹𝗮𝗯𝗲𝗹𝘀::𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥;
use super::𝗻𝘂𝗺𝗯𝗲𝗿𝗲𝗱_𝗹𝗮𝗯𝗲𝗹𝘀::𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧;
