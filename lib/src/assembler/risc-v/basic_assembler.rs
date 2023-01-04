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

// To ensure safety we provice separate types for different classes of registers.
// But Rust compiler currently is not advanced enough to produce e.g. effective conversion from 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 to
// 𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢: while you just need to copy value without doing anything compiler doesn't always optimize
// checks away.

// To make sure this woulnd't happen we are providing that conversion only for tests: 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜>
// would be converted to 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢> using safe code and matching values.

// The actual, production, conversion is compared to that one on the full range of 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜.

// This way we can guarantee that our tests are enough to ensure safety.
// Note: since mistakes here may trigger undefined behavior tests have to be run with “cargo +nightly miri test”.

𝖉𝖊𝖋𝖎𝖓𝖊_𝖊𝖓𝖚𝖒𝖘! {
    [{|value| !(1..=15).contains(&value)}]
    [𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {|value| value > 15},
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {|value| value > 15},
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {|value| value > 15},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {|value| value == 0 || value > 15},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {|value| value == 0 || value > 15},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {|value| value == 0 || value > 15}
    ]
    pub enum(i8, u8) 𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞 {
        𝔵1 = 1,
        𝔵2 = 2,
        𝔵3 = 3,
        𝔵4 = 4,
        𝔵5 = 5,
        𝔵6 = 6,
        𝔵7 = 7,
        𝔵8 = 8,
        𝔵9 = 9,
        𝔵10 = 10,
        𝔵11 = 11,
        𝔵12 = 12,
        𝔵13 = 13,
        𝔵14 = 14,
        𝔵15 = 15
    }

    [{|value| !(1..=31).contains(&value)}]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢
    ]
    [   𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {|value| value == 0}
    ]
    pub enum(i8, u8) 𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {
        𝔵1 = 1,
        𝔵2 = 2,
        𝔵3 = 3,
        𝔵4 = 4,
        𝔵5 = 5,
        𝔵6 = 6,
        𝔵7 = 7,
        𝔵8 = 8,
        𝔵9 = 9,
        𝔵10 = 10,
        𝔵11 = 11,
        𝔵12 = 12,
        𝔵13 = 13,
        𝔵14 = 14,
        𝔵15 = 15,
        𝔵16 = 16,
        𝔵17 = 17,
        𝔵18 = 18,
        𝔵19 = 19,
        𝔵20 = 20,
        𝔵21 = 21,
        𝔵22 = 22,
        𝔵23 = 23,
        𝔵24 = 24,
        𝔵25 = 25,
        𝔵26 = 26,
        𝔵27 = 27,
        𝔵28 = 28,
        𝔵29 = 29,
        𝔵30 = 30,
        𝔵31 = 31
    }

    [{|value| !(1..=15).contains(&value)}]
    [𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {|value| value > 15},
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {|value| value > 15},
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {|value| value > 15},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {|value| value == 0 || value > 15},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {|value| value == 0 || value > 15},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {|value| value == 0 || value > 15}
    ]
    pub enum(i8, u8) 𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞 {
        𝔯𝔞 = 1,
        𝔰𝔭 = 2,
        𝔤𝔭 = 3,
        𝔱𝔭 = 4,
        𝔱0 = 5,
        𝔰3 = 6,
        𝔰4 = 7,
        𝔰0 = 8,
        𝔰1 = 9,
        𝔞0 = 10,
        𝔞1 = 11,
        𝔞2 = 12,
        𝔞3 = 13,
        𝔰2 = 14,
        𝔱1 = 15
    }

    [{|value| !(1..=31).contains(&value)}]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢
    ]
    [   𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {|value| value == 0}
    ]
    pub enum(i8, u8) 𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {
        𝔯𝔞 = 1,
        𝔰𝔭 = 2,
        𝔤𝔭 = 3,
        𝔱𝔭 = 4,
        𝔱0 = 5,
        𝔰3 = 6,
        𝔰4 = 7,
        𝔰0 = 8,
        𝔰1 = 9,
        𝔞0 = 10,
        𝔞1 = 11,
        𝔞2 = 12,
        𝔞3 = 13,
        𝔰2 = 14,
        𝔱1 = 15,
        𝔰5 = 16,
        𝔰6 = 17,
        𝔰7 = 18,
        𝔰8 = 19,
        𝔰9 = 20,
        𝔰10 = 21,
        𝔰11 = 22,
        𝔰12 = 23,
        𝔰13 = 24,
        𝔰14 = 25,
        𝔰15 = 26,
        𝔰16 = 27,
        𝔰17 = 28,
        𝔰18 = 29,
        𝔰19 = 30,
        𝔰20 = 31
    }

    [{|value| !(1..=31).contains(&value)}]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢
    ]
    [   𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {|value| value == 0}
    ]
    pub enum(i8, u8) 𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {
        𝔯𝔞 = 1,
        𝔰𝔭 = 2,
        𝔤𝔭 = 3,
        𝔱𝔭 = 4,
        𝔱0 = 5,
        𝔱1 = 6,
        𝔱2 = 7,
        𝔰0 = 8,
        𝔰1 = 9,
        𝔞0 = 10,
        𝔞1 = 11,
        𝔞2 = 12,
        𝔞3 = 13,
        𝔞4 = 14,
        𝔞5 = 15,
        𝔞6 = 16,
        𝔞7 = 17,
        𝔰2 = 18,
        𝔰3 = 19,
        𝔰4 = 20,
        𝔰5 = 21,
        𝔰6 = 22,
        𝔰7 = 23,
        𝔰8 = 24,
        𝔰9 = 25,
        𝔰10 = 26,
        𝔰11 = 27,
        𝔱3 = 28,
        𝔱4 = 29,
        𝔱5 = 30,
        𝔱6 = 31
    }

    [{|value| !(0..=15).contains(&value)}]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞
    ]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {|value| value > 15},
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {|value| value > 15},
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {|value| value > 15},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {|value| value > 15},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {|value| value > 15},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {|value| value > 15}
    ]
    pub enum(i8, u8) 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞 {
        𝔵0 = 0,
        𝔵1 = 1,
        𝔵2 = 2,
        𝔵3 = 3,
        𝔵4 = 4,
        𝔵5 = 5,
        𝔵6 = 6,
        𝔵7 = 7,
        𝔵8 = 8,
        𝔵9 = 9,
        𝔵10 = 10,
        𝔵11 = 11,
        𝔵12 = 12,
        𝔵13 = 13,
        𝔵14 = 14,
        𝔵15 = 15
    }

    [{|value| !(0..=31).contains(&value)}]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢
    ]
    []
    pub enum(i8, u8) 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {
        𝔵0 = 0,
        𝔵1 = 1,
        𝔵2 = 2,
        𝔵3 = 3,
        𝔵4 = 4,
        𝔵5 = 5,
        𝔵6 = 6,
        𝔵7 = 7,
        𝔵8 = 8,
        𝔵9 = 9,
        𝔵10 = 10,
        𝔵11 = 11,
        𝔵12 = 12,
        𝔵13 = 13,
        𝔵14 = 14,
        𝔵15 = 15,
        𝔵16 = 16,
        𝔵17 = 17,
        𝔵18 = 18,
        𝔵19 = 19,
        𝔵20 = 20,
        𝔵21 = 21,
        𝔵22 = 22,
        𝔵23 = 23,
        𝔵24 = 24,
        𝔵25 = 25,
        𝔵26 = 26,
        𝔵27 = 27,
        𝔵28 = 28,
        𝔵29 = 29,
        𝔵30 = 30,
        𝔵31 = 31
    }

    [{|value| !(0..=15).contains(&value)}]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞
    ]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {|value| value > 15},
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {|value| value > 15},
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {|value| value > 15},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {|value| value > 15},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {|value| value > 15},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {|value| value > 15}
    ]
    pub enum(i8, u8) 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞 {
        𝔷𝔢𝔯𝔬 = 0,
        𝔯𝔞 = 1,
        𝔰𝔭 = 2,
        𝔤𝔭 = 3,
        𝔱𝔭 = 4,
        𝔱0 = 5,
        𝔰3 = 6,
        𝔰4 = 7,
        𝔰0 = 8,
        𝔰1 = 9,
        𝔞0 = 10,
        𝔞1 = 11,
        𝔞2 = 12,
        𝔞3 = 13,
        𝔰2 = 14,
        𝔱1 = 15
    }

    [{|value| !(0..=31).contains(&value)}]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢
    ]
    []
    pub enum(i8, u8) 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {
        𝔷𝔢𝔯𝔬 = 0,
        𝔯𝔞 = 1,
        𝔰𝔭 = 2,
        𝔤𝔭 = 3,
        𝔱𝔭 = 4,
        𝔱0 = 5,
        𝔰3 = 6,
        𝔰4 = 7,
        𝔰0 = 8,
        𝔰1 = 9,
        𝔞0 = 10,
        𝔞1 = 11,
        𝔞2 = 12,
        𝔞3 = 13,
        𝔰2 = 14,
        𝔱1 = 15,
        𝔰5 = 16,
        𝔰6 = 17,
        𝔰7 = 18,
        𝔰8 = 19,
        𝔰9 = 20,
        𝔰10 = 21,
        𝔰11 = 22,
        𝔰12 = 23,
        𝔰13 = 24,
        𝔰14 = 25,
        𝔰15 = 26,
        𝔰16 = 27,
        𝔰17 = 28,
        𝔰18 = 29,
        𝔰19 = 30,
        𝔰20 = 31
    }

    [{|value| !(0..=31).contains(&value)}]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢
    ]
    []
    pub enum(i8, u8) 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {
        𝔷𝔢𝔯𝔬 = 0,
        𝔯𝔞 = 1,
        𝔰𝔭 = 2,
        𝔤𝔭 = 3,
        𝔱𝔭 = 4,
        𝔱0 = 5,
        𝔱1 = 6,
        𝔱2 = 7,
        𝔰0 = 8,
        𝔰1 = 9,
        𝔞0 = 10,
        𝔞1 = 11,
        𝔞2 = 12,
        𝔞3 = 13,
        𝔞4 = 14,
        𝔞5 = 15,
        𝔞6 = 16,
        𝔞7 = 17,
        𝔰2 = 18,
        𝔰3 = 19,
        𝔰4 = 20,
        𝔰5 = 21,
        𝔰6 = 22,
        𝔰7 = 23,
        𝔰8 = 24,
        𝔰9 = 25,
        𝔰10 = 26,
        𝔰11 = 27,
        𝔱3 = 28,
        𝔱4 = 29,
        𝔱5 = 30,
        𝔱6 = 31
    }

    [{|value| !(0..=31).contains(&value)}]
    [𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐚𝐛𝐢]
    []
    pub enum(i8, u8) 𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {
        𝔣0 = 0,
        𝔣1 = 1,
        𝔣2 = 2,
        𝔣3 = 3,
        𝔣4 = 4,
        𝔣5 = 5,
        𝔣6 = 6,
        𝔣7 = 7,
        𝔣8 = 8,
        𝔣9 = 9,
        𝔣10 = 10,
        𝔣11 = 11,
        𝔣12 = 12,
        𝔣13 = 13,
        𝔣14 = 14,
        𝔣15 = 15,
        𝔣16 = 16,
        𝔣17 = 17,
        𝔣18 = 18,
        𝔣19 = 19,
        𝔣20 = 20,
        𝔣21 = 21,
        𝔣22 = 22,
        𝔣23 = 23,
        𝔣24 = 24,
        𝔣25 = 25,
        𝔣26 = 26,
        𝔣27 = 27,
        𝔣28 = 28,
        𝔣29 = 29,
        𝔣30 = 30,
        𝔣31 = 31
    }

    [{|value| !(0..=31).contains(&value)}]
    [𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜]
    []
    pub enum(i8, u8) 𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐚𝐛𝐢 {
        𝔣𝔱0 = 0,
        𝔣𝔱1 = 1,
        𝔣𝔱2 = 2,
        𝔣𝔱3 = 3,
        𝔣𝔱4 = 4,
        𝔣𝔱5 = 5,
        𝔣𝔱6 = 6,
        𝔣𝔱7 = 7,
        𝔣𝔰0 = 8,
        𝔣𝔰1 = 9,
        𝔣𝔞0 = 10,
        𝔣𝔞1 = 11,
        𝔣𝔞2 = 12,
        𝔣𝔞3 = 13,
        𝔣𝔞4 = 14,
        𝔣𝔞5 = 15,
        𝔣𝔞6 = 16,
        𝔣𝔞7 = 17,
        𝔣𝔰2 = 18,
        𝔣𝔰3 = 19,
        𝔣𝔰4 = 20,
        𝔣𝔰5 = 21,
        𝔣𝔰6 = 22,
        𝔣𝔰7 = 23,
        𝔣𝔰8 = 24,
        𝔣𝔰9 = 25,
        𝔣𝔰10 = 26,
        𝔣𝔰11 = 27,
        𝔣𝔱8 = 28,
        𝔣𝔱9 = 29,
        𝔣𝔱10 = 30,
        𝔣𝔱11 = 31
    }

    [{|value| !(0..=4).contains(&value) && value != 7}]
    []
    []
    pub enum(i8, u8) 𝐫𝐨𝐮𝐧𝐝𝐢𝐧𝐠_𝐦𝐨𝐝𝐞 {
        𝔯𝔫𝔢 = 0,
        𝔯𝔱𝔷 = 1,
        𝔯𝔡𝔫 = 2,
        𝔯𝔲𝔭 = 3,
        𝔯𝔪𝔪 = 4,
        𝔡𝔶𝔫 = 7
    }

    [{|value| !(1..=15).contains(&value)}]
    []
    []
    pub enum(i8, u8) 𝐟𝐞𝐧𝐜𝐞_𝐨𝐩𝐞𝐫𝐚𝐧𝐝 {
        𝔴 = 1,
        𝔯 = 2,
        𝔯𝔴 = 3,
        𝔬 = 4,
        𝔬𝔴 = 5,
        𝔬𝔯 = 6,
        𝔬𝔯𝔴 = 7,
        𝔦 = 8,
        𝔦𝔴 = 9,
        𝔦𝔯 = 10,
        𝔦𝔯𝔴 = 11,
        𝔦𝔬 = 12,
        𝔦𝔬𝔴 = 13,
        𝔦𝔬𝔯 = 14,
        𝔦𝔬𝔯𝔴 = 15
    }

    [{|value| !(0x001..=0x003).contains(&value) &&
              value != 0x100 &&
              !(0x104..=0x106).contains(&value) &&
              value != 0x10a &&
              !(0x140..=0x144).contains(&value) &&
              value != 0x180 &&
              value != 0x200 &&
              !(0x204..=0x205).contains(&value) &&
              !(0x240..=0x244).contains(&value) &&
              value != 0x280 &&
              !(0x300..=0x306).contains(&value) &&
              value != 0x30a &&
              value != 0x310 &&
              value != 0x31a &&
              value != 0x320 &&
              !(0x323..=0x344).contains(&value) &&
              !(0x34a..=0x34b).contains(&value) &&
              !(0x3a0..=0x3ef).contains(&value) &&
              value != 0x5a8 &&
              value != 0x5a8 &&
              value != 0x600 &&
              !(0x602..=0x607).contains(&value) &&
              value != 0x60a &&
              value != 0x615 &&
              value != 0x61a &&
              !(0x643..=0x645).contains(&value) &&
              value != 0x64a &&
              value != 0x747 &&
              value != 0x757 &&
              !(0x7a0..=0x7a3).contains(&value) &&
              value != 0x7a8 &&
              !(0x7b0..=0x7b3).contains(&value) &&
              value != 0xb00 &&
              !(0xb02..=0xb1f).contains(&value) &&
              value != 0xb80 &&
              !(0xb82..=0xb9f).contains(&value) &&
              !(0xc00..=0xc1f).contains(&value) &&
              !(0xc80..=0xc9f).contains(&value) &&
              value != 0xe12 &&
              !(0xf11..=0xf15).contains(&value)}]
    [𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐫𝐯𝟔𝟒]
    []
    pub enum(i16, u16) 𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐫𝐯𝟑𝟐 {
        𝔣𝔣𝔩𝔞𝔤𝔰 = 0x001,
        𝔣𝔯𝔪 = 0x002,
        𝔣𝔠𝔰𝔯 = 0x003,
        𝔰𝔰𝔱𝔞𝔱𝔲𝔰 = 0x100,
        𝔰𝔦𝔢 = 0x104,
        𝔰𝔱𝔳𝔢𝔠 = 0x105,
        𝔰𝔠𝔬𝔲𝔫𝔱𝔢𝔯𝔢𝔫 = 0x106,
        𝔰𝔢𝔫𝔳𝔠𝔣𝔤 = 0x10a,
        𝔰𝔰𝔠𝔯𝔞𝔱𝔠𝔥 = 0x140,
        𝔰𝔢𝔭𝔠 = 0x141,
        𝔰𝔠𝔞𝔲𝔰𝔢 = 0x142,
        𝔰𝔱𝔳𝔞𝔩 = 0x143,
        𝔰𝔦𝔭 = 0x144,
        𝔰𝔞𝔱𝔭 = 0x180,
        𝔳𝔰𝔰𝔱𝔞𝔱𝔲𝔰 = 0x200,
        𝔳𝔰𝔦𝔢 = 0x204,
        𝔳𝔰𝔱𝔳𝔢𝔠 = 0x205,
        𝔳𝔰𝔰𝔠𝔯𝔞𝔱𝔠𝔥 = 0x240,
        𝔳𝔰𝔢𝔭𝔠 = 0x241,
        𝔳𝔰𝔠𝔞𝔲𝔰𝔢 = 0x242,
        𝔳𝔰𝔱𝔳𝔞𝔩 = 0x243,
        𝔳𝔰𝔦𝔭 = 0x244,
        𝔳𝔰𝔞𝔱𝔭 = 0x280,
        𝔪𝔰𝔱𝔞𝔱𝔲𝔰 = 0x300,
        𝔪𝔦𝔰𝔞 = 0x301,
        𝔪𝔢𝔡𝔢𝔩𝔢𝔤 = 0x302,
        𝔪𝔦𝔡𝔢𝔩𝔢𝔤 = 0x303,
        𝔪𝔦𝔢 = 0x304,
        𝔪𝔱𝔳𝔢𝔠 = 0x305,
        𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯𝔢𝔫 = 0x306,
        𝔪𝔢𝔫𝔳𝔠𝔣𝔤 = 0x30a,
        𝔪𝔰𝔱𝔞𝔱𝔲𝔰𝔥 = 0x310,
        𝔪𝔢𝔫𝔳𝔠𝔣𝔤𝔥 = 0x31a,
        𝔪𝔠𝔬𝔲𝔫𝔱𝔦𝔫𝔥𝔦𝔟𝔦𝔱 = 0x320,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱3 = 0x323,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱4 = 0x324,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱5 = 0x325,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱6 = 0x326,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱7 = 0x327,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱8 = 0x328,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱9 = 0x329,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱10 = 0x32a,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱11 = 0x32b,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱12 = 0x32c,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱13 = 0x32d,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱14 = 0x32e,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱15 = 0x32f,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱16 = 0x330,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱17 = 0x331,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱18 = 0x332,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱19 = 0x333,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱20 = 0x334,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱21 = 0x335,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱22 = 0x336,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱23 = 0x337,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱24 = 0x338,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱25 = 0x339,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱26 = 0x33a,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱27 = 0x33b,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱28 = 0x33c,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱29 = 0x33d,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱30 = 0x33e,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱31 = 0x33f,
        𝔪𝔰𝔠𝔯𝔞𝔱𝔠𝔥 = 0x340,
        𝔪𝔢𝔭𝔠 = 0x341,
        𝔪𝔠𝔞𝔲𝔰𝔢 = 0x342,
        𝔪𝔱𝔳𝔞𝔩 = 0x343,
        𝔪𝔦𝔭 = 0x344,
        𝔪𝔱𝔦𝔫𝔰𝔱 = 0x34a,
        𝔪𝔱𝔳𝔞𝔩2 = 0x34b,
        𝔭𝔪𝔭𝔠𝔣𝔤0 = 0x3a0,
        𝔭𝔪𝔭𝔠𝔣𝔤1 = 0x3a1,
        𝔭𝔪𝔭𝔠𝔣𝔤2 = 0x3a2,
        𝔭𝔪𝔭𝔠𝔣𝔤3 = 0x3a3,
        𝔭𝔪𝔭𝔠𝔣𝔤4 = 0x3a4,
        𝔭𝔪𝔭𝔠𝔣𝔤5 = 0x3a5,
        𝔭𝔪𝔭𝔠𝔣𝔤6 = 0x3a6,
        𝔭𝔪𝔭𝔠𝔣𝔤7 = 0x3a7,
        𝔭𝔪𝔭𝔠𝔣𝔤8 = 0x3a8,
        𝔭𝔪𝔭𝔠𝔣𝔤9 = 0x3a9,
        𝔭𝔪𝔭𝔠𝔣𝔤10 = 0x3aa,
        𝔭𝔪𝔭𝔠𝔣𝔤11 = 0x3ab,
        𝔭𝔪𝔭𝔠𝔣𝔤12 = 0x3ac,
        𝔭𝔪𝔭𝔠𝔣𝔤13 = 0x3ad,
        𝔭𝔪𝔭𝔠𝔣𝔤14 = 0x3ae,
        𝔭𝔪𝔭𝔠𝔣𝔤15 = 0x3af,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯0 = 0x3b0,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯1 = 0x3b1,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯2 = 0x3b2,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯3 = 0x3b3,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯4 = 0x3b4,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯5 = 0x3b5,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯6 = 0x3b6,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯7 = 0x3b7,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯8 = 0x3b8,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯9 = 0x3b9,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯10 = 0x3ba,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯11 = 0x3bb,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯12 = 0x3bc,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯13 = 0x3bd,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯14 = 0x3be,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯15 = 0x3bf,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯16 = 0x3c0,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯17 = 0x3c1,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯18 = 0x3c2,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯19 = 0x3c3,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯20 = 0x3c4,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯21 = 0x3c5,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯22 = 0x3c6,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯23 = 0x3c7,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯24 = 0x3c8,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯25 = 0x3c9,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯26 = 0x3ca,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯27 = 0x3cb,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯28 = 0x3cc,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯29 = 0x3cd,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯30 = 0x3ce,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯31 = 0x3cf,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯32 = 0x3d0,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯33 = 0x3d1,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯34 = 0x3d2,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯35 = 0x3d3,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯36 = 0x3d4,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯37 = 0x3d5,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯38 = 0x3d6,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯39 = 0x3d7,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯40 = 0x3d8,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯41 = 0x3d9,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯42 = 0x3da,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯43 = 0x3db,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯44 = 0x3dc,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯45 = 0x3dd,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯46 = 0x3de,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯47 = 0x3df,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯48 = 0x3e0,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯49 = 0x3e1,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯50 = 0x3e2,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯51 = 0x3e3,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯52 = 0x3e4,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯53 = 0x3e5,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯54 = 0x3e6,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯55 = 0x3e7,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯56 = 0x3e8,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯57 = 0x3e9,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯58 = 0x3ea,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯59 = 0x3eb,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯60 = 0x3ec,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯61 = 0x3ed,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯62 = 0x3ee,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯63 = 0x3ef,
        𝔰𝔠𝔬𝔫𝔱𝔢𝔵𝔱 = 0x5a8,
        𝔥𝔰𝔱𝔞𝔱𝔲𝔰 = 0x600,
        𝔥𝔢𝔡𝔢𝔩𝔢𝔤 = 0x602,
        𝔥𝔦𝔡𝔢𝔩𝔢𝔤 = 0x603,
        𝔥𝔦𝔢 = 0x604,
        𝔥𝔱𝔦𝔪𝔢𝔡𝔢𝔩𝔱𝔞 = 0x605,
        𝔥𝔠𝔬𝔲𝔫𝔱𝔢𝔯𝔢𝔫 = 0x606,
        𝔥𝔤𝔢𝔦𝔢 = 0x607,
        𝔥𝔢𝔫𝔳𝔠𝔣𝔤 = 0x60a,
        𝔥𝔱𝔦𝔪𝔢𝔡𝔢𝔩𝔱𝔞𝔥 = 0x615,
        𝔥𝔢𝔫𝔳𝔠𝔣𝔤𝔥 = 0x61a,
        𝔥𝔱𝔳𝔞𝔩 = 0x643,
        𝔥𝔦𝔭 = 0x644,
        𝔥𝔳𝔦𝔭 = 0x645,
        𝔥𝔱𝔦𝔫𝔰𝔱 = 0x64a,
        𝔪𝔰𝔢𝔠𝔠𝔣𝔤 = 0x747,
        𝔪𝔰𝔢𝔠𝔠𝔣𝔤𝔥 = 0x757,
        𝔱𝔰𝔢𝔩𝔢𝔠𝔱 = 0x7a0,
        𝔱𝔡𝔞𝔱𝔞1 = 0x7a1,
        𝔱𝔡𝔞𝔱𝔞2 = 0x7a2,
        𝔱𝔡𝔞𝔱𝔞3 = 0x7a3,
        𝔪𝔠𝔬𝔫𝔱𝔢𝔵𝔱 = 0x7a8,
        𝔡𝔠𝔰𝔯 = 0x7b0,
        𝔡𝔭𝔠 = 0x7b1,
        𝔡𝔰𝔠𝔯𝔞𝔱𝔠𝔥0 = 0x7b2,
        𝔡𝔰𝔠𝔯𝔞𝔱𝔠𝔥1 = 0x7b3,
        𝔪𝔠𝔶𝔠𝔩𝔢 = 0xb00,
        𝔪𝔦𝔫𝔰𝔱𝔯𝔢𝔱 = 0xb02,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯3 = 0xb03,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯4 = 0xb04,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯5 = 0xb05,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯6 = 0xb06,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯7 = 0xb07,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯8 = 0xb08,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯9 = 0xb09,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯10 = 0xb0a,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯11 = 0xb0b,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯12 = 0xb0c,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯13 = 0xb0d,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯14 = 0xb0e,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯15 = 0xb0f,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯16 = 0xb10,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯17 = 0xb11,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯18 = 0xb12,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯19 = 0xb13,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯20 = 0xb14,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯21 = 0xb15,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯22 = 0xb16,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯23 = 0xb17,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯24 = 0xb18,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯25 = 0xb19,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯26 = 0xb1a,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯27 = 0xb1b,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯28 = 0xb1c,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯29 = 0xb1d,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯30 = 0xb1e,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯31 = 0xb1f,
        𝔪𝔠𝔶𝔠𝔩𝔢𝔥 = 0xb80,
        𝔪𝔦𝔫𝔰𝔱𝔯𝔢𝔱𝔥 = 0xb82,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯3𝔥 = 0xb83,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯4𝔥 = 0xb84,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯5𝔥 = 0xb85,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯6𝔥 = 0xb86,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯7𝔥 = 0xb87,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯8𝔥 = 0xb88,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯9𝔥 = 0xb89,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯10𝔥 = 0xb8a,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯11𝔥 = 0xb8b,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯12𝔥 = 0xb8c,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯13𝔥 = 0xb8d,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯14𝔥 = 0xb8e,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯15𝔥 = 0xb8f,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯16𝔥 = 0xb90,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯17𝔥 = 0xb91,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯18𝔥 = 0xb92,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯19𝔥 = 0xb93,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯20𝔥 = 0xb94,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯21𝔥 = 0xb95,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯22𝔥 = 0xb96,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯23𝔥 = 0xb97,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯24𝔥 = 0xb98,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯25𝔥 = 0xb99,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯26𝔥 = 0xb9a,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯27𝔥 = 0xb9b,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯28𝔥 = 0xb9c,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯29𝔥 = 0xb9d,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯30𝔥 = 0xb9e,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯31𝔥 = 0xb9f,
        𝔠𝔶𝔠𝔩𝔢 = 0xc00,
        𝔱𝔦𝔪𝔢 = 0xc01,
        𝔦𝔫𝔰𝔱𝔯𝔢𝔱 = 0xc02,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯3 = 0xc03,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯4 = 0xc04,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯5 = 0xc05,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯6 = 0xc06,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯7 = 0xc07,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯8 = 0xc08,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯9 = 0xc09,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯10 = 0xc0a,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯11 = 0xc0b,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯12 = 0xc0c,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯13 = 0xc0d,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯14 = 0xc0e,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯15 = 0xc0f,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯16 = 0xc10,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯17 = 0xc11,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯18 = 0xc12,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯19 = 0xc13,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯20 = 0xc14,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯21 = 0xc15,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯22 = 0xc16,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯23 = 0xc17,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯24 = 0xc18,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯25 = 0xc19,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯26 = 0xc1a,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯27 = 0xc1b,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯28 = 0xc1c,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯29 = 0xc1d,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯30 = 0xc1e,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯31 = 0xc1f,
        𝔠𝔶𝔠𝔩𝔢𝔥 = 0xc80,
        𝔱𝔦𝔪𝔢𝔥 = 0xc81,
        𝔦𝔫𝔰𝔱𝔯𝔢𝔱𝔥 = 0xc82,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯3𝔥 = 0xc83,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯4𝔥 = 0xc84,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯5𝔥 = 0xc85,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯6𝔥 = 0xc86,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯7𝔥 = 0xc87,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯8𝔥 = 0xc88,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯9𝔥 = 0xc89,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯10𝔥 = 0xc8a,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯11𝔥 = 0xc8b,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯12𝔥 = 0xc8c,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯13𝔥 = 0xc8d,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯14𝔥 = 0xc8e,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯15𝔥 = 0xc8f,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯16𝔥 = 0xc90,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯17𝔥 = 0xc91,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯18𝔥 = 0xc92,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯19𝔥 = 0xc93,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯20𝔥 = 0xc94,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯21𝔥 = 0xc95,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯22𝔥 = 0xc96,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯23𝔥 = 0xc97,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯24𝔥 = 0xc98,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯25𝔥 = 0xc99,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯26𝔥 = 0xc9a,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯27𝔥 = 0xc9b,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯28𝔥 = 0xc9c,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯29𝔥 = 0xc9d,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯30𝔥 = 0xc9e,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯31𝔥 = 0xc9f,
        𝔥𝔤𝔢𝔦𝔭 = 0xe12,
        𝔪𝔳𝔢𝔫𝔡𝔬𝔯𝔦𝔡 = 0xf11,
        𝔪𝔞𝔯𝔠𝔥𝔦𝔡 = 0xf12,
        𝔪𝔦𝔪𝔭𝔦𝔡 = 0xf13,
        𝔪𝔥𝔞𝔯𝔱𝔦𝔡 = 0xf14,
        𝔪𝔠𝔬𝔫𝔣𝔦𝔤𝔭𝔱𝔯 = 0xf15
    }

    [{|value| !(0x001..=0x003).contains(&value) &&
              value != 0x100 &&
              !(0x104..=0x106).contains(&value) &&
              value != 0x10a &&
              !(0x140..=0x144).contains(&value) &&
              value != 0x180 &&
              value != 0x200 &&
              !(0x204..=0x205).contains(&value) &&
              !(0x240..=0x244).contains(&value) &&
              value != 0x280 &&
              !(0x300..=0x306).contains(&value) &&
              value != 0x30a &&
              value != 0x320 &&
              !(0x323..=0x344).contains(&value) &&
              !(0x34a..=0x34b).contains(&value) &&
              !((0x3a0..=0x3ef).contains(&value) && (value & 0xff1 != 0x3a1)) &&
              value != 0x5a8 &&
              value != 0x5a8 &&
              value != 0x600 &&
              !(0x602..=0x607).contains(&value) &&
              value != 0x60a &&
              !(0x643..=0x645).contains(&value) &&
              value != 0x64a &&
              value != 0x747 &&
              !(0x7a0..=0x7a3).contains(&value) &&
              value != 0x7a8 &&
              !(0x7b0..=0x7b3).contains(&value) &&
              value != 0xb00 &&
              !(0xb02..=0xb1f).contains(&value) &&
              !(0xc00..=0xc1f).contains(&value) &&
              value != 0xe12 &&
              !(0xf11..=0xf15).contains(&value)}]
    []
    [   𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐫𝐯𝟑𝟐 {|value| (value & 0xff1 == 0x3a1) ||
                                   (value < 0x7a0) && (value & 0x0b0 == 0x010) ||
                                   (value >= 0x800) && (value & 0x080 == 0x080)}]
    pub enum(i16, u16) 𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐫𝐯𝟔𝟒 {
        𝔣𝔣𝔩𝔞𝔤𝔰 = 0x001,
        𝔣𝔯𝔪 = 0x002,
        𝔣𝔠𝔰𝔯 = 0x003,
        𝔰𝔰𝔱𝔞𝔱𝔲𝔰 = 0x100,
        𝔰𝔦𝔢 = 0x104,
        𝔰𝔱𝔳𝔢𝔠 = 0x105,
        𝔰𝔠𝔬𝔲𝔫𝔱𝔢𝔯𝔢𝔫 = 0x106,
        𝔰𝔢𝔫𝔳𝔠𝔣𝔤 = 0x10a,
        𝔰𝔰𝔠𝔯𝔞𝔱𝔠𝔥 = 0x140,
        𝔰𝔢𝔭𝔠 = 0x141,
        𝔰𝔠𝔞𝔲𝔰𝔢 = 0x142,
        𝔰𝔱𝔳𝔞𝔩 = 0x143,
        𝔰𝔦𝔭 = 0x144,
        𝔰𝔞𝔱𝔭 = 0x180,
        𝔳𝔰𝔰𝔱𝔞𝔱𝔲𝔰 = 0x200,
        𝔳𝔰𝔦𝔢 = 0x204,
        𝔳𝔰𝔱𝔳𝔢𝔠 = 0x205,
        𝔳𝔰𝔰𝔠𝔯𝔞𝔱𝔠𝔥 = 0x240,
        𝔳𝔰𝔢𝔭𝔠 = 0x241,
        𝔳𝔰𝔠𝔞𝔲𝔰𝔢 = 0x242,
        𝔳𝔰𝔱𝔳𝔞𝔩 = 0x243,
        𝔳𝔰𝔦𝔭 = 0x244,
        𝔳𝔰𝔞𝔱𝔭 = 0x280,
        𝔪𝔰𝔱𝔞𝔱𝔲𝔰 = 0x300,
        𝔪𝔦𝔰𝔞 = 0x301,
        𝔪𝔢𝔡𝔢𝔩𝔢𝔤 = 0x302,
        𝔪𝔦𝔡𝔢𝔩𝔢𝔤 = 0x303,
        𝔪𝔦𝔢 = 0x304,
        𝔪𝔱𝔳𝔢𝔠 = 0x305,
        𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯𝔢𝔫 = 0x306,
        𝔪𝔢𝔫𝔳𝔠𝔣𝔤 = 0x30a,
        𝔪𝔠𝔬𝔲𝔫𝔱𝔦𝔫𝔥𝔦𝔟𝔦𝔱 = 0x320,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱3 = 0x323,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱4 = 0x324,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱5 = 0x325,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱6 = 0x326,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱7 = 0x327,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱8 = 0x328,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱9 = 0x329,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱10 = 0x32a,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱11 = 0x32b,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱12 = 0x32c,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱13 = 0x32d,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱14 = 0x32e,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱15 = 0x32f,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱16 = 0x330,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱17 = 0x331,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱18 = 0x332,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱19 = 0x333,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱20 = 0x334,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱21 = 0x335,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱22 = 0x336,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱23 = 0x337,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱24 = 0x338,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱25 = 0x339,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱26 = 0x33a,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱27 = 0x33b,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱28 = 0x33c,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱29 = 0x33d,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱30 = 0x33e,
        𝔪𝔥𝔭𝔪𝔢𝔳𝔢𝔫𝔱31 = 0x33f,
        𝔪𝔰𝔠𝔯𝔞𝔱𝔠𝔥 = 0x340,
        𝔪𝔢𝔭𝔠 = 0x341,
        𝔪𝔠𝔞𝔲𝔰𝔢 = 0x342,
        𝔪𝔱𝔳𝔞𝔩 = 0x343,
        𝔪𝔦𝔭 = 0x344,
        𝔪𝔱𝔦𝔫𝔰𝔱 = 0x34a,
        𝔪𝔱𝔳𝔞𝔩2 = 0x34b,
        𝔭𝔪𝔭𝔠𝔣𝔤0 = 0x3a0,
        𝔭𝔪𝔭𝔠𝔣𝔤2 = 0x3a2,
        𝔭𝔪𝔭𝔠𝔣𝔤4 = 0x3a4,
        𝔭𝔪𝔭𝔠𝔣𝔤6 = 0x3a6,
        𝔭𝔪𝔭𝔠𝔣𝔤8 = 0x3a8,
        𝔭𝔪𝔭𝔠𝔣𝔤10 = 0x3aa,
        𝔭𝔪𝔭𝔠𝔣𝔤12 = 0x3ac,
        𝔭𝔪𝔭𝔠𝔣𝔤14 = 0x3ae,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯0 = 0x3b0,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯1 = 0x3b1,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯2 = 0x3b2,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯3 = 0x3b3,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯4 = 0x3b4,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯5 = 0x3b5,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯6 = 0x3b6,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯7 = 0x3b7,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯8 = 0x3b8,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯9 = 0x3b9,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯10 = 0x3ba,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯11 = 0x3bb,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯12 = 0x3bc,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯13 = 0x3bd,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯14 = 0x3be,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯15 = 0x3bf,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯16 = 0x3c0,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯17 = 0x3c1,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯18 = 0x3c2,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯19 = 0x3c3,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯20 = 0x3c4,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯21 = 0x3c5,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯22 = 0x3c6,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯23 = 0x3c7,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯24 = 0x3c8,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯25 = 0x3c9,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯26 = 0x3ca,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯27 = 0x3cb,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯28 = 0x3cc,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯29 = 0x3cd,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯30 = 0x3ce,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯31 = 0x3cf,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯32 = 0x3d0,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯33 = 0x3d1,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯34 = 0x3d2,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯35 = 0x3d3,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯36 = 0x3d4,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯37 = 0x3d5,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯38 = 0x3d6,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯39 = 0x3d7,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯40 = 0x3d8,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯41 = 0x3d9,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯42 = 0x3da,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯43 = 0x3db,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯44 = 0x3dc,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯45 = 0x3dd,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯46 = 0x3de,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯47 = 0x3df,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯48 = 0x3e0,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯49 = 0x3e1,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯50 = 0x3e2,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯51 = 0x3e3,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯52 = 0x3e4,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯53 = 0x3e5,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯54 = 0x3e6,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯55 = 0x3e7,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯56 = 0x3e8,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯57 = 0x3e9,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯58 = 0x3ea,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯59 = 0x3eb,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯60 = 0x3ec,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯61 = 0x3ed,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯62 = 0x3ee,
        𝔭𝔪𝔭𝔞𝔡𝔡𝔯63 = 0x3ef,
        𝔰𝔠𝔬𝔫𝔱𝔢𝔵𝔱 = 0x5a8,
        𝔥𝔰𝔱𝔞𝔱𝔲𝔰 = 0x600,
        𝔥𝔢𝔡𝔢𝔩𝔢𝔤 = 0x602,
        𝔥𝔦𝔡𝔢𝔩𝔢𝔤 = 0x603,
        𝔥𝔦𝔢 = 0x604,
        𝔥𝔱𝔦𝔪𝔢𝔡𝔢𝔩𝔱𝔞 = 0x605,
        𝔥𝔠𝔬𝔲𝔫𝔱𝔢𝔯𝔢𝔫 = 0x606,
        𝔥𝔤𝔢𝔦𝔢 = 0x607,
        𝔥𝔢𝔫𝔳𝔠𝔣𝔤 = 0x60a,
        𝔥𝔱𝔳𝔞𝔩 = 0x643,
        𝔥𝔦𝔭 = 0x644,
        𝔥𝔳𝔦𝔭 = 0x645,
        𝔥𝔱𝔦𝔫𝔰𝔱 = 0x64a,
        𝔪𝔰𝔢𝔠𝔠𝔣𝔤 = 0x747,
        𝔱𝔰𝔢𝔩𝔢𝔠𝔱 = 0x7a0,
        𝔱𝔡𝔞𝔱𝔞1 = 0x7a1,
        𝔱𝔡𝔞𝔱𝔞2 = 0x7a2,
        𝔱𝔡𝔞𝔱𝔞3 = 0x7a3,
        𝔪𝔠𝔬𝔫𝔱𝔢𝔵𝔱 = 0x7a8,
        𝔡𝔠𝔰𝔯 = 0x7b0,
        𝔡𝔭𝔠 = 0x7b1,
        𝔡𝔰𝔠𝔯𝔞𝔱𝔠𝔥0 = 0x7b2,
        𝔡𝔰𝔠𝔯𝔞𝔱𝔠𝔥1 = 0x7b3,
        𝔪𝔠𝔶𝔠𝔩𝔢 = 0xb00,
        𝔪𝔦𝔫𝔰𝔱𝔯𝔢𝔱 = 0xb02,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯3 = 0xb03,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯4 = 0xb04,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯5 = 0xb05,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯6 = 0xb06,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯7 = 0xb07,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯8 = 0xb08,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯9 = 0xb09,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯10 = 0xb0a,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯11 = 0xb0b,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯12 = 0xb0c,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯13 = 0xb0d,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯14 = 0xb0e,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯15 = 0xb0f,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯16 = 0xb10,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯17 = 0xb11,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯18 = 0xb12,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯19 = 0xb13,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯20 = 0xb14,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯21 = 0xb15,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯22 = 0xb16,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯23 = 0xb17,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯24 = 0xb18,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯25 = 0xb19,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯26 = 0xb1a,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯27 = 0xb1b,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯28 = 0xb1c,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯29 = 0xb1d,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯30 = 0xb1e,
        𝔪𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯31 = 0xb1f,
        𝔠𝔶𝔠𝔩𝔢 = 0xc00,
        𝔱𝔦𝔪𝔢 = 0xc01,
        𝔦𝔫𝔰𝔱𝔯𝔢𝔱 = 0xc02,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯3 = 0xc03,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯4 = 0xc04,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯5 = 0xc05,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯6 = 0xc06,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯7 = 0xc07,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯8 = 0xc08,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯9 = 0xc09,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯10 = 0xc0a,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯11 = 0xc0b,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯12 = 0xc0c,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯13 = 0xc0d,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯14 = 0xc0e,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯15 = 0xc0f,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯16 = 0xc10,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯17 = 0xc11,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯18 = 0xc12,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯19 = 0xc13,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯20 = 0xc14,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯21 = 0xc15,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯22 = 0xc16,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯23 = 0xc17,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯24 = 0xc18,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯25 = 0xc19,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯26 = 0xc1a,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯27 = 0xc1b,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯28 = 0xc1c,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯29 = 0xc1d,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯30 = 0xc1e,
        𝔥𝔭𝔪𝔠𝔬𝔲𝔫𝔱𝔢𝔯31 = 0xc1f,
        𝔥𝔤𝔢𝔦𝔭 = 0xe12,
        𝔪𝔳𝔢𝔫𝔡𝔬𝔯𝔦𝔡 = 0xf11,
        𝔪𝔞𝔯𝔠𝔥𝔦𝔡 = 0xf12,
        𝔪𝔦𝔪𝔭𝔦𝔡 = 0xf13,
        𝔪𝔥𝔞𝔯𝔱𝔦𝔡 = 0xf14,
        𝔪𝔠𝔬𝔫𝔣𝔦𝔤𝔭𝔱𝔯 = 0xf15
    }
}

macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖉𝖊𝖋𝖆𝖚𝖑𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗 {
    ($($𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮:ident::$𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝔃𝓮𝓻𝓸_𝓷𝓪𝓶𝓮:ident),*) => {
        $(
            impl Default for $𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                #[inline(always)]
                fn default() -> $𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                    $𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮::$𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝔃𝓮𝓻𝓸_𝓷𝓪𝓶𝓮
                }
            }
         )*
    }
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖉𝖊𝖋𝖆𝖚𝖑𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗! {
    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞::𝔵0,
    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜::𝔵0,
    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞::𝔷𝔢𝔯𝔬,
    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢::𝔷𝔢𝔯𝔬,
    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢::𝔷𝔢𝔯𝔬
}