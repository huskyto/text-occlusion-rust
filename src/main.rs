use std::fs;

use clap::{Arg, ArgAction, Command};


const HAIR_SPACE: char = '\u{200A}';
const ZERO_WIDTH_SPACE: char = '\u{200B}';
const ZERO_WIDTH_NON_JOINER: char = '\u{200C}';
const ZERO_WIDTH_JOINER: char = '\u{200D}';
const WORD_JOINER: char = '\u{2060}';
// const ZERO_WIDTH_NO_BREAK_SPACE: &str = "\u{FEFF}";

const BASE_4_CODES: [char; 4] = [ZERO_WIDTH_SPACE, ZERO_WIDTH_NON_JOINER,
                                 ZERO_WIDTH_JOINER, WORD_JOINER];
// const BASE_4_CODES: [char; 4] = ['a', 'b', 'c', 'd'];

const ZONE_CODE: [char; 3] = [ZERO_WIDTH_SPACE, HAIR_SPACE, ZERO_WIDTH_SPACE];


fn main() {
    let matches = Command::new("Text Occlusion")
        .about("Hides and recovers text or binary information in a text file.")

        .arg(Arg::new("tail-hide")
            .short('t')
            .long("tailhide")
            .action(ArgAction::SetTrue)
            .help("[Action] Sets operation to hide encode and hide text into the input"))
        .arg(Arg::new("recover")
            .short('r')
            .long("recover")
            .action(ArgAction::SetTrue)
            .help("[Action] Retrieves the hidden information."))

        .arg(Arg::new("input-file")
            .short('i')
            .long("input")
            .help("Input file that will start seemingly the same. Required."))
        .arg(Arg::new("output-file")
            .short('o')
            .long("output")
            .help("Modified output file. If not set, will output to stdio."))
        .arg(Arg::new("hide-file")
            .short('c')
            .long("hidefile")
            .help("File that will be hidden in the input. Required if hiding."))

        .get_matches();

    let flag_tail_hide = matches.get_flag("tail-hide");
    let flag_recover = matches.get_flag("recover");

    let input_file_opt: Option<String> = matches.get_one::<String>("input-file").cloned();
    let output_file_opt: Option<String> = matches.get_one::<String>("output-file").cloned();
    let hide_file_opt: Option<String> = matches.get_one::<String>("hide-file").cloned();

    if !flag_tail_hide && ! flag_recover {
        panic!("No action flag selected. Run with -h to see options.");
    }

    if flag_tail_hide && flag_recover {
        panic!("Only one action flag can be enabled.");
    }

    if input_file_opt.is_none() {
        panic!("No input file!");
    }

    if flag_tail_hide {
        if hide_file_opt.is_none() {
            panic!("No file to hide was selected.");
        }
        let input_text = fs::read_to_string(input_file_opt.unwrap())
                .expect("Invalid input file.");
        let to_hide = fs::read(hide_file_opt.unwrap()).unwrap();
        let tail_hidden = hide_on_tail(&input_text, &to_hide);
        if let Some(of) = output_file_opt {
            let _ = fs::write(of, tail_hidden);
        }
        else {
            println!("{tail_hidden}");
        }
    }

    else if flag_recover {
        let input_text = fs::read_to_string(input_file_opt.unwrap())
                .expect("Invalid input file.");
        let recovered = recover_hidden(&input_text)[0].clone();
        if let Some(of) = output_file_opt {
            let _ = fs::write(of, recovered);
        }
        else {
            println!("{}", String::from_utf8_lossy(&recovered));
        }
    }
}

fn to_base_4(input: &[u8]) -> String {
    let mut res = String::new();
    for n in input {
        let p1 = (n & 0b11000000) >> 6;
        res.push(BASE_4_CODES[p1 as usize]);
        let p2 = (n & 0b00110000) >> 4;
        res.push(BASE_4_CODES[p2 as usize]);
        let p3 = (n & 0b00001100) >> 2;
        res.push(BASE_4_CODES[p3 as usize]);
        let p4 = n & 0b00000011;
        res.push(BASE_4_CODES[p4 as usize]);
        // println!("{p1}, {p2}, {p3}, {p4}");
    }

    res
}

fn from_base_4(input: &str) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    let mut next_u8 = 0;
    let mut step = 0;
    for n in input.chars() {
        let v = get_char_val(n);
        if step == 0 {
            next_u8 += v << 6;
        }
        else if step == 1 {
            next_u8 += v << 4;
        }
        else if step == 2 {
            next_u8 += v << 2;
        }
        else {
            next_u8 += v;
            res.push(next_u8);
            next_u8 = 0;
            step = -1;
        }

        step += 1;
    }

    res
}

fn get_char_val(c: char) -> u8 {
    BASE_4_CODES.iter()
        .position(|bc| *bc == c)
        .expect(&format!("Invalid char: {c}")) as u8
}

fn hide_on_tail(source: &str, to_hide: &[u8]) -> String {
    let zone_flag: String = get_zone_flag();
    let coded = to_base_4(to_hide);
    format!("{source}{zone_flag}{coded}{zone_flag}")
}

fn recover_hidden(source: &str) -> Vec<Vec<u8>> {
    let mut res: Vec<Vec<u8>> = vec![];
    let zone_flag: String = get_zone_flag();
    let flags: Vec<(usize, &str)> = source.match_indices(&zone_flag).collect();
    for i in (0..flags.len()).step_by(2) {
        let start = flags[i].0 + zone_flag.len();
        let end = flags.get(i + 1).expect("Missing closing flag!").0;
        let hidden_str = &source[start..end];
        let recovered = from_base_4(hidden_str);
        res.push(recovered);
    }

    res
}

fn get_zone_flag() -> String {
    ZONE_CODE.iter().collect()
}

#[test]
fn test_encoding() {
    let encoded = to_base_4("Hidden text".as_bytes());
    assert_eq!(encoded, "‌​‍​‌‍‍‌‌‍‌​‌‍‌​‌‍‌‌‌‍⁠‍​‍​​‌⁠‌​‌‍‌‌‌⁠‍​‌⁠‌​");
}

#[test]
fn test_decoding() {
    let decoded = String::from_utf8(from_base_4("‌​‍​‌‍‍‌‌‍‌​‌‍‌​‌‍‌‌‌‍⁠‍​‍​​‌⁠‌​‌‍‌‌‌⁠‍​‌⁠‌​")).unwrap();
    assert_eq!(decoded, "Hidden text");
}
