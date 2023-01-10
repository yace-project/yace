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

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮>(pub(crate) 𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮);

// It's very important to process simple label expressions efficiently.
//
// The most common one is of the form 𝓵𝓪𝓫𝓮𝓵1 - 𝓵𝓪𝓫𝓮𝓵2 and we want to process it efficiently even if there are millions of labels.
//
// I bit less common is 𝓵𝓪𝓫𝓮𝓵1 - 𝓵𝓪𝓫𝓮𝓵2 + 𝓭𝓲𝓼𝓹𝓵𝓪𝓬𝓮𝓶𝓮𝓷𝓽 and it must be proceeded efficiently, too.
//
// We try to efficiently handle calculations which may fit into 4 usize word, but after certain level of complexity data is pushed
// on heap and we stop caring about speed, just make sure it works.
//
// Note: we are checking target_pointer_width here, but we really need usize width. Change when/if Rust would give new check.
#[derive(Clone, Debug)]
#[repr(u8)]
pub enum 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
    // Valid expression needs to at least push something in the stack, then do some operations and then end with value.
    // Thus, technically, we may not have as many operations as 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢.
    // But it's easier to just define 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢 constants.
    𝔷𝔢𝔯𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 0,
    𝔬𝔫𝔢_𝔟𝔶𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 1,
    𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 2,
    𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 3,
    𝔣𝔬𝔲𝔯_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 4,
    𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 5,
    𝔰𝔦𝔵_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 6,
    𝔰𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 7,
    𝔢𝔦𝔤𝔥𝔱_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 8,
    𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 9,
    𝔱𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 10,
    𝔢𝔩𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 11,
    𝔱𝔴𝔢𝔩𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 12,
    𝔱𝔥𝔦𝔯𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 13,
    𝔣𝔬𝔲𝔯𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 14,
    𝔣𝔦𝔣𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 15,
    #[cfg(target_pointer_width = "64")]
    𝔰𝔦𝔵𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 16,
    #[cfg(target_pointer_width = "64")]
    𝔰𝔢𝔳𝔢𝔫𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 17,
    #[cfg(target_pointer_width = "64")]
    𝔢𝔦𝔤𝔥𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 18,
    #[cfg(target_pointer_width = "64")]
    𝔫𝔦𝔫𝔢𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 19,
    #[cfg(target_pointer_width = "64")]
    𝔱𝔴𝔢𝔫𝔱𝔶_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 20,
    #[cfg(target_pointer_width = "64")]
    𝔱𝔴𝔢𝔫𝔱𝔶_𝔬𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 21,
    #[cfg(target_pointer_width = "64")]
    𝔱𝔴𝔢𝔫𝔱𝔶_𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 22,
    #[cfg(target_pointer_width = "64")]
    𝔱𝔴𝔢𝔫𝔱𝔶_𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 23,
    #[cfg(target_pointer_width = "64")]
    𝔱𝔴𝔢𝔫𝔱𝔶_𝔣𝔬𝔲𝔯_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 24,
    #[cfg(target_pointer_width = "64")]
    𝔱𝔴𝔢𝔫𝔱𝔶_𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 25,
    #[cfg(target_pointer_width = "64")]
    𝔱𝔴𝔢𝔫𝔱𝔶_𝔰𝔦𝔵_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 26,
    #[cfg(target_pointer_width = "64")]
    𝔱𝔴𝔢𝔫𝔱𝔶_𝔰𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 27,
    #[cfg(target_pointer_width = "64")]
    𝔱𝔴𝔢𝔫𝔱𝔶_𝔢𝔦𝔤𝔥𝔱_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 28,
    #[cfg(target_pointer_width = "64")]
    𝔱𝔴𝔢𝔫𝔱𝔶_𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 29,
    #[cfg(target_pointer_width = "64")]
    𝔱𝔥𝔦𝔯𝔱𝔶_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 30,
    #[cfg(target_pointer_width = "64")]
    𝔱𝔥𝔦𝔯𝔱𝔶_𝔬𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢]) = 31,
    #[cfg(target_pointer_width = "64")]
    𝔥𝔢𝔞𝔭_𝔟𝔞𝔰𝔢𝔡_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(Vec<u8>) = 32,
}

impl Default for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
    fn default() -> 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
        𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔷𝔢𝔯𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫([0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢])
    }
}

#[cfg(target_pointer_width = "32")]
#[allow(non_upper_case_globals)]
const 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢: usize = 15;
#[cfg(target_pointer_width = "64")]
#[allow(non_upper_case_globals)]
const 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢: usize = 31;

macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖔𝖕𝖊𝖗𝖆𝖙𝖎𝖔𝖓𝖘 {
    ($𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident $𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident $𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮:ident $($𝓲𝓷𝓽_𝓽𝔂𝓹𝓮:ident)*) => {
        $(
            impl core::ops::$𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> for $𝓲𝓷𝓽_𝓽𝔂𝓹𝓮 {
                type Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧;
                #[inline(always)]
                fn $𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(self, rhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧) -> 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
                    let lhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 = self.into();
                    lhs.extend_by_expression(𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::$𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮, rhs)
                }
            }

            impl core::ops::$𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<$𝓲𝓷𝓽_𝓽𝔂𝓹𝓮> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
                type Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧;
                #[inline(always)]
                fn $𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(self, rhs: $𝓲𝓷𝓽_𝓽𝔂𝓹𝓮) -> 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
                    let rhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 = rhs.into();
                    self.extend_by_expression(𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::$𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮, rhs)
                }
            }

            impl<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮: Clone + TryInto<u8> + TryInto<u16> + TryInto<u32> + TryInto<u64>>
                core::ops::$𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮>> for $𝓲𝓷𝓽_𝓽𝔂𝓹𝓮 {
                type Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧;
                #[inline(always)]
                fn $𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(self, rhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮>) -> 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
                    let lhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 = self.into();
                    let rhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 = rhs.into();
                    lhs.extend_by_expression(𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::$𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮, rhs)
                }
            }

            impl<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮: Clone + TryInto<u8> + TryInto<u16> + TryInto<u32> + TryInto<u64>>
                core::ops::$𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<$𝓲𝓷𝓽_𝓽𝔂𝓹𝓮> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮> {
                type Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧;
                #[inline(always)]
                fn $𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(self, rhs: $𝓲𝓷𝓽_𝓽𝔂𝓹𝓮) -> 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
                    let lhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 = self.into();
                    let rhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 = rhs.into();
                    lhs.extend_by_expression(𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::$𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮, rhs)
                }
            }
         )*
    };
    ($($𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident/$𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident => $𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮:ident),*) => {
        $(
            impl core::ops::$𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
                type Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧;
                #[inline(always)]
                fn $𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(self, rhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧) -> 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
                    self.extend_by_expression(𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::$𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮, rhs)
                }
            }

            impl<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮: Clone + TryInto<u8> + TryInto<u16> + TryInto<u32> + TryInto<u64>>
                core::ops::$𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮>
            {
                type Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧;
                #[inline(always)]
                fn $𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(self, rhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧) -> 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
                    let lhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 = self.into();
                    lhs.extend_by_expression(𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::$𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮, rhs)
                }
            }

            impl<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮: Clone + TryInto<u8> + TryInto<u16> + TryInto<u32> + TryInto<u64>>
                core::ops::$𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮>> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
            {
                type Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧;
                #[inline(always)]
                fn $𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(self, rhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮>) -> 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
                    let rhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 = rhs.into();
                    self.extend_by_expression(𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::$𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮, rhs)
                }
            }

            impl<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮: Clone + TryInto<u8> + TryInto<u16> + TryInto<u32> + TryInto<u64>>
                core::ops::$𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮>> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮>
            {
                type Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧;
                #[inline(always)]
                fn $𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(self, rhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮>) -> 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
                    let lhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 = self.into();
                    let rhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 = rhs.into();
                    lhs.extend_by_expression(𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::$𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮, rhs)
                }
            }

            𝖉𝖊𝖋𝖎𝖓𝖊_𝖔𝖕𝖊𝖗𝖆𝖙𝖎𝖔𝖓𝖘!($𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮 $𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮 $𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮 i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize);
         )*
    }
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖔𝖕𝖊𝖗𝖆𝖙𝖎𝖔𝖓𝖘!{
    Add/add => 𝔞𝔡𝔡_𝔬𝔭,
    Sub/sub => 𝔰𝔲𝔟_𝔬𝔭,
    Mul/mul => 𝔪𝔲𝔩_𝔬𝔭,
    Div/div => 𝔡𝔦𝔳_𝔬𝔭,
    Rem/rem => 𝔯𝔢𝔪_𝔬𝔭,
    BitAnd/bitand => 𝔞𝔫𝔡_𝔬𝔭,
    BitOr/bitor => 𝔬𝔯_𝔬𝔭,
    BitXor/bitxor => 𝔵𝔬𝔯_𝔬𝔭,
    Shl/shl => 𝔰𝔥𝔩_𝔬𝔭,
    Shr/shr => 𝔰𝔥𝔯_𝔬𝔭
}

impl<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮: Clone + TryInto<u8> + TryInto<u16> + TryInto<u32> + TryInto<u64>>
    core::ops::Neg for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮>
{
    type Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧;
    #[inline(always)]
    fn neg(self) -> 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
        let lhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 = self.into();
        lhs.extend_by_array([𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔫𝔢𝔤_𝔬𝔭 as u8])
    }
}
   
impl<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮: Clone + TryInto<u8> + TryInto<u16> + TryInto<u32> + TryInto<u64>>
    core::ops::Not for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮>
{
    type Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧;
    #[inline(always)]
    fn not(self) -> 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
        let lhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 = self.into();
        lhs.extend_by_array([𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔫𝔬𝔱_𝔬𝔭 as u8])
    }
}

impl core::ops::Neg for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
    type Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧;
    #[inline(always)]
    fn neg(self) -> 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
        self.extend_by_array([𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔫𝔢𝔤_𝔬𝔭 as u8])
    }
}
   
impl core::ops::Not for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
    type Output = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧;
    #[inline(always)]
    fn not(self) -> 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
        self.extend_by_array([𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔫𝔬𝔱_𝔬𝔭 as u8])
    }
}

impl 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧 {
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    pub(crate) fn extend_by_array<const 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮: usize>(mut self, 𝖺𝗋𝗋𝖺𝗒: [u8; 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮]) -> Self {
        let size;
        let buffer: &mut [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
        match self {
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔷𝔢𝔯𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (0, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔬𝔫𝔢_𝔟𝔶𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (1, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (2, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (3, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔬𝔲𝔯_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (4, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (5, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔦𝔵_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (6, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (7, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔢𝔦𝔤𝔥𝔱_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (8, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (9, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (10, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔢𝔩𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (11, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔩𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (12, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔦𝔯𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (13, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔬𝔲𝔯𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (14, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔣𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (15, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔦𝔵𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (16, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔢𝔳𝔢𝔫𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (17, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔢𝔦𝔤𝔥𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (18, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (19, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (20, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔬𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (21, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (22, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (23, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔣𝔬𝔲𝔯_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (24, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (25, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔰𝔦𝔵_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (26, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔰𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (27, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔢𝔦𝔤𝔥𝔱_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (28, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (29, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔦𝔯𝔱𝔶_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (30, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔦𝔯𝔱𝔶_𝔬𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (size, buffer) = (31, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔥𝔢𝔞𝔭_𝔟𝔞𝔰𝔢𝔡_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(mut vec) => {
                vec.extend_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                return 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔥𝔢𝔞𝔭_𝔟𝔞𝔰𝔢𝔡_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(vec);
            }
        }
        match size + 𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮 {
            0 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔷𝔢𝔯𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            1 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔬𝔫𝔢_𝔟𝔶𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            2 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            3 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            4 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔬𝔲𝔯_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            5 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            6 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔦𝔵_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            7 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            8 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔢𝔦𝔤𝔥𝔱_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            9 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            10 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            11 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔢𝔩𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            12 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔩𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            13 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔦𝔯𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            14 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔬𝔲𝔯𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            15 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔣𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            #[cfg(target_pointer_width = "64")]
            16 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔦𝔵𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            #[cfg(target_pointer_width = "64")]
            17 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔢𝔳𝔢𝔫𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            #[cfg(target_pointer_width = "64")]
            18 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔢𝔦𝔤𝔥𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            #[cfg(target_pointer_width = "64")]
            19 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            #[cfg(target_pointer_width = "64")]
            20 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            #[cfg(target_pointer_width = "64")]
            21 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔬𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            #[cfg(target_pointer_width = "64")]
            22 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            #[cfg(target_pointer_width = "64")]
            23 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            #[cfg(target_pointer_width = "64")]
            24 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔣𝔬𝔲𝔯_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            #[cfg(target_pointer_width = "64")]
            25 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            #[cfg(target_pointer_width = "64")]
            26 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔰𝔦𝔵_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            #[cfg(target_pointer_width = "64")]
            27 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔰𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            #[cfg(target_pointer_width = "64")]
            28 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔢𝔦𝔤𝔥𝔱_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            #[cfg(target_pointer_width = "64")]
            29 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            #[cfg(target_pointer_width = "64")]
            30 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔦𝔯𝔱𝔶_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            #[cfg(target_pointer_width = "64")]
            31 => { buffer[size..size+𝓪𝓻𝓻𝓪𝔂_𝓼𝓲𝔃𝓮].copy_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔦𝔯𝔱𝔶_𝔬𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*buffer)},
            _ => { let mut vec = buffer[0..size].to_vec();
                   vec.extend_from_slice(&𝖺𝗋𝗋𝖺𝗒[..]);
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔥𝔢𝔞𝔭_𝔟𝔞𝔰𝔢𝔡_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(vec)},
        }
    }
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    pub(crate) fn extend_by_expression(
        mut self, op: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬, rhs: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
    ) -> Self {
        let rhs_buffer: &[u8];
        match rhs {
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔷𝔢𝔯𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..0],
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔬𝔫𝔢_𝔟𝔶𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..1],
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..2],
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..3],
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔬𝔲𝔯_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..4],
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..5],
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔦𝔵_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..6],
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..7],
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔢𝔦𝔤𝔥𝔱_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..8],
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..9],
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..10],
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔢𝔩𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..11],
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔩𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..12],
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔦𝔯𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..13],
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔬𝔲𝔯𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..14],
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔣𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..15],
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔦𝔵𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..16],
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔢𝔳𝔢𝔫𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..17],
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔢𝔦𝔤𝔥𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..18],
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..19],
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..20],
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔬𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..21],
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..22],
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..23],
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔣𝔬𝔲𝔯_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..24],
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..25],
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔰𝔦𝔵_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..26],
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔰𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..27],
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔢𝔦𝔤𝔥𝔱_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..28],
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..29],
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔦𝔯𝔱𝔶_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..30],
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔦𝔯𝔱𝔶_𝔬𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref source_buffer) => rhs_buffer = &source_buffer[0..31],
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔥𝔢𝔞𝔭_𝔟𝔞𝔰𝔢𝔡_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref vec) => rhs_buffer = &vec,
        }
        let lhs_size;
        let lhs_buffer: &mut [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
        match self {
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔷𝔢𝔯𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (0, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔬𝔫𝔢_𝔟𝔶𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (1, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (2, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (3, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔬𝔲𝔯_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (4, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (5, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔦𝔵_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (6, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (7, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔢𝔦𝔤𝔥𝔱_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (8, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (9, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (10, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔢𝔩𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (11, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔩𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (12, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔦𝔯𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (13, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔬𝔲𝔯𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (14, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔣𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (15, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔦𝔵𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (16, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔢𝔳𝔢𝔫𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (17, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔢𝔦𝔤𝔥𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (18, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (19, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (20, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔬𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (21, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (22, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (23, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔣𝔬𝔲𝔯_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (24, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (25, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔰𝔦𝔵_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (26, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔰𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (27, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔢𝔦𝔤𝔥𝔱_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (28, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (29, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔦𝔯𝔱𝔶_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (30, source_buffer),
            #[cfg(target_pointer_width = "64")]
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔦𝔯𝔱𝔶_𝔬𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(ref mut source_buffer) => (lhs_size, lhs_buffer) = (31, source_buffer),
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔥𝔢𝔞𝔭_𝔟𝔞𝔰𝔢𝔡_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(mut vec) => {
                vec.extend_from_slice(rhs_buffer);
                vec.push(op as u8);
                return 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔥𝔢𝔞𝔭_𝔟𝔞𝔰𝔢𝔡_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(vec);
            }
        }
        match lhs_size + rhs_buffer.len() + 1 {
            1 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                   lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔬𝔫𝔢_𝔟𝔶𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            2 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                   lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            3 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                   lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            4 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                   lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔬𝔲𝔯_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            5 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                   lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            6 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                   lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔦𝔵_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            7 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                   lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            8 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                   lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔢𝔦𝔤𝔥𝔱_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            9 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                   lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            10 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            11 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔢𝔩𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            12 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔩𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            13 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔦𝔯𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            14 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔬𝔲𝔯𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            15 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔣𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            #[cfg(target_pointer_width = "64")]
            16 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔦𝔵𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            #[cfg(target_pointer_width = "64")]
            17 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔢𝔳𝔢𝔫𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            #[cfg(target_pointer_width = "64")]
            18 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔢𝔦𝔤𝔥𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            #[cfg(target_pointer_width = "64")]
            19 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            #[cfg(target_pointer_width = "64")]
            20 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            #[cfg(target_pointer_width = "64")]
            21 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔬𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            #[cfg(target_pointer_width = "64")]
            22 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            #[cfg(target_pointer_width = "64")]
            23 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            #[cfg(target_pointer_width = "64")]
            24 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔣𝔬𝔲𝔯_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            #[cfg(target_pointer_width = "64")]
            25 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            #[cfg(target_pointer_width = "64")]
            26 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔰𝔦𝔵_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            #[cfg(target_pointer_width = "64")]
            27 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔰𝔢𝔳𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            #[cfg(target_pointer_width = "64")]
            28 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔢𝔦𝔤𝔥𝔱_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            #[cfg(target_pointer_width = "64")]
            29 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔢𝔫𝔱𝔶_𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            #[cfg(target_pointer_width = "64")]
            30 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔦𝔯𝔱𝔶_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            #[cfg(target_pointer_width = "64")]
            31 => { lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].copy_from_slice(rhs_buffer);
                    lhs_buffer[lhs_size+rhs_buffer.len()] = op as u8;
                    𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔦𝔯𝔱𝔶_𝔬𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(*lhs_buffer)},
            _ => { let mut vec = lhs_buffer[lhs_size..lhs_size+rhs_buffer.len()].to_vec();
                   vec.extend_from_slice(rhs_buffer);
                   vec.push(op as u8);
                   𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔥𝔢𝔞𝔭_𝔟𝔞𝔰𝔢𝔡_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(vec)},
        }
    }
}

impl<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮: Clone + TryInto<u8> + TryInto<u16> + TryInto<u32> + TryInto<u64>>
    From<𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮>> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
{
    #[inline(always)]
    fn from(lbl: 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥<𝓹𝓸𝓼𝓲𝓽𝓲𝓸𝓷_𝓽𝔂𝓹𝓮>) -> Self {
        if let Ok(value) = TryInto::<u8>::try_into(lbl.0.clone()) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔩𝔞𝔟𝔢𝔩_𝔲8 as u8;
            buffer[1] = value;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u16>::try_into(lbl.0.clone()) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔩𝔞𝔟𝔢𝔩_𝔲16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u32>::try_into(lbl.0.clone()) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔩𝔞𝔟𝔢𝔩_𝔲32 as u8;
            buffer[1..4].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u64>::try_into(lbl.0) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔩𝔞𝔟𝔢𝔩_𝔲64 as u8;
            buffer[1..8].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else {
            panic!("label number doesn't fit into u64!");
        }
    }
}

impl From<i8> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
{
    #[inline(always)]
    fn from(value: i8) -> Self {
        if let Ok(value) = TryInto::<u8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲8 as u8;
            buffer[1] = value;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦8 as u8;
            buffer[1] = value as u8;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        }
    }
}

impl From<u8> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
{
    #[inline(always)]
    fn from(value: u8) -> Self {
        let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
        buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲8 as u8;
        buffer[1] = value;
        𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
    }
}

impl From<i16> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
{
    #[inline(always)]
    fn from(value: i16) -> Self {
        if let Ok(value) = TryInto::<u8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲8 as u8;
            buffer[1] = value;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦8 as u8;
            buffer[1] = value as u8;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u16>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        }
    }
}

impl From<u16> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
{
    #[inline(always)]
    fn from(value: u16) -> Self {
        if let Ok(value) = TryInto::<u8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲8 as u8;
            buffer[1] = value;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        }
    }
}

impl From<i32> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
{
    #[inline(always)]
    fn from(value: i32) -> Self {
        if let Ok(value) = TryInto::<u8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲8 as u8;
            buffer[1] = value;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦8 as u8;
            buffer[1] = value as u8;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u16>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i16>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u32>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲32 as u8;
            buffer[1..4].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦32 as u8;
            buffer[1..4].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        }
    }
}

impl From<u32> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
{
    #[inline(always)]
    fn from(value: u32) -> Self {
        if let Ok(value) = TryInto::<u8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲8 as u8;
            buffer[1] = value;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u16>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲32 as u8;
            buffer[1..4].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        }
    }
}

#[cfg(target_pointer_width = "32")]
impl From<isize> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
{
    #[inline(always)]
    fn from(value: isize) -> Self {
        if let Ok(value) = TryInto::<u8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲8 as u8;
            buffer[1] = value;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦8 as u8;
            buffer[1] = value as u8;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u16>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i16>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u32>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲32 as u8;
            buffer[1..4].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i32>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦32 as u8;
            buffer[1..4].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else {
            panic!("isize doesn't fit into i32 on 32-bit platform!");
        }
    }
}

#[cfg(target_pointer_width = "32")]
impl From<usize> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
{
    #[inline(always)]
    fn from(value: usize) -> Self {
        if let Ok(value) = TryInto::<u8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲8 as u8;
            buffer[1] = value;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u16>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u32>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲32 as u8;
            buffer[1..4].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else {
            panic!("usize doesn't fit into u32 on 32-bit platform!");
        }
    }
}

impl From<i64> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
{
    #[inline(always)]
    fn from(value: i64) -> Self {
        if let Ok(value) = TryInto::<u8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲8 as u8;
            buffer[1] = value;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦8 as u8;
            buffer[1] = value as u8;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u16>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i16>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u32>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲32 as u8;
            buffer[1..4].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i32>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦32 as u8;
            buffer[1..4].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u64>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲64 as u8;
            buffer[1..8].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦64 as u8;
            buffer[1..8].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        }
    }
}

impl From<u64> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
{
    #[inline(always)]
    fn from(value: u64) -> Self {
        if let Ok(value) = TryInto::<u8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲8 as u8;
            buffer[1] = value;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u16>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u32>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲32 as u8;
            buffer[1..4].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲64 as u8;
            buffer[1..8].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        }
    }
}

#[cfg(target_pointer_width = "64")]
impl From<isize> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
{
    #[inline(always)]
    fn from(value: isize) -> Self {
        if let Ok(value) = TryInto::<u8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲8 as u8;
            buffer[1] = value;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦8 as u8;
            buffer[1] = value as u8;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u16>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i16>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u32>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲32 as u8;
            buffer[1..4].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i32>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦32 as u8;
            buffer[1..4].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u64>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲64 as u8;
            buffer[1..8].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i64>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦64 as u8;
            buffer[1..8].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else {
            panic!("isize doesn't fit into i64 on 64-bit platform!");
        }
    }
}

#[cfg(target_pointer_width = "64")]
impl From<usize> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
{
    #[inline(always)]
    fn from(value: usize) -> Self {
        if let Ok(value) = TryInto::<u8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲8 as u8;
            buffer[1] = value;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u16>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u32>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲32 as u8;
            buffer[1..4].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u64>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲64 as u8;
            buffer[1..8].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else {
            panic!("usize doesn't fit into u64 on 64-bit platform!");
        }
    }
}

impl From<i128> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
{
    #[inline(always)]
    fn from(value: i128) -> Self {
        if let Ok(value) = TryInto::<u8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲8 as u8;
            buffer[1] = value;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦8 as u8;
            buffer[1] = value as u8;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u16>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i16>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u32>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲32 as u8;
            buffer[1..4].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i32>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦32 as u8;
            buffer[1..4].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u64>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲64 as u8;
            buffer[1..8].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<i64>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦64 as u8;
            buffer[1..8].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u128>::try_into(value) {
            #[cfg(not(target_pointer_width = "64"))]
            let mut buffer: [u8; 17] = [0; 17];
            #[cfg(target_pointer_width = "64")]
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲128 as u8;
            buffer[1..16].copy_from_slice(&value.to_ne_bytes());
            #[cfg(not(target_pointer_width = "64"))]
            return 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔥𝔢𝔞𝔭_𝔟𝔞𝔰𝔢𝔡_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer.to_vec());
            #[cfg(target_pointer_width = "64")]
            return 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔢𝔳𝔢𝔫𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer);
        } else {
            #[cfg(not(target_pointer_width = "64"))]
            let mut buffer: [u8; 17] = [0; 17];
            #[cfg(target_pointer_width = "64")]
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦128 as u8;
            buffer[1..16].copy_from_slice(&value.to_ne_bytes());
            #[cfg(not(target_pointer_width = "64"))]
            return 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔥𝔢𝔞𝔭_𝔟𝔞𝔰𝔢𝔡_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer.to_vec());
            #[cfg(target_pointer_width = "64")]
            return 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔢𝔳𝔢𝔫𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer);
        }
    }
}

impl From<u128> for 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧
{
    #[inline(always)]
    fn from(value: u128) -> Self {
        if let Ok(value) = TryInto::<u8>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲8 as u8;
            buffer[1] = value;
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔴𝔬_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u16>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲16 as u8;
            buffer[1..2].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔱𝔥𝔯𝔢𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u32>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲32 as u8;
            buffer[1..4].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔣𝔦𝔳𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else if let Ok(value) = TryInto::<u64>::try_into(value) {
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲64 as u8;
            buffer[1..8].copy_from_slice(&value.to_ne_bytes());
            𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔫𝔦𝔫𝔢_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer)
        } else {
            #[cfg(not(target_pointer_width = "64"))]
            let mut buffer: [u8; 17] = [0; 17];
            #[cfg(target_pointer_width = "64")]
            let mut buffer: [u8; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢] = [0; 𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫_𝔭𝔞𝔶𝔩𝔬𝔞𝔡_𝔰𝔦𝔷𝔢];
            buffer[0] = 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬::𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲128 as u8;
            buffer[1..16].copy_from_slice(&value.to_ne_bytes());
            #[cfg(not(target_pointer_width = "64"))]
            return 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔥𝔢𝔞𝔭_𝔟𝔞𝔰𝔢𝔡_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer.to_vec());
            #[cfg(target_pointer_width = "64")]
            return 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧::𝔰𝔢𝔳𝔢𝔫𝔱𝔢𝔢𝔫_𝔟𝔶𝔱𝔢𝔰_𝔬𝔭𝔢𝔯𝔞𝔱𝔦𝔬𝔫(buffer);
        }
    }
}

// Note: we can not make it enum-with-payload because it would turn 8ᵇⁱᵗ tag into 128ᵇⁱᵗ monster because alignment.
// Thus we keep tags and operations separated.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub(crate) enum 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬 {
    _𝔢𝔪𝔦𝔱_𝔯𝔢𝔰𝔲𝔩𝔱 = 0, // Not used in 𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐥𝐚𝐛𝐞𝐥_𝐞𝐱𝐩𝐫𝐞𝐬𝐬𝐢𝐨𝐧_𝐨𝐩𝐞𝐫𝐚𝐭𝐢𝐨𝐧𝐬 itself, added when moved to 𝐧𝐚𝐭𝐢𝐯𝐞_𝐦𝐚𝐜𝐡𝐢𝐧𝐞_𝐜𝐨𝐝𝐞 pools.
    𝔩𝔞𝔟𝔢𝔩_𝔲8 = 1,
    𝔩𝔞𝔟𝔢𝔩_𝔲16 = 2,
    𝔩𝔞𝔟𝔢𝔩_𝔲32 = 3,
    𝔩𝔞𝔟𝔢𝔩_𝔲64 = 4,
    𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲8 = 5,
    𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦8 = 6,
    𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲16 = 7,
    𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦16 = 8,
    𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲32 = 9,
    𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦32 = 10,
    𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲64 = 11,
    𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦64 = 12,
    𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔲128 = 13,
    𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱_𝔦128 = 14,
    _𝔩𝔞𝔟𝔢𝔩_𝔢𝔵𝔭𝔯𝔢𝔰𝔰𝔦𝔬𝔫 = 15,
    𝔞𝔡𝔡_𝔬𝔭 = 16,
    𝔰𝔲𝔟_𝔬𝔭 = 17,
    𝔪𝔲𝔩_𝔬𝔭 = 18,
    𝔡𝔦𝔳_𝔬𝔭 = 19,
    𝔯𝔢𝔪_𝔬𝔭 = 20,
    𝔞𝔫𝔡_𝔬𝔭 = 21,
    𝔬𝔯_𝔬𝔭 = 22,
    𝔵𝔬𝔯_𝔬𝔭 = 23,
    𝔰𝔥𝔩_𝔬𝔭 = 24,
    𝔰𝔥𝔯_𝔬𝔭 = 25,
    𝔫𝔢𝔤_𝔬𝔭 = 26,
    𝔫𝔬𝔱_𝔬𝔭 = 27,
}
