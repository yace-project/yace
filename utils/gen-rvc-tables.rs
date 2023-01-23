#![allow(uncommon_codepoints)]
#![allow(non_camel_case_types)]
#![allow(confusable_idents)]

use yace_codegen::𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘;

type 𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢 =
    yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝘃𝟯𝟮𝗶::𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫;
type 𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢 =
    yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝘃𝟲𝟰𝗶::𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫;

use yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝗶𝘀𝗰_𝘃::𝒂𝒅𝒅𝒓𝒆𝒔𝒔;

use yace::𝗱𝗶𝘀𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝘃𝟯𝟮𝗶::𝑪𝑷𝑼 as 𝑪𝑷𝑼_𝗿𝘃𝟯𝟮𝗶;
use yace::𝗱𝗶𝘀𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝘃𝟲𝟰𝗶::𝑪𝑷𝑼 as 𝑪𝑷𝑼_𝗿𝘃𝟲𝟰𝗶;

use yace::𝗱𝗶𝘀𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝘃𝟯𝟮𝗶::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝 as 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟯𝟮𝗶;
use yace::𝗱𝗶𝘀𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝘃𝟲𝟰𝗶::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝 as 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶;

use yace::𝗱𝗶𝘀𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝘃𝟯𝟮𝗶::𝐨𝐩𝐞𝐫𝐚𝐧𝐝 as 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶;
use yace::𝗱𝗶𝘀𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝘃𝟲𝟰𝗶::𝐨𝐩𝐞𝐫𝐚𝐧𝐝 as 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶;

fn main() {
    let mut primary_table_rv32 = [(0i8, 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔲𝔫𝔯𝔢𝔠𝔬𝔤𝔫𝔦𝔷𝔢𝔡); u8::MAX as usize + 1];
    let mut primary_table_rv64 = [(0i8, 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔲𝔫𝔯𝔢𝔠𝔬𝔤𝔫𝔦𝔷𝔢𝔡); u8::MAX as usize + 1];
    let mut secondary_table_add = None;
    let mut secondary_table_addi = None;
    let mut secondary_table_addi16sp = None;
    let mut secondary_table_addi4spn = None;
    let mut secondary_table_beqz = None;
    let mut secondary_table_jal = None;
    let mut secondary_table_ld = None;
    let mut secondary_table_ldsp = None;
    let mut secondary_table_lw = None;
    let mut secondary_table_lwsp = None;
    let mut secondary_table_sdsp = None;
    let mut secondary_table_swsp = None;
    for (index, encoded_byte) in (u8::MIN..=u8::MAX).enumerate() {
        if encoded_byte & 0b11 == 0b11 {
            primary_table_rv32[index].1 = match (encoded_byte >> 4) & 0b111 {
                0 => 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_0,
                1 => 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_1,
                2 => 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_2,
                3 => 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_3,
                4 => 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_4,
                5 => 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_5,
                6 => 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_6,
                _ => 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_7,
            };
            primary_table_rv64[index].1 = match (encoded_byte >> 4) & 0b111 {
                0 => 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_0,
                1 => 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_1,
                2 => 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_2,
                3 => 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_3,
                4 => 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_4,
                5 => 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_5,
                6 => 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_6,
                _ => 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_7,
            };
            continue;
        }
        let encoded_parcel = ((encoded_byte & 0xfc) as u16) << 8 | (encoded_byte & 0x03) as u16;
        let instruction_rv32 = slow_decode_compressed_instruction_rv32i(encoded_parcel);
        let instruction_rv64 = slow_decode_compressed_instruction_rv64i(encoded_parcel);
        primary_table_rv32[index].1 = instruction_rv32.2;
        primary_table_rv64[index].1 = instruction_rv64.2;
        match instruction_rv32.2 {
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡𝔦4𝔰𝔭𝔫 => {
                assert_eq!(instruction_rv32.3 & -0x00000400, 0);
                assert_eq!(instruction_rv32.4, 0);
                primary_table_rv32[index].0 = (instruction_rv32.3 >> 2) as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔩𝔡
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔴
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔩𝔴
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔰𝔡
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔰𝔴
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔰𝔴 => {
                assert_eq!(instruction_rv32.3 & -0x00000400, 0);
                assert_eq!(instruction_rv32.4, 0);
                primary_table_rv32[index].0 = instruction_rv32.3 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 => {
                assert_eq!(instruction_rv32.3 & -0x00000020, 0x00000000);
                if instruction_rv32.4 == 0 {
                    primary_table_rv32[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00
                }
                assert_eq!(instruction_rv32.4 & -0x00000020, 0);
                primary_table_rv32[index].0 = instruction_rv32.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 => {
                assert_eq!(instruction_rv32.3 & -0x00000020, -0x00000020);
                if instruction_rv32.4 == 0 {
                    primary_table_rv32[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00
                }
                assert_eq!(instruction_rv32.4 & -0x00000020, 0);
                primary_table_rv32[index].0 = instruction_rv32.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔧 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔧𝔞𝔩 => {
                assert!(matches!(instruction_rv32.3 & -0x000007f1, 0x00000000 | -0x00000800));
                primary_table_rv32[index].0 = (instruction_rv32.3 >> 4) as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 => {
                assert_eq!(instruction_rv32.3 & -0x00000020, 0x00000000);
                assert_eq!(instruction_rv32.4 & -0x00000020, 0);
                primary_table_rv32[index].0 = instruction_rv32.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 => {
                assert_eq!(instruction_rv32.3 & -0x00000020, -0x00000020);
                assert_eq!(instruction_rv32.4 & -0x00000020, 0);
                primary_table_rv32[index].0 = instruction_rv32.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 => {
                if instruction_rv32.4 == 2 {
                    assert_eq!(instruction_rv32.3 & -0x00000200, 0x00000000);
                } else {
                    assert_eq!(instruction_rv32.3 & -0x00020000, 0x00000000);
                }
                if instruction_rv32.4 == 0 {
                    primary_table_rv32[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00
                }
                assert_eq!(instruction_rv32.4 & -0x00000020, 0);
                primary_table_rv32[index].0 = instruction_rv32.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 => {
                if instruction_rv32.4 == 2 {
                    assert_eq!(instruction_rv32.3 & -0x00000200, -0x00000200);
                } else {
                    assert_eq!(instruction_rv32.3 & -0x00020000, -0x00020000);
                }
                if instruction_rv32.4 == 0 {
                    primary_table_rv32[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00
                }
                assert_eq!(instruction_rv32.4 & -0x00000020, 0);
                primary_table_rv32[index].0 = instruction_rv32.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔰𝔯𝔩𝔦_𝔩𝔬𝔴 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔰𝔯𝔞𝔦_𝔩𝔬𝔴 => {
                assert_eq!(instruction_rv32.3 & -0x00000020, 0x00000000);
                assert_eq!(instruction_rv32.4, 0);
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔰𝔯𝔩𝔦_𝔥𝔦𝔤𝔥 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔰𝔯𝔞𝔦_𝔥𝔦𝔤𝔥 => {
                assert_eq!(instruction_rv32.3 & -0x00000020, 0x00000020);
                assert_eq!(instruction_rv32.4, 0);
                primary_table_rv32[index].0 = 0x20;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔫𝔡𝔦 => {
                assert!(matches!(instruction_rv32.3 & -0x00000020, 0x00000000 | -0x00000020));
                assert_eq!(instruction_rv32.4, 0);
                primary_table_rv32[index].0 = instruction_rv32.3 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡𝔴_𝔰𝔲𝔟𝔴
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔫𝔡_𝔬𝔯_𝔰𝔲𝔟_𝔵𝔬𝔯
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔯𝔢𝔰𝔢𝔯𝔳𝔢𝔡 => {
                assert_eq!(instruction_rv32.3, 0);
                assert_eq!(instruction_rv32.4, 0);
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔟𝔢𝔮𝔷 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔟𝔫𝔢𝔷 => {
                assert!(matches!(instruction_rv32.3 & -0x000000ff, 0x00000000 | -0x00000100));
                assert_eq!(instruction_rv32.4, 0);
                primary_table_rv32[index].0 = (instruction_rv32.3 >> 1) as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔰𝔩𝔩𝔦_𝔩𝔬𝔴 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔰𝔩𝔩𝔦_𝔥𝔦𝔤𝔥 => {
                assert!(matches!(instruction_rv32.3 & -0x00000020, 0x00000020 | 0x00000000));
                assert_eq!(instruction_rv32.4 & -0x00000020, 0);
                primary_table_rv32[index].0 = instruction_rv32.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔩𝔡𝔰𝔭_𝔩𝔬𝔴
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔩𝔴𝔰𝔭_𝔩𝔬𝔴
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥 => {
                assert!(matches!(instruction_rv32.3 & 0x00000020, 0x00000000 | 0x00000020));
                assert_eq!(instruction_rv32.4 & -0x00000020, 0);
                primary_table_rv32[index].0 = instruction_rv32.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴 => {
                assert_eq!(instruction_rv32.3 & 0x00000020, 0x00000000);
                if instruction_rv32.4 == 0 {
                    primary_table_rv32[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴_00
                }
                assert_eq!(instruction_rv32.4 & -0x00000020, 0);
                primary_table_rv32[index].0 = instruction_rv32.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥 => {
                assert_eq!(instruction_rv32.3 & 0x00000020, 0x00000020);
                if instruction_rv32.4 == 0 {
                    primary_table_rv32[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥_00
                }
                assert_eq!(instruction_rv32.4 & -0x00000020, 0);
                primary_table_rv32[index].0 = instruction_rv32.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔪𝔳 => {
                assert_eq!(instruction_rv32.3, 0);
                if instruction_rv32.4 == 0 {
                    primary_table_rv32[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔪𝔳_00
                }
                assert_eq!(instruction_rv32.4 & -0x00000020, 0);
                primary_table_rv32[index].0 = instruction_rv32.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡 => {
                assert_eq!(instruction_rv32.3, 0);
                if instruction_rv32.4 == 0 {
                    primary_table_rv32[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡_00
                }
                assert_eq!(instruction_rv32.4 & -0x00000020, 0);
                primary_table_rv32[index].0 = instruction_rv32.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔰𝔡𝔰𝔭
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔰𝔴𝔰𝔭
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔰𝔴𝔰𝔭 => {
                assert_eq!(instruction_rv32.3 & -0x00000040, 0);
                assert_eq!(instruction_rv32.4, 0);
                primary_table_rv32[index].0 = instruction_rv32.3 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔪𝔳_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔲𝔫𝔯𝔢𝔠𝔬𝔤𝔫𝔦𝔷𝔢𝔡
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_0
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_1
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_2
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_3
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_4
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_5
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_6
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_7
                => panic!("Unsupported enum value {:?}!", instruction_rv32.2),
        }
        match instruction_rv64.2 {
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦4𝔰𝔭𝔫 => {
                assert_eq!(instruction_rv64.3 & -0x00000400, 0);
                assert_eq!(instruction_rv64.4, 0);
                primary_table_rv64[index].0 = (instruction_rv64.3 >> 2) as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔣𝔩𝔡
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔴
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔡
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔣𝔰𝔡
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔰𝔴
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔰𝔡 => {
                assert_eq!(instruction_rv64.3 & -0x00000400, 0);
                assert_eq!(instruction_rv64.4, 0);
                primary_table_rv64[index].0 = instruction_rv64.3 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 => {
                assert_eq!(instruction_rv64.3 & -0x00000020, 0x00000000);
                if instruction_rv64.4 == 0 {
                    primary_table_rv64[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00
                }
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 => {
                assert_eq!(instruction_rv64.3 & -0x00000020, -0x00000020);
                if instruction_rv64.4 == 0 {
                    primary_table_rv64[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00
                }
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 => {
                assert_eq!(instruction_rv64.3 & -0x00000020, 0x00000000);
                if instruction_rv64.4 == 0 {
                    primary_table_rv64[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00
                }
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 => {
                assert_eq!(instruction_rv64.3 & -0x00000020, -0x00000020);
                if instruction_rv64.4 == 0 {
                    primary_table_rv64[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00
                }
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 => {
                assert_eq!(instruction_rv64.3 & -0x00000020, 0x00000000);
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 => {
                assert_eq!(instruction_rv64.3 & -0x00000020, -0x00000020);
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 => {
                if instruction_rv64.4 == 2 {
                    assert_eq!(instruction_rv64.3 & -0x00000200, 0x00000000);
                } else {
                    assert_eq!(instruction_rv64.3 & -0x00020000, 0x00000000);
                }
                if instruction_rv64.4 == 0 {
                    primary_table_rv64[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00
                }
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 => {
                if instruction_rv64.4 == 2 {
                    assert_eq!(instruction_rv64.3 & -0x00000200, -0x00000200);
                } else {
                    assert_eq!(instruction_rv64.3 & -0x00020000, -0x00020000);
                }
                if instruction_rv64.4 == 0 {
                    primary_table_rv64[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00
                }
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔰𝔯𝔞𝔦 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔰𝔯𝔩𝔦 => {
                assert!(matches!(instruction_rv64.3 & -0x00000020, 0x00000000 | 0x00000020));
                assert_eq!(instruction_rv64.4, 0);
                primary_table_rv64[index].0 = instruction_rv64.3 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔫𝔡𝔦 => {
                assert!(matches!(instruction_rv64.3 & -0x00000020, 0x00000000 | -0x00000020));
                assert_eq!(instruction_rv64.4, 0);
                primary_table_rv64[index].0 = instruction_rv64.3 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔫𝔡_𝔬𝔯_𝔰𝔲𝔟_𝔵𝔬𝔯
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔴_𝔰𝔲𝔟𝔴
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔯𝔢𝔰𝔢𝔯𝔳𝔢𝔡 => {
                assert_eq!(instruction_rv64.3, 0);
                assert_eq!(instruction_rv64.4, 0);
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔧 => {
                assert!(matches!(instruction_rv64.3 & -0x000007f1, 0x00000000 | -0x00000800));
                primary_table_rv64[index].0 = (instruction_rv64.3 >> 4) as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔟𝔢𝔮𝔷 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔟𝔫𝔢𝔷 => {
                assert!(matches!(instruction_rv64.3 & -0x000000ff, 0x00000000 | -0x00000100));
                assert_eq!(instruction_rv64.4, 0);
                primary_table_rv64[index].0 = (instruction_rv64.3 >> 1) as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔰𝔩𝔩𝔦_𝔩𝔬𝔴 => {
                assert_eq!(instruction_rv64.3 & -0x00000020, 0x00000000);
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔣𝔩𝔡𝔰𝔭_𝔩𝔬𝔴
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔣𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥 => {
                assert!(matches!(instruction_rv64.3 & 0x00000020, 0x00000000 | 0x00000020));
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔰𝔩𝔩𝔦_𝔥𝔦𝔤𝔥 => {
                assert_eq!(instruction_rv64.3 & -0x00000020, 0x00000020);
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴 => {
                assert_eq!(instruction_rv64.3 & 0x00000020, 0x00000000);
                if instruction_rv64.4 == 0 {
                    primary_table_rv64[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴_00
                }
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥 => {
                assert_eq!(instruction_rv64.3 & 0x00000020, 0x00000020);
                if instruction_rv64.4 == 0 {
                    primary_table_rv64[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥_00
                }
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔡𝔰𝔭_𝔩𝔬𝔴 => {
                assert_eq!(instruction_rv64.3 & 0x00000020, 0x00000000);
                if instruction_rv64.4 == 0 {
                    primary_table_rv64[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔡𝔰𝔭_𝔩𝔬𝔴_00
                }
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥 => {
                assert_eq!(instruction_rv64.3 & 0x00000020, 0x00000020);
                if instruction_rv64.4 == 0 {
                    primary_table_rv64[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥_00
                }
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔪𝔳 => {
                assert_eq!(instruction_rv64.3, 0);
                if instruction_rv64.4 == 0 {
                    primary_table_rv64[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔪𝔳_00
                }
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡 => {
                assert_eq!(instruction_rv64.3, 0);
                if instruction_rv64.4 == 0 {
                    primary_table_rv64[index].1 = 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡_00
                }
                assert_eq!(instruction_rv64.4 & -0x00000020, 0);
                primary_table_rv64[index].0 = instruction_rv64.4 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔣𝔰𝔡𝔰𝔭
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔰𝔴𝔰𝔭
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔰𝔡𝔰𝔭 => {
                assert_eq!(instruction_rv64.3 & -0x00000040, 0);
                assert_eq!(instruction_rv64.4, 0);
                primary_table_rv64[index].0 = instruction_rv64.3 as i8;
            }
            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔡𝔰𝔭_𝔩𝔬𝔴_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔪𝔳_00
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔲𝔫𝔯𝔢𝔠𝔬𝔤𝔫𝔦𝔷𝔢𝔡
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_0
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_1
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_2
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_3
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_4
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_5
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_6
            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_7
                => panic!("Unsupported enum value {:?}!", instruction_rv64.2),
        }
        let mut initialize_secondary_table: bool = false;
        for ignored_byte in u8::MIN..u8::MAX {
            let encoded_alt_parcel =
                ((encoded_byte & 0xfc) as u16) << 8 | (ignored_byte as u16) << 2 | (encoded_byte & 0x03) as u16;
            let alt_instruction_rv32 = slow_decode_compressed_instruction_rv32i(encoded_alt_parcel);
            let alt_instruction_rv64 = slow_decode_compressed_instruction_rv64i(encoded_alt_parcel);
            if instruction_rv32.2 != alt_instruction_rv32.2 {
                println!(
                    "rv32: 0b{:016b} {} {:?}",
                    encoded_parcel, instruction_rv32.0, instruction_rv32.2
                );
                println!(
                    "rv32: 0b{:016b} {} {:?}",
                    encoded_alt_parcel, alt_instruction_rv32.0, alt_instruction_rv32.2
                );
            }
            match primary_table_rv32[index].1 {
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 => {
                    if secondary_table_addi.is_none() {
                        secondary_table_addi = Some([(0i8, 0i8); u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_addi) = secondary_table_addi else {
                      panic!("Internal logic error");
                    };
                    let rd;
                    if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::𝔫𝔬𝔭 = alt_instruction_rv32.0 {
                       rd = 0;
                    } else {
                        let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[0] else {
                            panic!("Internal logic error");
                        };
                        rd = op as i8;
                    };
                    if initialize_secondary_table {
                        secondary_table_addi[ignored_byte as usize].0 =
                            (alt_instruction_rv32.3 & 0b11111) as i8;
                        secondary_table_addi[ignored_byte as usize].1 = rd & 0b111;
                    } else {
                        assert_eq!(
                            alt_instruction_rv32.3,
                            if primary_table_rv32[index].1 as i8 & 1 == 1 { -0x20 } else { 0 }
                                | (secondary_table_addi[ignored_byte as usize].0 as i32)
                        );
                        assert_eq!(rd, primary_table_rv32[index].0 | secondary_table_addi[ignored_byte as usize].1);
                    }
                }
                // Must be after 𝔠_𝔞𝔡𝔡𝔦, see here: https://github.com/rust-lang/rust/issues/42333#issuecomment-306694398
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔪𝔳_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔪𝔳 => {
                    if secondary_table_add.is_none() {
                        secondary_table_add = Some([0i8; u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_addi) = secondary_table_addi else {
                      panic!("Internet logic error");
                    };
                    let Some(ref mut secondary_table_add) = secondary_table_add else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let rs;
                    if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::𝔢𝔟𝔯𝔢𝔞𝔨 | 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::𝔲𝔫𝔦𝔪𝔭 = alt_instruction_rv32.0 {
                       rd = 0;
                       rs = ((encoded_alt_parcel >> 2) & 0b11111) as i8;
                    } else {
                        let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[0] else {
                            panic!("Internal logic error");
                        };
                        if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::𝔧𝔞𝔩𝔯 = alt_instruction_rv32.0 {
                            rs = 0;
                            let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:op, 𝖽𝗂𝗌𝗉:_}) =
                                alt_instruction_rv32.1[1] else {
                                panic!("Internal logic error");
                            };
                            rd = op as i8;
                        } else {
                            rd = op as i8;
                            let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[2] else {
                                panic!("Internal logic error");
                            };
                            rs = op as i8;
                        }
                    }
                    if initialize_secondary_table {
                        secondary_table_add[ignored_byte as usize] = rs;
                    } else {
                        assert_eq!(rd, primary_table_rv32[index].0 | secondary_table_addi[ignored_byte as usize].1);
                        assert_eq!(rs, secondary_table_add[ignored_byte as usize]);
                    }
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡𝔦4𝔰𝔭𝔫 => {
                    if secondary_table_addi4spn.is_none() {
                        secondary_table_addi4spn = Some([(0i8, 0i8); u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_addi4spn) = secondary_table_addi4spn else {
                        panic!("Internal logic error");
                    };
                    let rd;
                    if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::𝔲𝔫𝔦𝔪𝔭 = alt_instruction_rv32.0 {
                       rd = (8 | (encoded_alt_parcel >> 2) & 0b111) as i8;
                    } else {
                        let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[0] else {
                            panic!("Internal logic error");
                        };
                        rd = op as i8;
                    };
                    if initialize_secondary_table {
                        secondary_table_addi4spn[ignored_byte as usize].0 =
                            ((alt_instruction_rv32.3 ^ (primary_table_rv32[index].0 as u8 as i32) << 2) >> 2) as i8;
                        secondary_table_addi4spn[ignored_byte as usize].1 = rd;
                    } else {
                        assert_eq!(
                            alt_instruction_rv32.3,
                            ((primary_table_rv32[index].0 as u8 as i32)
                                | (secondary_table_addi4spn[ignored_byte as usize].0 as u8 as i32))
                                << 2
                        );
                        assert_eq!(rd, secondary_table_addi4spn[ignored_byte as usize].1);
                    }
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔩𝔡 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔰𝔡 => {
                    if secondary_table_ld.is_none() {
                        secondary_table_ld = Some([(0u8, 0i8, 0i8); u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_ld) = secondary_table_ld else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[0] else {
                        panic!("Internal logic error");
                    };
                    rd = op as i8;
                    let rs = match alt_instruction_rv32.1[1] {
                        𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:op, 𝖽𝗂𝗌𝗉:_}) => op as i8,
                        𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:op, 𝖽𝗂𝗌𝗉:_}) => op as i8,
                        _ => panic!("Internal logic error {:?}", alt_instruction_rv32.1),
                    };
                    if initialize_secondary_table {
                        secondary_table_ld[ignored_byte as usize].0 =
                            (alt_instruction_rv32.3 ^ (primary_table_rv32[index].0 as i32)) as u8;
                        secondary_table_ld[ignored_byte as usize].1 = rd;
                        secondary_table_ld[ignored_byte as usize].2 = rs;
                    } else {
                        assert_eq!(
                            alt_instruction_rv32.3,
                            ((primary_table_rv32[index].0 as i32) | (secondary_table_ld[ignored_byte as usize].0 as i32))
                        );
                        assert_eq!(rd, secondary_table_ld[ignored_byte as usize].1);
                        assert_eq!(rs, secondary_table_ld[ignored_byte as usize].2);
                    }
                }
                // Must be after 𝔠_𝔣𝔩𝔡, see here: https://github.com/rust-lang/rust/issues/42333#issuecomment-306694398
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔫𝔡_𝔬𝔯_𝔰𝔲𝔟_𝔵𝔬𝔯 => {
                    let Some(ref mut secondary_table_ld) = secondary_table_ld else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[0] else {
                        panic!("Internal logic error");
                    };
                    rd = op as i8;
                    let rs;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[2] else {
                        panic!("Internal logic error");
                    };
                    rs = op as i8;
                    assert_eq!(rd, secondary_table_ld[ignored_byte as usize].2);
                    assert_eq!(rs, secondary_table_ld[ignored_byte as usize].1);
                }
                // Must be after 𝔠_𝔣𝔩𝔡, see here: https://github.com/rust-lang/rust/issues/42333#issuecomment-306694398
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔫𝔡𝔦 => {
                    let Some(ref mut secondary_table_ld) = secondary_table_ld else {
                      panic!("Internet logic error");
                    };
                    let Some(ref mut secondary_table_addi) = secondary_table_addi else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[0] else {
                        panic!("Internal logic error");
                    };
                    rd = op as i8;
                    assert_eq!(
                        alt_instruction_rv32.3,
                        (primary_table_rv32[index].0 | secondary_table_addi[ignored_byte as usize].0) as i32,
                    );
                    assert_eq!(rd, secondary_table_ld[ignored_byte as usize].2);
                }
                // Must be after 𝔠_𝔣𝔩𝔡, see here: https://github.com/rust-lang/rust/issues/42333#issuecomment-306694398
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔟𝔢𝔮𝔷 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔟𝔫𝔢𝔷 => {
                    if secondary_table_beqz.is_none() {
                        secondary_table_beqz = Some([0i8; u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_beqz) = secondary_table_beqz else {
                        panic!("Internal logic error");
                    };
                    let Some(ref mut secondary_table_ld) = secondary_table_ld else {
                      panic!("Internet logic error");
                    };
                    let rs;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[0] else {
                        panic!("Internal logic error");
                    };
                    rs = op as i8;
                    if initialize_secondary_table {
                        secondary_table_beqz[ignored_byte as usize] =
                            ((alt_instruction_rv32.3 ^ (primary_table_rv32[index].0 as i32) << 1) >> 1) as i8;
                    } else {
                        assert_eq!(
                            alt_instruction_rv32.3,
                            ((primary_table_rv32[index].0 as i32)
                                | (secondary_table_beqz[ignored_byte as usize] as i32))
                                << 1
                        );
                        assert_eq!(rs, secondary_table_ld[ignored_byte as usize].2);
                    }
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔩𝔡𝔰𝔭_𝔩𝔬𝔴 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥 => {
                    if secondary_table_ldsp.is_none() {
                        secondary_table_ldsp = Some([0i8; u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_addi) = secondary_table_addi else {
                      panic!("Internal logic error");
                    };
                    let Some(ref mut secondary_table_ldsp) = secondary_table_ldsp else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[0] else {
                        panic!("Internal logic error");
                    };
                    rd = op as i8;
                    if initialize_secondary_table {
                        secondary_table_ldsp[ignored_byte as usize] = (alt_instruction_rv32.3 >> 3) as i8 ;
                    } else {
                        assert_eq!(
                            alt_instruction_rv32.3,
                            if primary_table_rv32[index].1 as i8 & 1 == 1 { 0x20 } else { 0 }
                                | (secondary_table_ldsp[ignored_byte as usize] as i32) << 3,
                        );
                        assert_eq!(rd, primary_table_rv32[index].0 | secondary_table_addi[ignored_byte as usize].1);
                    }
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔩𝔴 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔰𝔴 => {
                    if secondary_table_lw.is_none() {
                        secondary_table_lw = Some([(0i8, 0i8); u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_ld) = secondary_table_ld else {
                      panic!("Internet logic error");
                    };
                    let Some(ref mut secondary_table_lw) = secondary_table_lw else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[0] else {
                        panic!("Internal logic error");
                    };
                    rd = op as i8;
                    let rs = match alt_instruction_rv32.1[1] {
                        𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:op, 𝖽𝗂𝗌𝗉:_}) => op as i8,
                        𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:op, 𝖽𝗂𝗌𝗉:_}) => op as i8,
                        _ => panic!("Internal logic error {:?}", alt_instruction_rv32.1),
                    };
                    if initialize_secondary_table {
                        secondary_table_lw[ignored_byte as usize].0 =
                            (alt_instruction_rv32.3 ^ (primary_table_rv32[index].0 as i32)) as i8;
                        secondary_table_lw[ignored_byte as usize].1 = rd;
                    } else {
                        assert_eq!(
                            alt_instruction_rv32.3,
                            ((primary_table_rv32[index].0 as i32) | (secondary_table_lw[ignored_byte as usize].0 as i32))
                        );
                        assert_eq!(rd, secondary_table_lw[ignored_byte as usize].1);
                        assert_eq!(rs, secondary_table_ld[ignored_byte as usize].2);
                    }
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔩𝔴𝔰𝔭_𝔩𝔬𝔴
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥 => {
                    if secondary_table_lwsp.is_none() {
                        secondary_table_lwsp = Some([0i8; u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_addi) = secondary_table_addi else {
                      panic!("Internal logic error");
                    };
                    let Some(ref mut secondary_table_lwsp) = secondary_table_lwsp else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::𝔲𝔫𝔦𝔪𝔭 = alt_instruction_rv32.0 {
                        rd = 0;
                    } else {
                        rd = match alt_instruction_rv32.1[0] {
                            𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) => op as i8,
                            𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) => op as i8,
                            _ => panic!("Internal logic error"),
                        };
                    }
                    if initialize_secondary_table {
                        secondary_table_lwsp[ignored_byte as usize] = alt_instruction_rv32.3 as i8;
                    } else {
                        assert_eq!(
                            alt_instruction_rv32.3,
                            if primary_table_rv32[index].1 as i8 & 1 == 1 { 0x20 } else { 0 }
                                | secondary_table_lwsp[ignored_byte as usize] as u8 as i32,
                        );
                        assert_eq!(rd, primary_table_rv32[index].0 | secondary_table_addi[ignored_byte as usize].1);
                    }
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔰𝔡𝔰𝔭 => {
                    if secondary_table_sdsp.is_none() {
                        secondary_table_sdsp = Some([0i8; u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_add) = secondary_table_add else {
                      panic!("Internal logic error");
                    };
                    let Some(ref mut secondary_table_sdsp) = secondary_table_sdsp else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[0] else {
                        panic!("Internal logic error");
                    };
                    rd = op as i8;
                    if initialize_secondary_table {
                        secondary_table_sdsp[ignored_byte as usize] =
                           ((alt_instruction_rv32.3 ^ primary_table_rv32[index].0 as i32) >> 4) as i8;
                    } else {
                        assert_eq!(
                            alt_instruction_rv32.3,
                            primary_table_rv32[index].0 as i32 | (secondary_table_sdsp[ignored_byte as usize] as i32) << 4,
                        );
                        assert_eq!(rd, secondary_table_add[ignored_byte as usize]);
                    }
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔣𝔰𝔴𝔰𝔭 |
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔰𝔴𝔰𝔭 => {
                    if secondary_table_swsp.is_none() {
                        secondary_table_swsp = Some([0i8; u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_add) = secondary_table_add else {
                      panic!("Internal logic error");
                    };
                    let Some(ref mut secondary_table_swsp) = secondary_table_swsp else {
                      panic!("Internet logic error");
                    };
                    let rd = match alt_instruction_rv32.1[0] {
                        𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) => op as i8,
                        𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) => op as i8,
                        _ => panic!("Internal logic error"),
                    };
                    if initialize_secondary_table {
                        secondary_table_swsp[ignored_byte as usize] =
                           (alt_instruction_rv32.3 ^ primary_table_rv32[index].0 as i32) as i8;
                    } else {
                        assert_eq!(
                            alt_instruction_rv32.3,
                            (primary_table_rv32[index].0 | secondary_table_swsp[ignored_byte as usize]) as u8 as i32,
                        );
                        assert_eq!(rd, secondary_table_add[ignored_byte as usize]);
                    }
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔧 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔧𝔞𝔩 => {
                    if secondary_table_jal.is_none() {
                        secondary_table_jal = Some([0i16; u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_jal) = secondary_table_jal else {
                      panic!("Internet logic error");
                    };
                    if initialize_secondary_table {
                        secondary_table_jal[ignored_byte as usize] =
                            (alt_instruction_rv32.3 ^ (primary_table_rv32[index].0 as i32) << 4) as i16;
                    } else {
                        assert_eq!(
                            alt_instruction_rv32.3,
                            (primary_table_rv32[index].0 as i32) << 4| secondary_table_jal[ignored_byte as usize] as i32
                        );
                    }
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 => {
                    if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::𝔞𝔡𝔡𝔦 = alt_instruction_rv32.0 {
                        if secondary_table_addi16sp.is_none() {
                            secondary_table_addi16sp = Some([(0i8, 0i8); u8::MAX as usize + 1]);
                            initialize_secondary_table = true;
                        }
                        let Some(ref mut secondary_table_addi16sp) = secondary_table_addi16sp else {
                            panic!("Internal logic error");
                        };
                        secondary_table_addi16sp[0x40].1 = 2; // Hack to deal with unimp.
                        let rd;
                        let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[0] else {
                            panic!("Internal logic error");
                        };
                        rd = op as i8;
                        if initialize_secondary_table {
                           secondary_table_addi16sp[ignored_byte as usize].0 =
                                ((alt_instruction_rv32.3 & 0b111110000) >> 4) as i8;
                           secondary_table_addi16sp[ignored_byte as usize].1 = rd & 0b111;
                        } else {
                            assert_eq!(
                                alt_instruction_rv32.3,
                                if primary_table_rv32[index].1 as i8 & 1 == 1 { -0x200 } else { 0 }
                                    | (secondary_table_addi16sp[ignored_byte as usize].0 as i32) << 4
                            );
                            assert_eq!(rd, primary_table_rv32[index].0 | secondary_table_addi16sp[ignored_byte as usize].1);
                        }
                    } else {
                        let Some(ref mut secondary_table_addi) = secondary_table_addi else {
                          panic!("Internal logic error");
                        };
                        if let Some(ref mut secondary_table_addi16sp) = secondary_table_addi16sp {
                            secondary_table_addi16sp[ignored_byte as usize].1 = secondary_table_addi[ignored_byte as usize].1;
                            let rd;
                            if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::𝔲𝔫𝔦𝔪𝔭 = alt_instruction_rv32.0 {
                                rd = alt_instruction_rv32.4 as i8;
                            } else {
                                let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[0] else {
                                    panic!("Internal logic error");
                                };
                                rd = op as i8;
                            };
                            assert_eq!(
                                alt_instruction_rv32.3,
                                if primary_table_rv32[index].1 as i8 & 1 == 1 { -0x20000 } else { 0 }
                                    | (secondary_table_addi[ignored_byte as usize].0 as i32) << 12,
                            );
                            assert_eq!(rd, primary_table_rv32[index].0 | secondary_table_addi16sp[ignored_byte as usize].1);
                        };
                    }
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔩𝔴 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔰𝔴 => {
                    if secondary_table_lw.is_none() {
                        secondary_table_lw = Some([(0i8, 0i8); u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_lw) = secondary_table_lw else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[0] else {
                        panic!("Internal logic error");
                    };
                    rd = op as i8;
                    if initialize_secondary_table {
                        secondary_table_lw[ignored_byte as usize].0 =
                            (alt_instruction_rv32.3 ^ (primary_table_rv32[index].0 as i32)) as i8;
                        secondary_table_lw[ignored_byte as usize].1 = rd;
                    } else {
                        assert_eq!(
                            alt_instruction_rv32.3,
                            ((primary_table_rv32[index].0 as i32) | (secondary_table_lw[ignored_byte as usize].0 as i32))
                        );
                        assert_eq!(rd, secondary_table_lw[ignored_byte as usize].1);
                    }
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔰𝔯𝔞𝔦_𝔩𝔬𝔴
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔰𝔯𝔞𝔦_𝔥𝔦𝔤𝔥
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔰𝔯𝔩𝔦_𝔩𝔬𝔴
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔰𝔯𝔩𝔦_𝔥𝔦𝔤𝔥 => {
                    let Some(ref mut secondary_table_ld) = secondary_table_ld else {
                      panic!("Internet logic error");
                    };
                    let Some(ref mut secondary_table_addi) = secondary_table_addi else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::𝔲𝔫𝔦𝔪𝔭 = alt_instruction_rv32.0 {
                       rd = (8 | (encoded_alt_parcel >> 7) & 0b111) as i8;
                    } else {
                        let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[0] else {
                            panic!("Internal logic error");
                        };
                        rd = op as i8;
                    }
                    assert_eq!(
                        alt_instruction_rv32.3,
                        if primary_table_rv32[index].1 as i8 & 1 == 1 { 0x20 } else { 0 }
                            | (secondary_table_addi[ignored_byte as usize].0 as i32),
                    );
                    assert_eq!(rd, secondary_table_ld[ignored_byte as usize].2);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔰𝔩𝔩𝔦_𝔩𝔬𝔴 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔰𝔩𝔩𝔦_𝔥𝔦𝔤𝔥 => {
                    let Some(ref mut secondary_table_addi) = secondary_table_addi else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::𝔲𝔫𝔦𝔪𝔭 = alt_instruction_rv32.0 {
                       rd = ((encoded_alt_parcel >> 7) & 0b11111) as i8;
                    } else {
                        let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv32.1[0] else {
                            panic!("Internal logic error");
                        };
                        rd = op as i8;
                    }
                    assert_eq!(
                        alt_instruction_rv32.3,
                        if primary_table_rv32[index].1 as i8 & 1 == 1 { 0x20 } else { 0 }
                            | secondary_table_addi[ignored_byte as usize].0 as i32,
                    );
                    assert_eq!(rd, primary_table_rv32[index].0 | secondary_table_addi[ignored_byte as usize].1);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔞𝔡𝔡𝔴_𝔰𝔲𝔟𝔴 // We recognize it in main opcode table, but it's not support in rv32 mode.
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔯𝔢𝔰𝔢𝔯𝔳𝔢𝔡 => (),
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔠_𝔲𝔫𝔯𝔢𝔠𝔬𝔤𝔫𝔦𝔷𝔢𝔡
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_0
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_1
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_2
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_3
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_4
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_5
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_6
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_7
                    => panic!("Unsupported enum value {:?}!", primary_table_rv32[index].1),
            }
            match primary_table_rv64[index].1 {
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔪𝔳_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔪𝔳 => {
                    if secondary_table_add.is_none() {
                        secondary_table_add = Some([0i8; u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_addi) = secondary_table_addi else {
                      panic!("Internet logic error");
                    };
                    let Some(ref mut secondary_table_add) = secondary_table_add else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let rs;
                    if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::𝔢𝔟𝔯𝔢𝔞𝔨 | 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::𝔲𝔫𝔦𝔪𝔭 = alt_instruction_rv64.0 {
                       rd = 0;
                       rs = ((encoded_alt_parcel >> 2) & 0b11111) as i8;
                    } else {
                        let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                            panic!("Internal logic error");
                        };
                        if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::𝔧𝔞𝔩𝔯 = alt_instruction_rv64.0 {
                            rs = 0;
                            let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:op, 𝖽𝗂𝗌𝗉:_}) =
                                alt_instruction_rv64.1[1] else {
                                panic!("Internal logic error");
                            };
                            rd = op as i8;
                        } else {
                            rd = op as i8;
                            let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[2] else {
                                panic!("Internal logic error");
                            };
                            rs = op as i8;
                        }
                    }
                    if initialize_secondary_table {
                        secondary_table_add[ignored_byte as usize] = rs;
                    } else {
                        assert_eq!(rd, primary_table_rv64[index].0 | secondary_table_addi[ignored_byte as usize].1);
                        assert_eq!(rs, secondary_table_add[ignored_byte as usize]);
                    }
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 => {
                    let Some(ref mut secondary_table_addi) = secondary_table_addi else {
                      panic!("Internal logic error");
                    };
                    let rd;
                    if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::𝔫𝔬𝔭 | 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::𝔲𝔫𝔦𝔪𝔭 = alt_instruction_rv64.0 {
                       rd = 0;
                    } else {
                        let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                            panic!("Internal logic error");
                        };
                        rd = op as i8;
                    };
                    assert_eq!(
                        alt_instruction_rv64.3,
                        if primary_table_rv64[index].1 as i8 & 1 == 1 { -0x20 } else { 0 }
                            | (secondary_table_addi[ignored_byte as usize].0 as i32)
                    );
                    assert_eq!(rd, primary_table_rv64[index].0 | secondary_table_addi[ignored_byte as usize].1);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔦4𝔰𝔭𝔫 => {
                    let Some(ref mut secondary_table_addi4spn) = secondary_table_addi4spn else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::𝔲𝔫𝔦𝔪𝔭 = alt_instruction_rv64.0 {
                       rd = (8 | (encoded_alt_parcel >> 2) & 0b111) as i8;
                    } else {
                        let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                            panic!("Internal logic error");
                        };
                        rd = op as i8;
                    };
                    assert_eq!(
                        alt_instruction_rv64.3,
                        ((primary_table_rv64[index].0 as u8 as i32)
                            | (secondary_table_addi4spn[ignored_byte as usize].0 as u8 as i32))
                            << 2
                    );
                    assert_eq!(rd, secondary_table_addi4spn[ignored_byte as usize].1);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔫𝔡_𝔬𝔯_𝔰𝔲𝔟_𝔵𝔬𝔯
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔡𝔡𝔴_𝔰𝔲𝔟𝔴 => {
                    let Some(ref mut secondary_table_ld) = secondary_table_ld else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let rs;
                    if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::𝔲𝔫𝔦𝔪𝔭 = alt_instruction_rv64.0 {
                       rd = (8 | (encoded_alt_parcel >> 7) & 0b111) as i8;
                       rs = (8 | (encoded_alt_parcel >> 2) & 0b111) as i8;
                    } else {
                        let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                            panic!("Internal logic error");
                        };
                        rd = op as i8;
                        let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[2] else {
                            panic!("Internal logic error");
                        };
                        rs = op as i8;
                    }
                    assert_eq!(rd, secondary_table_ld[ignored_byte as usize].2);
                    assert_eq!(rs, secondary_table_ld[ignored_byte as usize].1);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔞𝔫𝔡𝔦 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔰𝔯𝔞𝔦 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔰𝔯𝔩𝔦 => {
                    let Some(ref mut secondary_table_ld) = secondary_table_ld else {
                      panic!("Internet logic error");
                    };
                    let Some(ref mut secondary_table_addi) = secondary_table_addi else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::𝔲𝔫𝔦𝔪𝔭 = alt_instruction_rv64.0 {
                       rd = (8 | (encoded_alt_parcel >> 7) & 0b111) as i8;
                    } else {
                        let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                            panic!("Internal logic error");
                        };
                        rd = op as i8;
                    }
                    assert_eq!(
                        alt_instruction_rv64.3,
                        (primary_table_rv64[index].0 | secondary_table_addi[ignored_byte as usize].0) as i32,
                    );
                    assert_eq!(rd, secondary_table_ld[ignored_byte as usize].2);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔟𝔢𝔮𝔷 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔟𝔫𝔢𝔷 => {
                    let Some(ref mut secondary_table_beqz) = secondary_table_beqz else {
                        panic!("Internal logic error");
                    };
                    let Some(ref mut secondary_table_ld) = secondary_table_ld else {
                      panic!("Internet logic error");
                    };
                    let rs;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                        panic!("Internal logic error");
                    };
                    rs = op as i8;
                    assert_eq!(
                        alt_instruction_rv64.3,
                        ((primary_table_rv64[index].0 as i32)
                            | (secondary_table_beqz[ignored_byte as usize] as i32))
                            << 1
                    );
                    assert_eq!(rs, secondary_table_ld[ignored_byte as usize].2);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔣𝔩𝔡 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔣𝔰𝔡 => {
                    let Some(ref mut secondary_table_ld) = secondary_table_ld else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::𝔲𝔫𝔦𝔪𝔭 = alt_instruction_rv64.0 {
                       rd = (8 | (encoded_alt_parcel >> 2) & 0b111) as i8;
                    } else {
                        let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                            panic!("Internal logic error");
                        };
                        rd = op as i8;
                    };
                    let rs = match alt_instruction_rv64.1[1] {
                        𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:op, 𝖽𝗂𝗌𝗉:_}) => op as i8,
                        𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:op, 𝖽𝗂𝗌𝗉:_}) => op as i8,
                        _ => panic!("Internal logic error {:?}", alt_instruction_rv64.1),
                    };
                    assert_eq!(
                        alt_instruction_rv64.3,
                        ((primary_table_rv64[index].0 as i32) | (secondary_table_ld[ignored_byte as usize].0 as u8 as i32))
                    );
                    assert_eq!(rd, secondary_table_ld[ignored_byte as usize].1);
                    assert_eq!(rs, secondary_table_ld[ignored_byte as usize].2);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔣𝔩𝔡𝔰𝔭_𝔩𝔬𝔴 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔣𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥 => {
                    if secondary_table_ldsp.is_none() {
                        secondary_table_ldsp = Some([0i8; u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_addi) = secondary_table_addi else {
                      panic!("Internal logic error");
                    };
                    let Some(ref mut secondary_table_ldsp) = secondary_table_ldsp else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                        panic!("Internal logic error");
                    };
                    rd = op as i8;
                    assert_eq!(
                        alt_instruction_rv64.3,
                        if primary_table_rv64[index].1 as i8 & 1 == 1 { 0x20 } else { 0 }
                            | (secondary_table_ldsp[ignored_byte as usize] as i32) << 3,
                    );
                    assert_eq!(rd, primary_table_rv64[index].0 | secondary_table_addi[ignored_byte as usize].1);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔣𝔰𝔡𝔰𝔭 => {
                    let Some(ref mut secondary_table_add) = secondary_table_add else {
                      panic!("Internal logic error");
                    };
                    let Some(ref mut secondary_table_sdsp) = secondary_table_sdsp else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                        panic!("Internal logic error");
                    };
                    rd = op as i8;
                    assert_eq!(
                        alt_instruction_rv64.3,
                        primary_table_rv64[index].0 as i32 | (secondary_table_sdsp[ignored_byte as usize] as i32) << 4,
                    );
                    assert_eq!(rd, secondary_table_add[ignored_byte as usize]);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔧 => {
                    let Some(ref mut secondary_table_jal) = secondary_table_jal else {
                      panic!("Internet logic error");
                    };
                    assert_eq!(
                        alt_instruction_rv64.3,
                        (primary_table_rv64[index].0 as i32) << 4| secondary_table_jal[ignored_byte as usize] as i32
                    );
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔡 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔰𝔡 => {
                    let Some(ref mut secondary_table_ld) = secondary_table_ld else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                        panic!("Internal logic error");
                    };
                    rd = op as i8;
                    let rs = match alt_instruction_rv64.1[1] {
                        𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:op, 𝖽𝗂𝗌𝗉:_}) => op as i8,
                        𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:op, 𝖽𝗂𝗌𝗉:_}) => op as i8,
                        _ => panic!("Internal logic error {:?}", alt_instruction_rv64.1),
                    };
                    assert_eq!(
                        alt_instruction_rv64.3,
                        ((primary_table_rv64[index].0 as i32) | (secondary_table_ld[ignored_byte as usize].0 as i32))
                    );
                    assert_eq!(rd, secondary_table_ld[ignored_byte as usize].1);
                    assert_eq!(rs, secondary_table_ld[ignored_byte as usize].2);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔡𝔰𝔭_𝔩𝔬𝔴_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔡𝔰𝔭_𝔩𝔬𝔴
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥 => {
                    if secondary_table_ldsp.is_none() {
                        secondary_table_ldsp = Some([0i8; u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_addi) = secondary_table_addi else {
                      panic!("Internal logic error");
                    };
                    let Some(ref mut secondary_table_ldsp) = secondary_table_ldsp else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::𝔲𝔫𝔦𝔪𝔭 = alt_instruction_rv64.0 {
                        rd = 0;
                    } else {
                        let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                            panic!("Internal logic error");
                        };
                        rd = op as i8;
                    }
                    assert_eq!(
                        alt_instruction_rv64.3,
                        if primary_table_rv64[index].1 as i8 & 1 == 1 { 0x20 } else { 0 }
                            | (secondary_table_ldsp[ignored_byte as usize] as i32) << 3,
                    );
                    assert_eq!(rd, primary_table_rv64[index].0 | secondary_table_addi[ignored_byte as usize].1);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 => {
                    if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::𝔞𝔡𝔡𝔦 = alt_instruction_rv64.0 {
                        let Some(ref mut secondary_table_addi16sp) = secondary_table_addi16sp else {
                            panic!("Internal logic error");
                        };
                        let rd;
                        let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                            panic!("Internal logic error");
                        };
                        rd = op as i8;
                        assert_eq!(
                            alt_instruction_rv64.3,
                            if primary_table_rv64[index].1 as i8 & 1 == 1 { -0x200 } else { 0 }
                                | (secondary_table_addi16sp[ignored_byte as usize].0 as i32) << 4
                        );
                        assert_eq!(rd, primary_table_rv64[index].0 | secondary_table_addi16sp[ignored_byte as usize].1);
                    } else {
                        let Some(ref mut secondary_table_addi) = secondary_table_addi else {
                          panic!("Internal logic error");
                        };
                        if let Some(ref mut secondary_table_addi16sp) = secondary_table_addi16sp {
                            secondary_table_addi16sp[ignored_byte as usize].1 = secondary_table_addi[ignored_byte as usize].1;
                            let rd;
                            if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::𝔲𝔫𝔦𝔪𝔭 = alt_instruction_rv64.0 {
                                rd = alt_instruction_rv64.4 as i8;
                            } else {
                                let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                                    panic!("Internal logic error");
                                };
                                rd = op as i8;
                            };
                            assert_eq!(
                                alt_instruction_rv64.3,
                                if primary_table_rv64[index].1 as i8 & 1 == 1 { -0x20000 } else { 0 }
                                    | (secondary_table_addi[ignored_byte as usize].0 as i32) << 12
                            );
                            assert_eq!(rd, primary_table_rv64[index].0 | secondary_table_addi16sp[ignored_byte as usize].1);
                        };
                    }
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔴 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔰𝔴 => {
                    let Some(ref mut secondary_table_ld) = secondary_table_ld else {
                      panic!("Internet logic error");
                    };
                    let Some(ref mut secondary_table_lw) = secondary_table_lw else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                        panic!("Internal logic error");
                    };
                    rd = op as i8;
                    let rs = match alt_instruction_rv64.1[1] {
                        𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:op, 𝖽𝗂𝗌𝗉:_}) => op as i8,
                        𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:op, 𝖽𝗂𝗌𝗉:_}) => op as i8,
                        _ => panic!("Internal logic error {:?}", alt_instruction_rv64.1),
                    };
                    assert_eq!(
                        alt_instruction_rv64.3,
                        ((primary_table_rv64[index].0 as i32) | (secondary_table_lw[ignored_byte as usize].0 as i32))
                    );
                    assert_eq!(rd, secondary_table_lw[ignored_byte as usize].1);
                    assert_eq!(rs, secondary_table_ld[ignored_byte as usize].2);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥_00
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥 => {
                    let Some(ref mut secondary_table_addi) = secondary_table_addi else {
                      panic!("Internal logic error");
                    };
                    let Some(ref mut secondary_table_lwsp) = secondary_table_lwsp else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    if let 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::𝔲𝔫𝔦𝔪𝔭 = alt_instruction_rv64.0 {
                        rd = 0;
                    } else {
                        let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                            panic!("Internal logic error");
                        };
                        rd = op as i8;
                    }
                    assert_eq!(
                        alt_instruction_rv64.3,
                        if primary_table_rv64[index].1 as i8 & 1 == 1 { 0x20 } else { 0 }
                            | secondary_table_lwsp[ignored_byte as usize] as u8 as i32,
                    );
                    assert_eq!(rd, primary_table_rv64[index].0 | secondary_table_addi[ignored_byte as usize].1);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔰𝔩𝔩𝔦_𝔩𝔬𝔴 | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔰𝔩𝔩𝔦_𝔥𝔦𝔤𝔥 => {
                    let Some(ref mut secondary_table_addi) = secondary_table_addi else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                        panic!("Internal logic error");
                    };
                    rd = op as i8;
                    assert_eq!(
                        alt_instruction_rv64.3,
                        if primary_table_rv64[index].1 as i8 & 1 == 1 { 0x20 } else { 0 }
                            | secondary_table_addi[ignored_byte as usize].0 as i32,
                    );
                    assert_eq!(rd, primary_table_rv64[index].0 | secondary_table_addi[ignored_byte as usize].1);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔰𝔡𝔰𝔭 => {
                    if secondary_table_sdsp.is_none() {
                        secondary_table_sdsp = Some([0i8; u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_add) = secondary_table_add else {
                      panic!("Internal logic error");
                    };
                    let Some(ref mut secondary_table_sdsp) = secondary_table_sdsp else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                        panic!("Internal logic error");
                    };
                    rd = op as i8;
                    assert_eq!(
                        alt_instruction_rv64.3,
                        primary_table_rv64[index].0 as i32 | (secondary_table_sdsp[ignored_byte as usize] as i32) << 4,
                    );
                    assert_eq!(rd, secondary_table_add[ignored_byte as usize]);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔰𝔴𝔰𝔭 => {
                    if secondary_table_swsp.is_none() {
                        secondary_table_swsp = Some([0i8; u8::MAX as usize + 1]);
                        initialize_secondary_table = true;
                    }
                    let Some(ref mut secondary_table_add) = secondary_table_add else {
                      panic!("Internal logic error");
                    };
                    let Some(ref mut secondary_table_swsp) = secondary_table_swsp else {
                      panic!("Internet logic error");
                    };
                    let rd;
                    let 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶::<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(op) = alt_instruction_rv64.1[0] else {
                        panic!("Internal logic error");
                    };
                    rd = op as i8;
                    assert_eq!(
                        alt_instruction_rv64.3,
                        (primary_table_rv64[index].0 | secondary_table_swsp[ignored_byte as usize]) as u8 as i32,
                    );
                    assert_eq!(rd, secondary_table_add[ignored_byte as usize]);
                }
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔯𝔢𝔰𝔢𝔯𝔳𝔢𝔡 => (),
                𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔠_𝔲𝔫𝔯𝔢𝔠𝔬𝔤𝔫𝔦𝔷𝔢𝔡
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_0
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_1
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_2
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_3
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_4
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_5
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_6
                | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_7
                    => panic!("Unsupported enum value {:?}!", primary_table_rv32[index].1),
            }
            if instruction_rv64.2 != alt_instruction_rv64.2 {
                println!(
                    "rv64: 0b{:016b} {} {:?}",
                    encoded_parcel, instruction_rv64.0, instruction_rv64.2
                );
                println!(
                    "rv64: 0b{:016b} {} {:?}",
                    encoded_alt_parcel, alt_instruction_rv64.0, alt_instruction_rv64.2
                );
            }
        }
    }
    println!("{primary_table_rv32:?}");
    println!("{primary_table_rv64:?}");
}

// Actual decoder in main 𝔜𝔄ℭ𝔈 crate uses very fast table-based decoder.
// But since it's hard to fill these tables by hand we need slow-yet-obviously-correct one to fill these tables.
// But there are additional complexity, too: certain instructions are only supported in 𝔯𝔳32 or 𝔯𝔳64 mode!
// We are using the same 𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘 macro as 𝔜𝔄ℭ𝔈 crate to solve the issue.
macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖑𝖔𝖜_𝖉𝖊𝖈𝖔𝖉𝖊_𝖈𝖔𝖒𝖕𝖗𝖊𝖘𝖘𝖊𝖉_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓 {
    ($( $𝓼𝓵𝓸𝔀_𝓭𝓮𝓬𝓸𝓭𝓮_𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓼𝓮𝓭_𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        fn $𝓼𝓵𝓸𝔀_𝓭𝓮𝓬𝓸𝓭𝓮_𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓼𝓮𝓭_𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷:ident
            -> ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮:ident,
                $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮:ident<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮:ident as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮:ident>,
                $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹:ident)
        {
            $($𝓮𝔁𝓽𝓻𝓪_𝓬𝓸𝓭𝓮:tt)*
        }
      )*) => {
        $(
            𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! {
                𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖑𝖔𝖜_𝖉𝖊𝖈𝖔𝖉𝖊_𝖈𝖔𝖒𝖕𝖗𝖊𝖘𝖘𝖊𝖉_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓! {
                    $𝓼𝓵𝓸𝔀_𝓭𝓮𝓬𝓸𝓭𝓮_𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓼𝓮𝓭_𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
                    fn $𝓼𝓵𝓸𝔀_𝓭𝓮𝓬𝓸𝓭𝓮_𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓼𝓮𝓭_𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷(machine_code: u16)
                        -> ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮, Vec<$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>, $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹, i32, i32)
                    {
                        $($𝓮𝔁𝓽𝓻𝓪_𝓬𝓸𝓭𝓮)*
                        match (machine_code >> 15,
                               machine_code >> 14 & 1,
                               machine_code >> 13 & 1,
                               machine_code >> 12 & 1,
                               machine_code >> 11 & 1,
                               machine_code >> 10 & 1,
                               machine_code >> 9 & 1,
                               machine_code >> 8 & 1,
                               machine_code >> 7 & 1,
                               machine_code >> 6 & 1,
                               machine_code >> 5 & 1,
                               machine_code >> 4 & 1,
                               machine_code >> 3 & 1,
                               machine_code >> 2 & 1,
                               machine_code >> 1 & 1,
                               machine_code & 1) {
                            (0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,_r2,_r1,_r0,  0,  0) =>
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔲𝔫𝔦𝔪𝔭,
                                 Vec::new(),
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔡𝔡𝔦4𝔰𝔭𝔫,
                                 0,
                                 0),
                            (0,  0,  0, i5, i4, i9, i8, i7, i6, i2, i3, r2, r1, r0,  0,  0) => {
                                let imm = i9<<9 | i8<<8 | i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3 | i2<<2;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔞𝔡𝔡𝔦,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵2.into()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(imm.unwrap())],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔡𝔡𝔦4𝔰𝔭𝔫,
                                 imm.unwrap().into(),
                                 0)
                            }
                            (0,  0,  1, i5, i4, i3, s2, s1, s0, i7, i6, r2, r1, r0,  0,  0) => {
                                let imm = i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                let rs1 = 8 | s2<<2 | s1<<1 | s0;
                                let rs1: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs1.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔣𝔩𝔡,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:rs1.unwrap(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔣𝔩𝔡,
                                 imm.unwrap().into(),
                                 0)
                            }
                            (0,  1,  0, i5, i4, i3, s2, s1, s0, i2, i6, r2, r1, r0,  0,  0) => {
                                let imm = i6<<6 | i5<<5 | i4<<4 | i3<<3 | i2<<2;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                let rs1 = 8 | s2<<2 | s1<<1 | s0;
                                let rs1: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs1.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔩𝔴,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:rs1.unwrap(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔴,
                                 imm.unwrap().into(),
                                 0)
                            }
                    Ξ𝔯𝔳32[  (0,  1,  1, i5, i4, i3, s2, s1, s0, i2, i6, r2, r1, r0,  0,  0) => {
                                let imm = i6<<6 | i5<<5 | i4<<4 | i3<<3 | i2<<2;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                let rs1 = 8 | s2<<2 | s1<<1 | s0;
                                let rs1: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs1.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔣𝔩𝔴,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:rs1.unwrap(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔣𝔩𝔴,
                                 imm.unwrap().into(),
                                 0)
                            }]
                    Ξ𝔯𝔳64[  (0,  1,  1, i5, i4, i3, s2, s1, s0, i7, i6, r2, r1, r0,  0,  0) => {
                                let imm = i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                let rs1 = 8 | s2<<2 | s1<<1 | s0;
                                let rs1: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs1.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔩𝔡,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:rs1.unwrap(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔡,
                                 imm.unwrap().into(),
                                 0)
                            }]
                            (1,  0,  0,  _,  _,  _,  _,  _,  _,  _,  _,  _,  _,  _,  0,  0) =>
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔲𝔫𝔦𝔪𝔭,
                                 Vec::new(),
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔯𝔢𝔰𝔢𝔯𝔳𝔢𝔡,
                                 0,
                                 0),
                            (1,  0,  1, i5, i4, i3, s2, s1, s0, i7, i6, r2, r1, r0,  0,  0) => {
                                let imm = i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                let rs1 = 8 | s2<<2 | s1<<1 | s0;
                                let rs1: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs1.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔣𝔰𝔡,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:rs1.unwrap(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔣𝔰𝔡,
                                 imm.unwrap().into(),
                                 0)
                            }
                            (1,  1,  0, i5, i4, i3, s2, s1, s0, i2, i6, r2, r1, r0,  0,  0) => {
                                let imm = i6<<6 | i5<<5 | i4<<4 | i3<<3 | i2<<2;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                let rs1 = 8 | s2<<2 | s1<<1 | s0;
                                let rs1: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs1.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔰𝔴,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:rs1.unwrap(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔰𝔴,
                                 imm.unwrap().into(),
                                 0)
                            }
                    Ξ𝔯𝔳32[  (1,  1,  1, i5, i4, i3, s2, s1, s0, i2, i6, r2, r1, r0,  0,  0) => {
                                let imm = i6<<6 | i5<<5 | i4<<4 | i3<<3 | i2<<2;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                let rs1 = 8 | s2<<2 | s1<<1 | s0;
                                let rs1: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs1.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔣𝔰𝔴,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:rs1.unwrap(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔣𝔰𝔴,
                                 imm.unwrap().into(),
                                 0)
                            }]
                    Ξ𝔯𝔳64[  (1,  1,  1, i5, i4, i3, s2, s1, s0, i7, i6, r2, r1, r0,  0,  0) => {
                                let imm = i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                let rs1 = 8 | s2<<2 | s1<<1 | s0;
                                let rs1: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs1.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔰𝔡,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:rs1.unwrap(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔰𝔡,
                                 imm.unwrap().into(),
                                 0)
                            }]
                            (0,  0,  0, i5,  0,  0,  0,  0,  0, i4, i3, i2, i1, i0,  0,  1) =>
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔫𝔬𝔭,
                                 Vec::new(),
                                 if i5 == 1 { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 } else { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 },
                                 (i5 as i32 * -1)<<5 | (i4<<4 | i3<<3 | i2<<2 | i1<<1 | i0) as i32,
                                 0),
                            (0,  0,  0, i5, r4, r3, r2, r1, r0, i4, i3, i2, i1, i0,  0,  1) => {
                                let imm = (i5 as i32 * -1)<<5 | (i4<<4 | i3<<3 | i2<<2 | i1<<1 | i0) as i32;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔞𝔡𝔡𝔦,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(imm.unwrap())],
                                 if i5 == 1 { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 } else { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 },
                                 imm.unwrap().into(),
                                 (r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0).into())
                            }
                    Ξ𝔯𝔳32[  (0,  0,  1,i11, i4, i9, i8,i10, i6, i7, i3, i2, i1, i5,  0,  1) => {
                                let imm = (i11 as i32 * -1)<<11 | (i10<<10 | i9<<9 | i8<<8 | i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3 | i2<<2 | i1<<1) as i32;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐣𝐮𝐦𝐩_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔧𝔞𝔩,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵1.into()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔧𝔲𝔪𝔭_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(imm.unwrap())],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔧𝔞𝔩,
                                 imm.unwrap().into(),
                                 0)
                            }]
                    Ξ𝔯𝔳64[  (0,  0,  1, i5,  0,  0,  0,  0,  0, i4, i3, i2, i1, i0,  0,  1) =>
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔲𝔫𝔦𝔪𝔭,
                                 Vec::new(),
                                 if i5 == 1 { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 } else { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 },
                                 (i5 as i32 * -1)<<5 | (i4<<4 | i3<<3 | i2<<2 | i1<<1 | i0) as i32,
                                 0),
                            (0,  0,  1, i5, r4, r3, r2, r1, r0, i4, i3, i2, i1, i0,  0,  1) => {
                                let imm = (i5 as i32 * -1)<<5 | (i4<<4 | i3<<3 | i2<<2 | i1<<1 | i0) as i32;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔞𝔡𝔡𝔦𝔴,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(imm.unwrap())],
                                 if i5 == 1 { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 } else { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔡𝔡𝔦𝔴_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 },
                                 imm.unwrap().into(),
                                 (r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0).into())
                            }]
                            (0,  1,  0, i5, r4, r3, r2, r1, r0, i4, i3, i2, i1, i0,  0,  1) => {
                                let imm = (i5 as i32 * -1)<<5 | (i4<<4 | i3<<3 | i2<<2 | i1<<1 | i0) as i32;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔞𝔡𝔡𝔦,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵0.into()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(imm.unwrap())],
                                 if i5 == 1 { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 } else { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 },
                                 imm.unwrap().into(),
                                 (r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0).into())
                            }
                            (0,  1,  1,  0, r4, r3, r2, r1, r0,  0,  0,  0,  0,  0,  0,  1) =>
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔲𝔫𝔦𝔪𝔭,
                                 Vec::new(),
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢,
                                 0,
                                 (r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0).into()),
                            (0,  1,  1, i9,  0,  0,  0,  1,  0, i4, i6, i8, i7, i5,  0,  1) => {
                                let imm = (i9 as i32 * -1)<<9 | (i8<<8 | i7<<7 | i6<<6 | i5<<5 | i4<<4) as i32;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔞𝔡𝔡𝔦,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵2.into()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵2.into()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(imm.unwrap())],
                                 if i9 == 1 { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 } else { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 },
                                 imm.unwrap().into(),
                                 2)
                            }
                            (0,  1,  1,i17, r4, r3, r2, r1, r0,i𝟭𝟲,i15,i14,i13,i12,  0,  1) => {
                                let imm = (i17 as i32 * -1)<<17 | (i𝟭𝟲 as i32)<<16 | (i15 as i32)<<15 | (i14 as i32)<<14 | (i13 as i32)<<13 | (i12 as i32)<<12;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐮𝐩𝐩𝐞𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔩𝔲𝔦,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔲𝔭𝔭𝔢𝔯_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(imm.unwrap())],
                                 if i17 == 1 { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 } else { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 },
                                 imm.unwrap().into(),
                                 (r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0).into())
                            }
                    Ξ𝔯𝔳32[  (1,  0,  0,  1,  0,  0,_r2,_r1,_r0, i4, i3, i2, i1, i0,  0,  1) =>
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔲𝔫𝔦𝔪𝔭,
                                 Vec::new(),
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔰𝔯𝔩𝔦_𝔥𝔦𝔤𝔥,
                                 (1<<5 | i4<<4 | i3<<3 | i2<<2 | i1<<1 | i0) as i32,
                                 0),]
                            (1,  0,  0, i5,  0,  0, r2, r1, r0, i4, i3, i2, i1, i0,  0,  1) => {
                                let imm = i5<<5 | i4<<4 | i3<<3 | i2<<2 | i1<<1 | i0;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐬𝐡𝐢𝐟𝐭_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔰𝔯𝔩𝔦,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔰𝔥𝔦𝔣𝔱_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(imm.unwrap())],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::Ξ𝔯𝔳32[𝔠_𝔰𝔯𝔩𝔦_𝔩𝔬𝔴]Ξ𝔯𝔳64[𝔠_𝔰𝔯𝔩𝔦],
                                 imm.unwrap().into(),
                                 0)
                            }
                    Ξ𝔯𝔳32[  (1,  0,  0,  1,  0,  1,_r2,_r1,_r0, i4, i3, i2, i1, i0,  0,  1) =>
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔲𝔫𝔦𝔪𝔭,
                                 Vec::new(),
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔰𝔯𝔞𝔦_𝔥𝔦𝔤𝔥,
                                 (1<<5 | i4<<4 | i3<<3 | i2<<2 | i1<<1 | i0) as i32,
                                 0),]
                            (1,  0,  0, i5,  0,  1, r2, r1, r0, i4, i3, i2, i1, i0,  0,  1) => {
                                let imm = i5<<5 | i4<<4 | i3<<3 | i2<<2 | i1<<1 | i0;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐬𝐡𝐢𝐟𝐭_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔰𝔯𝔞𝔦,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔰𝔥𝔦𝔣𝔱_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(imm.unwrap())],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::Ξ𝔯𝔳32[𝔠_𝔰𝔯𝔞𝔦_𝔩𝔬𝔴]Ξ𝔯𝔳64[𝔠_𝔰𝔯𝔞𝔦],
                                 imm.unwrap().into(),
                                 0)
                            }
                            (1,  0,  0, i5,  1,  0, r2, r1, r0, i4, i3, i2, i1, i0,  0,  1) => {
                                let imm = (i5 as i32 * -1)<<5 | (i4<<4 | i3<<3 | i2<<2 | i1<<1 | i0) as i32;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔞𝔫𝔡𝔦,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(imm.unwrap())],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔫𝔡𝔦,
                                 imm.unwrap().into(),
                                 0)
                            }
                            (1,  0,  0,  0,  1,  1, r2, r1, r0,  0,  0, s2, s1, s0,  0,  1) => {
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                let rs2 = 8 | s2<<2 | s1<<1 | s0;
                                let rs2: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs2.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔰𝔲𝔟,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rs2.unwrap())],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔫𝔡_𝔬𝔯_𝔰𝔲𝔟_𝔵𝔬𝔯,
                                 0,
                                 0)
                            }
                            (1,  0,  0,  0,  1,  1, r2, r1, r0,  0,  1, s2, s1, s0,  0,  1) => {
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                let rs2 = 8 | s2<<2 | s1<<1 | s0;
                                let rs2: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs2.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔵𝔬𝔯,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rs2.unwrap())],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔫𝔡_𝔬𝔯_𝔰𝔲𝔟_𝔵𝔬𝔯,
                                 0,
                                 0)
                            }
                            (1,  0,  0,  0,  1,  1, r2, r1, r0,  1,  0, s2, s1, s0,  0,  1) => {
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                let rs2 = 8 | s2<<2 | s1<<1 | s0;
                                let rs2: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs2.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔬𝔯,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rs2.unwrap())],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔫𝔡_𝔬𝔯_𝔰𝔲𝔟_𝔵𝔬𝔯,
                                 0,
                                 0)
                            }
                            (1,  0,  0,  0,  1,  1, r2, r1, r0,  1,  1, s2, s1, s0,  0,  1) => {
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                let rs2 = 8 | s2<<2 | s1<<1 | s0;
                                let rs2: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs2.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔞𝔫𝔡,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rs2.unwrap())],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔫𝔡_𝔬𝔯_𝔰𝔲𝔟_𝔵𝔬𝔯,
                                 0,
                                 0)
                            }
                    Ξ𝔯𝔳64[  (1,  0,  0,  1,  1,  1, r2, r1, r0,  0,  0, s2, s1, s0,  0,  1) => {
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                let rs2 = 8 | s2<<2 | s1<<1 | s0;
                                let rs2: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs2.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔰𝔲𝔟𝔴,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rs2.unwrap())],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔡𝔡𝔴_𝔰𝔲𝔟𝔴,
                                 0,
                                 0)
                            }
                            (1,  0,  0,  1,  1,  1, r2, r1, r0,  0,  1, s2, s1, s0,  0,  1) => {
                                let rd = 8 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                let rs2 = 8 | s2<<2 | s1<<1 | s0;
                                let rs2: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs2.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔞𝔡𝔡𝔴,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rs2.unwrap())],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔡𝔡𝔴_𝔰𝔲𝔟𝔴,
                                 0,
                                 0)
                            }]
                            (1,  0,  0,  1,  1,  1,_r2,_r1,_r0,_x1,_x0,_s2,_s1,_s0,  0,  1) => {
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔲𝔫𝔦𝔪𝔭,
                                 Vec::new(),
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔡𝔡𝔴_𝔰𝔲𝔟𝔴,
                                 0,
                                 0)
                            }
                            (1,  0,  1,i11, i4, i9, i8,i10, i6, i7, i3, i2, i1, i5,  0,  1) => {
                                let imm = (i11 as i32 * -1)<<11 | (i10<<10 | i9<<9 | i8<<8 | i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3 | i2<<2 | i1<<1) as i32;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐣𝐮𝐦𝐩_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔧𝔞𝔩,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵0.into()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔧𝔲𝔪𝔭_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(imm.unwrap())],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔧,
                                 imm.unwrap().into(),
                                 0)
                            }
                            (1,  1,  0, i8, i4, i3, s2, s1, s0, i7, i6, i2, i1, i5,  0,  1) => {
                                let imm = (i8 as i32 * -1)<<8 | (i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3 | i2<<2 | i1<<1) as i32;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐛𝐫𝐚𝐧𝐜𝐡_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rs1 = 8 | s2<<2 | s1<<1 | s0;
                                let rs1: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs1.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔟𝔢𝔮,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rs1.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵0.into()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔟𝔯𝔞𝔫𝔠𝔥_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(imm.unwrap())],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔟𝔢𝔮𝔷,
                                 imm.unwrap().into(),
                                 0)
                            }
                            (1,  1,  1, i8, i4, i3, s2, s1, s0, i7, i6, i2, i1, i5,  0,  1) => {
                                let imm = (i8 as i32 * -1)<<8 | (i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3 | i2<<2 | i1<<1) as i32;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐛𝐫𝐚𝐧𝐜𝐡_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rs1 = 8 | s2<<2 | s1<<1 | s0;
                                let rs1: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs1.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔟𝔫𝔢,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rs1.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵0.into()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔟𝔯𝔞𝔫𝔠𝔥_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(imm.unwrap())],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔟𝔫𝔢𝔷,
                                 imm.unwrap().into(),
                                 0)
                            }
                    Ξ𝔯𝔳32[  (0,  0,  0,  1, r4, r3, r2, r1, r0, i4, i3, i2, i1, i0,  1,  0) =>
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔲𝔫𝔦𝔪𝔭,
                                 Vec::new(),
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔰𝔩𝔩𝔦_𝔥𝔦𝔤𝔥,
                                 (1<<5 | i4<<4 | i3<<3 | i2<<2 | i1<<1 | i0) as i32,
                                 (r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0) as i32),]
                            (0,  0,  0, i5, r4, r3, r2, r1, r0, i4, i3, i2, i1, i0,  1,  0) => {
                                let imm = i5<<5 | i4<<4 | i3<<3 | i2<<2 | i1<<1 | i0;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐬𝐡𝐢𝐟𝐭_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔰𝔩𝔩𝔦,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔰𝔥𝔦𝔣𝔱_𝔦𝔪𝔪𝔢𝔡𝔦𝔞𝔱𝔢_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(imm.unwrap())],
                                 if i5 == 1 { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔰𝔩𝔩𝔦_𝔥𝔦𝔤𝔥 } else { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔰𝔩𝔩𝔦_𝔩𝔬𝔴 },
                                 imm.unwrap().into(),
                                 (r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0).into())
                            }
                            (0,  0,  1, i5, r4, r3, r2, r1, r0, i4, i3, i8, i7, i6,  1,  0) => {
                                let imm = i8<<8 | i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔣𝔩𝔡,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵2.into(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 if i5 == 1 { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔣𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥 } else { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔣𝔩𝔡𝔰𝔭_𝔩𝔬𝔴 },
                                 imm.unwrap().into(),
                                 (r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0).into())
                            }
                            (0,  1,  0, i5,  0,  0,  0,  0,  0, i4, i3, i2, i7, i6,  1,  0) =>
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔲𝔫𝔦𝔪𝔭,
                                 Vec::new(),
                                 if i5 == 1 { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥 } else { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴 },
                                 (i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3 | i2<<2) as i32,
                                 0),
                            (0,  1,  0, i5, r4, r3, r2, r1, r0, i4, i3, i2, i7, i6,  1,  0) => {
                                let imm = i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3 | i2<<2;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔩𝔴,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵2.into(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 if i5 == 1 { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥 } else { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴 },
                                 imm.unwrap().into(),
                                 (r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0).into())
                            }
                    Ξ𝔯𝔳32[  (0,  1,  1, i5, r4, r3, r2, r1, r0, i4, i3, i2, i7, i6,  1,  0) => {
                                let imm = i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3 | i2<<2;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔣𝔩𝔴,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵2.into(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 if i5 == 1 { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔣𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥 } else { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔣𝔩𝔴𝔰𝔭_𝔩𝔬𝔴 },
                                 imm.unwrap().into(),
                                 (r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0).into())
                            }]
                    Ξ𝔯𝔳64[  (0,  1,  1, i5,  0,  0,  0,  0,  0, i4, i3, i8, i7, i6,  1,  0) =>
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔲𝔫𝔦𝔪𝔭,
                                 Vec::new(),
                                 if i5 == 1 { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥 } else { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔡𝔰𝔭_𝔩𝔬𝔴 },
                                 (i8<<8 | i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3) as i32,
                                 0),
                            (0,  1,  1, i5, r4, r3, r2, r1, r0, i4, i3, i8, i7, i6,  1,  0) => {
                                let imm = i8<<8 | i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rd = r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔩𝔡,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵2.into(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 if i5 == 1 { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥 } else { $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔩𝔡𝔰𝔭_𝔩𝔬𝔴 },
                                 imm.unwrap().into(),
                                 (r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0).into())
                            }]
                            (1,  0,  0,  0,  0,  0,  0,  0,  0,_s4,_s3,_s2,_s1,_s0,  1,  0) =>
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔲𝔫𝔦𝔪𝔭,
                                 Vec::new(),
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔪𝔳,
                                 0,
                                 0),
                            (1,  0,  0,  0, s4, s3, s2, s1, s0,  0,  0,  0,  0,  0,  1,  0) => {
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = 0.try_into();
                                let rs1 = s4<<4 | s3<<3 | s2<<2 | s1<<1 | s0;
                                let rs1: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs1.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔧𝔞𝔩𝔯,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵0.into()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:rs1.unwrap(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔪𝔳,
                                 0,
                                 (s4<<4 | s3<<3 | s2<<2 | s1<<1 | s0).into())
                            }
                            (1,  0,  0,  0, r4, r3, r2, r1, r0, s4, s3, s2, s1, s0,  1,  0) => {
                                let rd = r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                let rs2 = s4<<4 | s3<<3 | s2<<2 | s1<<1 | s0;
                                let rs2: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs2.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔞𝔡𝔡,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵0.into()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rs2.unwrap())],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔪𝔳,
                                 0,
                                 (r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0).into())
                            }
                            (1,  0,  0,  1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  1,  0) =>
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔢𝔟𝔯𝔢𝔞𝔨,
                                 Vec::new(),
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔡𝔡,
                                 0,
                                 0),
                            (1,  0,  0,  1, s4, s3, s2, s1, s0,  0,  0,  0,  0,  0,  1,  0) => {
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = 0.try_into();
                                let rs1 = s4<<4 | s3<<3 | s2<<2 | s1<<1 | s0;
                                let rs1: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs1.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔧𝔞𝔩𝔯,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵1.into()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔰𝔬𝔲𝔯𝔠𝔢_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:rs1.unwrap(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔡𝔡,
                                 0,
                                 (s4<<4 | s3<<3 | s2<<2 | s1<<1 | s0).into())
                            }
                            (1,  0,  0,  1, r4, r3, r2, r1, r0, s4, s3, s2, s1, s0,  1,  0) => {
                                let rd = r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0;
                                let rd: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rd.try_into();
                                let rs2 = s4<<4 | s3<<3 | s2<<2 | s1<<1 | s0;
                                let rs2: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs2.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔞𝔡𝔡,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rd.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rs2.unwrap())],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔞𝔡𝔡,
                                 0,
                                 (r4<<4 | r3<<3 | r2<<2 | r1<<1 | r0).into())
                            }
                            (1,  0,  1, i5, i4, i3, i8, i7, i6, s4, s3, s2, s1, s0,  1,  0) => {
                                let imm = i8<<8 | i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rs2 = s4<<4 | s3<<3 | s2<<2 | s1<<1 | s0;
                                let rs2: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs2.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔣𝔰𝔡,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rs2.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵2.into(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔣𝔰𝔡𝔰𝔭,
                                 imm.unwrap().into(),
                                 0)
                            }
                            (1,  1,  0, i5, i4, i3, i2, i7, i6, s4, s3, s2, s1, s0,  1,  0) => {
                                let imm = i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3 | i2<<2;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rs2 = s4<<4 | s3<<3 | s2<<2 | s1<<1 | s0;
                                let rs2: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs2.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔰𝔴,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rs2.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵2.into(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔰𝔴𝔰𝔭,
                                 imm.unwrap().into(),
                                 0)
                            }
                    Ξ𝔯𝔳32[  (1,  1,  1, i5, i4, i3, i2, i7, i6, s4, s3, s2, s1, s0,  1,  0) => {
                                let imm = i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3 | i2<<2;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rs2 = s4<<4 | s3<<3 | s2<<2 | s1<<1 | s0;
                                let rs2: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs2.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔣𝔰𝔴,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔣𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rs2.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵2.into(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔣𝔰𝔴𝔰𝔭,
                                 imm.unwrap().into(),
                                 0)
                            }]
                    Ξ𝔯𝔳64[  (1,  1,  1, i5, i4, i3, i8, i7, i6, s4, s3, s2, s1, s0,  1,  0) => {
                                let imm = i8<<8 | i7<<7 | i6<<6 | i5<<5 | i4<<4 | i3<<3;
                                let imm: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, _> = imm.try_into();
                                let rs2 = s4<<4 | s3<<3 | s2<<2 | s1<<1 | s0;
                                let rs2: Result<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜, _> = rs2.try_into();
                                ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔰𝔡,
                                 vec![$𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(rs2.unwrap()),
                                      $𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓽𝔂𝓹𝓮::<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>::𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔬𝔭𝔢𝔯𝔞𝔫𝔡(𝒂𝒅𝒅𝒓𝒆𝒔𝒔{𝖻𝖺𝗌𝖾:<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as $𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>::𝔵2.into(), 𝖽𝗂𝗌𝗉:imm.unwrap()})],
                                 $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔰𝔡𝔰𝔭,
                                 imm.unwrap().into(),
                                 0)
                            }]
                            _ => ($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓴𝓲𝓷𝓭_𝓽𝔂𝓹𝓮::𝔲𝔫𝔦𝔪𝔭,
                                  Vec::new(),
                                  $𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓮𝓭_𝓼𝓽𝓮𝓹::𝔠_𝔲𝔫𝔯𝔢𝔠𝔬𝔤𝔫𝔦𝔷𝔢𝔡,
                                  0,
                                  0)
                        }
                    }
                }
            }
         )*
    };
    ($( $𝓼𝓵𝓸𝔀_𝓭𝓮𝓬𝓸𝓭𝓮_𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓼𝓮𝓭_𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        fn $𝓼𝓵𝓸𝔀_𝓭𝓮𝓬𝓸𝓭𝓮_𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓼𝓮𝓭_𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷:ident $𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼:tt
            -> $𝓸𝓾𝓽𝓹𝓾𝓽_𝓽𝔂𝓹𝓮:tt
        $𝓼𝓵𝓸𝔀_𝓭𝓮𝓬𝓸𝓭𝓮_𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓼𝓮𝓭_𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓫𝓸𝓭𝔂:tt
      )*) => {
        $(
            fn $𝓼𝓵𝓸𝔀_𝓭𝓮𝓬𝓸𝓭𝓮_𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓼𝓮𝓭_𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷 $𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼
                -> $𝓸𝓾𝓽𝓹𝓾𝓽_𝓽𝔂𝓹𝓮
            $𝓼𝓵𝓸𝔀_𝓭𝓮𝓬𝓸𝓭𝓮_𝓬𝓸𝓶𝓹𝓻𝓮𝓼𝓼𝓮𝓭_𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓫𝓸𝓭𝔂
         )*
    };
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖑𝖔𝖜_𝖉𝖊𝖈𝖔𝖉𝖊_𝖈𝖔𝖒𝖕𝖗𝖊𝖘𝖘𝖊𝖉_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓! {
    [𝔯𝔳32𝔦]
    fn slow_decode_compressed_instruction_rv32i
        -> (𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟯𝟮𝗶, 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟯𝟮𝗶<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢 as 𝑪𝑷𝑼_𝗿𝘃𝟯𝟮𝗶>, 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐)
    {
    }

    [𝔯𝔳64𝔦]
    fn slow_decode_compressed_instruction_rv64i
        -> (𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶, 𝐨𝐩𝐞𝐫𝐚𝐧𝐝_𝗿𝘃𝟲𝟰𝗶<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢 as 𝑪𝑷𝑼_𝗿𝘃𝟲𝟰𝗶>, 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒)
    {
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i8)]
enum 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐 {
    𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_0 = 0,
    𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_1 = 1,
    𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_2 = 2,
    𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_3 = 3,
    𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_4 = 4,
    𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_5 = 5,
    𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_6 = 6,
    𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_7 = 7,
    𝔠_𝔞𝔡𝔡𝔦4𝔰𝔭𝔫 = 8,
    𝔠_𝔣𝔩𝔡 = 9,
    𝔠_𝔩𝔴 = 10,
    𝔠_𝔣𝔩𝔴 = 11,
    𝔠_𝔯𝔢𝔰𝔢𝔯𝔳𝔢𝔡 = 12,
    𝔠_𝔣𝔰𝔡 = 13,
    𝔠_𝔰𝔴 = 14,
    𝔠_𝔣𝔰𝔴 = 15,
    // Here we need to decode both immediate (one bit) and parts of 𝔯𝔡1/𝔯𝔰1.
    // We handle negative bit separately.
    𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00 = 16,
    𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00 = 17, // Used for 𝔠_𝔫𝔬𝔭, too.
    𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 = 18,
    𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 = 19,
    𝔠_𝔧𝔞𝔩 = 20,
    𝔠_𝔩𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 = 22,
    𝔠_𝔩𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 = 23,
    𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00 = 24,
    𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00 = 25,
    𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 = 26,
    𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 = 27,
    // Treating these separately simplifies the logic, although, strictly speaking, is not needed.
    𝔠_𝔰𝔯𝔩𝔦_𝔩𝔬𝔴 = 28,
    𝔠_𝔰𝔯𝔩𝔦_𝔥𝔦𝔤𝔥 = 29,
    𝔠_𝔰𝔯𝔞𝔦_𝔩𝔬𝔴 = 30,
    𝔠_𝔰𝔯𝔞𝔦_𝔥𝔦𝔤𝔥 = 31,
    𝔠_𝔞𝔫𝔡𝔦 = 32,
    // These instruction needs additional decoding.
    𝔠_𝔞𝔫𝔡_𝔬𝔯_𝔰𝔲𝔟_𝔵𝔬𝔯 = 34,
    𝔠_𝔞𝔡𝔡𝔴_𝔰𝔲𝔟𝔴 = 35,
    𝔠_𝔧 = 21,
    𝔠_𝔟𝔢𝔮𝔷 = 36,
    𝔠_𝔟𝔫𝔢𝔷 = 37,
    𝔠_𝔰𝔩𝔩𝔦_𝔩𝔬𝔴 = 38,
    𝔠_𝔰𝔩𝔩𝔦_𝔥𝔦𝔤𝔥 = 39,
    𝔠_𝔣𝔩𝔡𝔰𝔭_𝔩𝔬𝔴 = 40,
    𝔠_𝔣𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥 = 41,
    𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴_00 = 42,
    𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥_00 = 43,
    𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴 = 44,
    𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥 = 45,
    𝔠_𝔣𝔩𝔴𝔰𝔭_𝔩𝔬𝔴 = 46,
    𝔠_𝔣𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥 = 47,
    𝔠_𝔪𝔳_00 = 48, // Used for 𝔠_𝔧𝔯, too.
    𝔠_𝔪𝔳 = 49,
    𝔠_𝔞𝔡𝔡_00 = 50, // Used for 𝔠_𝔧𝔞𝔩𝔯 and 𝔠_𝔢𝔟𝔯𝔢𝔞𝔨, too.
    𝔠_𝔞𝔡𝔡 = 33,
    𝔠_𝔣𝔰𝔡𝔰𝔭 = 51,
    𝔠_𝔰𝔴𝔰𝔭 = 52,
    𝔠_𝔣𝔰𝔴𝔰𝔭 = 53,
    𝔠_𝔲𝔫𝔯𝔢𝔠𝔬𝔤𝔫𝔦𝔷𝔢𝔡 = 54,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i8)]
enum 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒 {
    𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_0 = 0,
    𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_1 = 1,
    𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_2 = 2,
    𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_3 = 3,
    𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_4 = 4,
    𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_5 = 5,
    𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_6 = 6,
    𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_7 = 7,
    𝔠_𝔞𝔡𝔡𝔦4𝔰𝔭𝔫 = 8,
    𝔠_𝔣𝔩𝔡 = 9,
    𝔠_𝔩𝔴 = 10,
    𝔠_𝔩𝔡 = 11,
    𝔠_𝔯𝔢𝔰𝔢𝔯𝔳𝔢𝔡 = 12,
    𝔠_𝔣𝔰𝔡 = 13,
    𝔠_𝔰𝔴 = 14,
    𝔠_𝔰𝔡 = 15,
    // Here we need to decode both immediate (one bit) and parts of 𝔯𝔡1/𝔯𝔰1.
    // We handle negative bit separately.
    𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00 = 16,
    𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00 = 17, // Used for 𝔠_𝔫𝔬𝔭, too.
    𝔠_𝔞𝔡𝔡𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 = 18,
    𝔠_𝔞𝔡𝔡𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 = 19,
    𝔠_𝔞𝔡𝔡𝔦𝔴_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00 = 20,
    𝔠_𝔞𝔡𝔡𝔦𝔴_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00 = 21,
    𝔠_𝔞𝔡𝔡𝔦𝔴_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 = 22, // Used for 𝔠_𝔧𝔞𝔩, too.
    𝔠_𝔞𝔡𝔡𝔦𝔴_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 = 23,
    𝔠_𝔩𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 = 24,
    𝔠_𝔩𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 = 25,
    𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢_00 = 26,
    𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢_00 = 27,
    𝔠_𝔩𝔲𝔦_𝔭𝔬𝔰𝔦𝔱𝔦𝔳𝔢 = 28,
    𝔠_𝔩𝔲𝔦_𝔫𝔞𝔤𝔞𝔱𝔦𝔳𝔢 = 29,
    𝔠_𝔰𝔯𝔩𝔦 = 30,
    𝔠_𝔰𝔯𝔞𝔦 = 31,
    𝔠_𝔞𝔫𝔡𝔦 = 32,
    // These instruction needs additional decoding.
    𝔠_𝔞𝔫𝔡_𝔬𝔯_𝔰𝔲𝔟_𝔵𝔬𝔯 = 33,
    𝔠_𝔞𝔡𝔡𝔴_𝔰𝔲𝔟𝔴 = 34,
    𝔠_𝔧 = 35,
    𝔠_𝔟𝔢𝔮𝔷 = 36,
    𝔠_𝔟𝔫𝔢𝔷 = 37,
    𝔠_𝔰𝔩𝔩𝔦_𝔩𝔬𝔴 = 38,
    𝔠_𝔰𝔩𝔩𝔦_𝔥𝔦𝔤𝔥 = 39,
    𝔠_𝔣𝔩𝔡𝔰𝔭_𝔩𝔬𝔴 = 40,
    𝔠_𝔣𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥 = 41,
    𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴_00 = 42,
    𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥_00 = 43,
    𝔠_𝔩𝔴𝔰𝔭_𝔩𝔬𝔴 = 44,
    𝔠_𝔩𝔴𝔰𝔭_𝔥𝔦𝔤𝔥 = 45,
    𝔠_𝔩𝔡𝔰𝔭_𝔩𝔬𝔴_00 = 46,
    𝔠_𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥_00 = 47,
    𝔠_𝔩𝔡𝔰𝔭_𝔩𝔬𝔴 = 48,
    𝔠_𝔩𝔡𝔰𝔭_𝔥𝔦𝔤𝔥 = 49,
    𝔠_𝔪𝔳_00 = 50, // Used for 𝔠_𝔧𝔯, too.
    𝔠_𝔪𝔳 = 51,
    𝔠_𝔞𝔡𝔡_00 = 52, // Used for 𝔠_𝔧𝔞𝔩𝔯 and 𝔠_𝔢𝔟𝔯𝔢𝔞𝔨, too.
    𝔠_𝔞𝔡𝔡 = 53,
    𝔠_𝔣𝔰𝔡𝔰𝔭 = 54,
    𝔠_𝔰𝔴𝔰𝔭 = 55,
    𝔠_𝔰𝔡𝔰𝔭 = 56,
    𝔠_𝔲𝔫𝔯𝔢𝔠𝔬𝔤𝔫𝔦𝔷𝔢𝔡 = 57,
}
