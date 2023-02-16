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

#[cfg(feature = "std")]
macro_rules! 𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙 {
    ($𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮:ident: $($𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮:ident => $𝓮𝓷𝓾𝓶_𝓼𝓽𝓻𝓲𝓷𝓰:literal),*) => {
        $(
            assert_eq!(super::𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮::$𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮.to_string(), $𝓮𝓷𝓾𝓶_𝓼𝓽𝓻𝓲𝓷𝓰);
         )*
    }
}

#[cfg(feature = "std")]
#[test]
fn test_enums_display() {
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞:
        𝔵1 => "x1", 𝔵2 => "x2", 𝔵3 => "x3", 𝔵4 => "x4", 𝔵5 => "x5", 𝔵6 => "x6", 𝔵7 => "x7", 𝔵8 => "x8",
        𝔵9 => "x9", 𝔵10 => "x10", 𝔵11 => "x11", 𝔵12 => "x12", 𝔵13 => "x13", 𝔵14 => "x14", 𝔵15 => "x15");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜:
        𝔵1 => "x1", 𝔵2 => "x2", 𝔵3 => "x3", 𝔵4 => "x4", 𝔵5 => "x5", 𝔵6 => "x6", 𝔵7 => "x7", 𝔵8 => "x8", 𝔵9 => "x9",
        𝔵10 => "x10", 𝔵11 => "x11", 𝔵12 => "x12", 𝔵13 => "x13", 𝔵14 => "x14", 𝔵15 => "x15", 𝔵16 => "x16", 𝔵17 => "x17",
        𝔵18 => "x18", 𝔵19 => "x19", 𝔵20 => "x20", 𝔵21 => "x21", 𝔵22 => "x22", 𝔵23 => "x23", 𝔵24 => "x24", 𝔵25 => "x25",
        𝔵26 => "x26", 𝔵27 => "x27", 𝔵28 => "x28", 𝔵29 => "x29", 𝔵30 => "x30", 𝔵31 => "x31");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞:
        𝔞0 => "a0", 𝔞1 => "a1", 𝔞2 => "a2", 𝔞3 => "a3",
        𝔰0 => "s0", 𝔰1 => "s1", 𝔰2 => "s2", 𝔰3 => "s3", 𝔰4 => "s4",
        𝔤𝔭 => "gp", 𝔯𝔞 => "ra", 𝔰𝔭 => "sp", 𝔱𝔭 => "tp", 𝔱0 => "t0", 𝔱1 => "t1");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢:
        𝔞0 => "a0", 𝔞1 => "a1", 𝔞2 => "a2", 𝔞3 => "a3",
        𝔰0 => "s0", 𝔰1 => "s1", 𝔰2 => "s2", 𝔰3 => "s3", 𝔰4 => "s4", 𝔰5 => "s5", 𝔰6 => "s6",
        𝔰7 => "s7", 𝔰8 => "s8", 𝔰9 => "s9", 𝔰10 => "s10", 𝔰11 => "s11", 𝔰12 => "s12", 𝔰13 => "s13",
        𝔰14 => "s14", 𝔰15 => "s15", 𝔰16 => "s16", 𝔰17 => "s17", 𝔰18 => "s18", 𝔰19 => "s19", 𝔰20 => "s20",
        𝔤𝔭 => "gp", 𝔯𝔞 => "ra", 𝔰𝔭 => "sp", 𝔱𝔭 => "tp", 𝔱0 => "t0", 𝔱1 => "t1");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢:
        𝔞0 => "a0", 𝔞1 => "a1", 𝔞2 => "a2", 𝔞3 => "a3", 𝔞4 => "a4", 𝔞5 => "a5", 𝔞6 => "a6", 𝔞7 => "a7",
        𝔰0 => "s0", 𝔰1 => "s1", 𝔰2 => "s2", 𝔰3 => "s3", 𝔰4 => "s4", 𝔰5 => "s5",
        𝔰6 => "s6", 𝔰7 => "s7", 𝔰8 => "s8", 𝔰9 => "s9", 𝔰10 => "s10", 𝔰11 => "s11",
        𝔱0 => "t0", 𝔱1 => "t1", 𝔱2 => "t2", 𝔱3 => "t3", 𝔱4 => "t4", 𝔱5 => "t5", 𝔱6 => "t6",
        𝔤𝔭 => "gp", 𝔯𝔞 => "ra", 𝔰𝔭 => "sp", 𝔱𝔭 => "tp");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞:
        𝔵0 => "x0", 𝔵1 => "x1", 𝔵2 => "x2", 𝔵3 => "x3", 𝔵4 => "x4", 𝔵5 => "x5", 𝔵6 => "x6", 𝔵7 => "x7",
        𝔵8 => "x8", 𝔵9 => "x9", 𝔵10 => "x10", 𝔵11 => "x11", 𝔵12 => "x12", 𝔵13 => "x13", 𝔵14 => "x14", 𝔵15 => "x15");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜:
        𝔵0 => "x0", 𝔵1 => "x1", 𝔵2 => "x2", 𝔵3 => "x3", 𝔵4 => "x4", 𝔵5 => "x5", 𝔵6 => "x6", 𝔵7 => "x7", 𝔵8 => "x8",
        𝔵9 => "x9", 𝔵10 => "x10", 𝔵11 => "x11", 𝔵12 => "x12", 𝔵13 => "x13", 𝔵14 => "x14", 𝔵15 => "x15", 𝔵16 => "x16",
        𝔵17 => "x17", 𝔵18 => "x18", 𝔵19 => "x19", 𝔵20 => "x20", 𝔵21 => "x21", 𝔵22 => "x22", 𝔵23 => "x23", 𝔵24 => "x24",
        𝔵25 => "x25", 𝔵26 => "x26", 𝔵27 => "x27", 𝔵28 => "x28", 𝔵29 => "x29", 𝔵30 => "x30", 𝔵31 => "x31");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢_𝐫𝐯𝟑𝟐𝐞:
        𝔞0 => "a0", 𝔞1 => "a1", 𝔞2 => "a2", 𝔞3 => "a3",
        𝔰0 => "s0", 𝔰1 => "s1", 𝔰2 => "s2", 𝔰3 => "s3", 𝔰4 => "s4",
        𝔤𝔭 => "gp", 𝔯𝔞 => "ra", 𝔰𝔭 => "sp", 𝔱𝔭 => "tp", 𝔷𝔢𝔯𝔬 => "zero", 𝔱0 => "t0", 𝔱1 => "t1");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢:
        𝔞0 => "a0", 𝔞1 => "a1", 𝔞2 => "a2", 𝔞3 => "a3",
        𝔰0 => "s0", 𝔰1 => "s1", 𝔰2 => "s2", 𝔰3 => "s3", 𝔰4 => "s4", 𝔰5 => "s5", 𝔰6 => "s6",
        𝔰7 => "s7", 𝔰8 => "s8", 𝔰9 => "s9", 𝔰10 => "s10", 𝔰11 => "s11", 𝔰12 => "s12", 𝔰13 => "s13",
        𝔰14 => "s14", 𝔰15 => "s15", 𝔰16 => "s16", 𝔰17 => "s17", 𝔰18 => "s18", 𝔰19 => "s19", 𝔰20 => "s20",
        𝔤𝔭 => "gp", 𝔯𝔞 => "ra", 𝔰𝔭 => "sp", 𝔱𝔭 => "tp", 𝔷𝔢𝔯𝔬 => "zero", 𝔱0 => "t0", 𝔱1 => "t1");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢:
        𝔞0 => "a0", 𝔞1 => "a1", 𝔞2 => "a2", 𝔞3 => "a3", 𝔞4 => "a4", 𝔞5 => "a5", 𝔞6 => "a6", 𝔞7 => "a7",
        𝔰0 => "s0", 𝔰1 => "s1", 𝔰2 => "s2", 𝔰3 => "s3", 𝔰4 => "s4", 𝔰5 => "s5",
        𝔰6 => "s6", 𝔰7 => "s7", 𝔰8 => "s8", 𝔰9 => "s9", 𝔰10 => "s10", 𝔰11 => "s11",
        𝔱0 => "t0", 𝔱1 => "t1", 𝔱2 => "t2", 𝔱3 => "t3", 𝔱4 => "t4", 𝔱5 => "t5", 𝔱6 => "t6",
        𝔤𝔭 => "gp", 𝔯𝔞 => "ra", 𝔰𝔭 => "sp", 𝔱𝔭 => "tp", 𝔷𝔢𝔯𝔬 => "zero");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜:
        𝔣0 => "f0", 𝔣1 => "f1", 𝔣2 => "f2", 𝔣3 => "f3", 𝔣4 => "f4", 𝔣5 => "f5", 𝔣6 => "f6", 𝔣7 => "f7", 𝔣8 => "f8",
        𝔣9 => "f9", 𝔣10 => "f10", 𝔣11 => "f11", 𝔣12 => "f12", 𝔣13 => "f13", 𝔣14 => "f14", 𝔣15 => "f15", 𝔣16 => "f16",
        𝔣17 => "f17", 𝔣18 => "f18", 𝔣19 => "f19", 𝔣20 => "f20", 𝔣21 => "f21", 𝔣22 => "f22", 𝔣23 => "f23", 𝔣24 => "f24",
        𝔣25 => "f25", 𝔣26 => "f26", 𝔣27 => "f27", 𝔣28 => "f28", 𝔣29 => "f29", 𝔣30 => "f30", 𝔣31 => "f31");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐚𝐛𝐢:
        𝔣𝔞0 => "fa0", 𝔣𝔞1 => "fa1", 𝔣𝔞2 => "fa2", 𝔣𝔞3 => "fa3", 𝔣𝔞4 => "fa4", 𝔣𝔞5 => "fa5", 𝔣𝔞6 => "fa6", 𝔣𝔞7 => "fa7",
        𝔣𝔰0 => "fs0", 𝔣𝔰1 => "fs1", 𝔣𝔰2 => "fs2", 𝔣𝔰3 => "fs3", 𝔣𝔰4 => "fs4", 𝔣𝔰5 => "fs5",
        𝔣𝔰6 => "fs6", 𝔣𝔰7 => "fs7", 𝔣𝔰8 => "fs8", 𝔣𝔰9 => "fs9", 𝔣𝔰10 => "fs10", 𝔣𝔰11 => "fs11",
        𝔣𝔱0 => "ft0", 𝔣𝔱1 => "ft1", 𝔣𝔱2 => "ft2", 𝔣𝔱3 => "ft3", 𝔣𝔱4 => "ft4", 𝔣𝔱5 => "ft5",
        𝔣𝔱6 => "ft6", 𝔣𝔱7 => "ft7", 𝔣𝔱8 => "ft8", 𝔣𝔱9 => "ft9", 𝔣𝔱10 => "ft10", 𝔣𝔱11 => "ft11");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐫𝐨𝐮𝐧𝐝𝐢𝐧𝐠_𝐦𝐨𝐝𝐞:
        𝔯𝔫𝔢 => "rne", 𝔯𝔱𝔷 => "rtz", 𝔯𝔡𝔫 => "rdn", 𝔯𝔲𝔭 => "rup", 𝔯𝔪𝔪 => "rmm", 𝔡𝔶𝔫 => "dyn");
    𝖊𝖓𝖚𝖒𝖘_𝖙𝖔_𝖘𝖙𝖗𝖎𝖓𝖌_𝖙𝖊𝖘𝖙!(𝐟𝐞𝐧𝐜𝐞:
        𝔴 => "w", 𝔯 => "r", 𝔯𝔴 => "rw", 𝔬 => "o", 𝔬𝔴 => "ow", 𝔬𝔯 => "or", 𝔬𝔯𝔴 => "orw",
        𝔦 => "i", 𝔦𝔴 => "iw", 𝔦𝔯 => "ir", 𝔦𝔯𝔴 => "irw", 𝔦𝔬 => "io", 𝔦𝔬𝔴 => "iow", 𝔦𝔬𝔯 => "ior", 𝔦𝔬𝔯𝔴 => "iorw");
}

#[cfg(feature = "std")]
#[test]
fn test_csr_register_display() {
    verify_display_for_type::<super::𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐫𝐯𝟑𝟐>();
    verify_display_for_type::<super::𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐫𝐯𝟔𝟒>();

    fn verify_display_for_type<𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮: Clone + core::fmt::Display + core::fmt::Debug + TryFrom<i16>>() {
        for value in i16::MIN..=i16::MAX {
            if let Ok(value) = TryInto::<𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>::try_into(value) {
                let mut expected_string = Vec::new();
                for c in format!("{:?}", value.clone()).chars() {
                    expected_string.push(match c {
                        'ℭ' => b'C',
                        'ℌ' => b'H',
                        'ℑ' => b'I',
                        'ℜ' => b'R',
                        'ℨ' => b'Z',
                        '0'..='9' => c as u8,
                        '𝔄'..='𝔜' => (c as u32 - '𝔄' as u32) as u8 + b'A',
                        '𝔞'..='𝔷' => (c as u32 - '𝔞' as u32) as u8 + b'a',
                        _ => panic!("unsipported character in enum name"),
                    });
                }
                let expected_string = std::str::from_utf8(&expected_string).unwrap();
                assert_eq!(expected_string, value.to_string());
            }
        }
    }
}
