#![allow(uncommon_codepoints)]
#![allow(non_camel_case_types)]

use std::collections::BTreeMap;

use yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝘃𝟯𝟮𝗶::𝑪𝑷𝑼 as 𝑪𝑷𝑼_𝗿𝘃𝟯𝟮𝗶;
use yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝘃𝟲𝟰𝗶::𝑪𝑷𝑼 as 𝑪𝑷𝑼_𝗿𝘃𝟲𝟰𝗶;

type 𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢 =
    yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝘃𝟯𝟮𝗶::𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫;
type 𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢 =
    yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝘃𝟲𝟰𝗶::𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫;

fn main() {
    generate_display_for_type::<<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟑𝟐𝐢 as 𝑪𝑷𝑼_𝗿𝘃𝟯𝟮𝗶>::𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>("𝔠𝔰𝔯_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝐫𝐯𝟑𝟐_𝔯𝔢𝔣𝔢𝔯𝔢𝔫𝔠𝔢", "𝔠𝔰𝔯_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝐫𝐯𝟑𝟐_𝔫𝔞𝔪𝔢𝔰");
    generate_display_for_type::<<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢 as 𝑪𝑷𝑼_𝗿𝘃𝟲𝟰𝗶>::𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>("𝔠𝔰𝔯_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝐫𝐯𝟔𝟒_𝔯𝔢𝔣𝔢𝔯𝔢𝔫𝔠𝔢", "𝔠𝔰𝔯_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝐫𝐯𝟔𝟒_𝔫𝔞𝔪𝔢𝔰");
}

fn generate_display_for_type<𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮: Clone + core::fmt::Display + core::fmt::Debug + TryFrom<i16>>(
    csr_register_reference_var_name: &str,
    csr_register_names_var_name: &str,
) {
    let mut strings = BTreeMap::new();
    for value in i16::MIN..=i16::MAX {
        if let Ok(string) = TryInto::<𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>::try_into(value) {
            #[allow(non_snake_case)]
            let mut ASCII_string = Vec::new();
            for c in format!("{:?}", string).chars() {
                ASCII_string.push(match c {
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
            if ASCII_string.len() > 15 {
                panic!("Too long name: {}", std::str::from_utf8(&ASCII_string).unwrap());
            }
            strings.insert((!(ASCII_string.len()), ASCII_string), value);
        }
    }
    let mut csr_register_reference = [0u16; 0x1000];
    let mut csr_register_names = Vec::new();
    for ((_, string), value) in strings.into_iter() {
        if let Ok(position) = subarray_position(&csr_register_names, &string) {
            csr_register_reference[value as usize] = (position & 0xfff) as u16 | (string.len() << 12) as u16;
        } else {
            csr_register_reference[value as usize] = (csr_register_names.len() & 0xfff) as u16 | (string.len() << 12) as u16;
            csr_register_names.extend_from_slice(&string);
        }
    }
    if csr_register_names.len() > 0xfff {
        panic!("Too large generated csr_register_names text");
    }
    println!(
        "#[allow(non_upper_case_globals)]\n#[cfg(feature = \"std\")]\nconst {}: [u16; 0x1000] = [{}];",
        csr_register_reference_var_name,
        csr_register_reference
            .into_iter()
            .map(|value| if value == 0 {
                "0".to_owned()
            } else {
                format!("0x{value:04x}")
            })
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "#[allow(non_upper_case_globals)]\n#[cfg(feature = \"std\")]\nconst {}: [u8; {}] = *b\"{}\";",
        csr_register_names_var_name,
        csr_register_names.len(),
        std::str::from_utf8(&csr_register_names).unwrap()
    );
}

fn subarray_position(array: &[u8], subarray: &[u8]) -> Result<usize, ()> {
    for (position, window) in array.windows(subarray.len()).enumerate() {
        if subarray == window {
            return Ok(position);
        }
    }
    Err(())
}
