
const HAIR_SPACE: &str = "\u{200A}";
const ZERO_WIDTH_SPACE: char = '\u{200B}';
const ZERO_WIDTH_NON_JOINER: char = '\u{200C}';
const ZERO_WIDTH_JOINER: char = '\u{200D}';
const WORD_JOINER: char = '\u{2060}';
// const ZERO_WIDTH_NO_BREAK_SPACE: &str = "\u{FEFF}";

const BASE_4_CODES: [char; 4] = [ZERO_WIDTH_SPACE, ZERO_WIDTH_NON_JOINER,
                                 ZERO_WIDTH_JOINER, WORD_JOINER];
// const BASE_4_CODES: [char; 4] = ['a', 'b', 'c', 'd'];


fn main() {
    // println!("Hello, world!");

    let mut example_text = "This is some text!".to_string();
    example_text.push_str(HAIR_SPACE);
    example_text.push_str(HAIR_SPACE);
    example_text.push_str(HAIR_SPACE);
    example_text.push_str(HAIR_SPACE);
    example_text.push_str("borka");

    let to_hide = "This is some hidden text o:".to_string();
    let b4 = to_base_4(to_hide.as_bytes());

    println!("{example_text}");
    println!("B4: {b4}.");

    // println!("Hidden u8: {:#?}", to_hide.as_bytes());
    let recovered = from_base_4(&b4);
    // println!("Recovered u8: {:#?}", recovered);
    let rec_str = String::from_utf8(recovered).unwrap();
    println!("Recovered: {rec_str}");
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
        // TODO there must be a better way?
    if c == BASE_4_CODES[0] {
        0
    }
    else if c == BASE_4_CODES[1] {
        1
    }
    else if c == BASE_4_CODES[2] {
        2
    }
    else if c == BASE_4_CODES[3] {
        3
    }
    else {
        panic!("Invalid char: {c}")
    }
    
}
