use lazy_static::lazy_static;

pub struct Character {
    pub glyph: char,
    pub bitmap: [u8; 5],
}

impl Character {
    fn new(glyph: char, bitmap: [u8; 5]) -> Self {
        Character { glyph, bitmap }
    }
}

lazy_static! {
    pub static ref FONT_CHARACTERS: Vec<Character> = vec![
        Character::new('0', [0xf0, 0x90, 0x90, 0x90, 0xf0]),
        Character::new('1', [0x20, 0x60, 0x20, 0x20, 0x70]),
        Character::new('2', [0xf0, 0x10, 0xf0, 0x80, 0xf0]),
        Character::new('3', [0xf0, 0x10, 0xf0, 0x10, 0xf0]),
        Character::new('4', [0x90, 0x90, 0xf0, 0x10, 0x10]),
        Character::new('5', [0xf0, 0x80, 0xf0, 0x10, 0xf0]),
        Character::new('6', [0xf0, 0x80, 0xf0, 0x90, 0xf0]),
        Character::new('7', [0xf0, 0x10, 0x20, 0x40, 0x40]),
        Character::new('8', [0xf0, 0x90, 0xf0, 0x90, 0xf0]),
        Character::new('9', [0xf0, 0x90, 0xf0, 0x10, 0xf0]),
        Character::new('A', [0xf0, 0x90, 0xf0, 0x90, 0x90]),
        Character::new('B', [0xe0, 0x90, 0xe0, 0x90, 0xe0]),
        Character::new('C', [0xf0, 0x80, 0x80, 0x80, 0xf0]),
        Character::new('D', [0xe0, 0x90, 0x90, 0x90, 0xe0]),
        Character::new('E', [0xf0, 0x80, 0xf0, 0x80, 0xf0]),
        Character::new('F', [0xf0, 0x80, 0xf0, 0x80, 0x80]),
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_character(char: &Character) {
        println!("Printing character {}", char.glyph);
        let char_string: String = char
            .bitmap
            .iter()
            .map(|line_byte| {
                let mut line = String::with_capacity(4);

                for shift in (4..=7).rev() {
                    line.push(if line_byte & (1 << shift) != 0 {
                        'X'
                    } else {
                        ' '
                    });
                }

                line
            })
            .collect::<Vec<String>>()
            .join("\n");

        println!("{}\n", char_string)
    }

    #[test]
    fn test_characters() {
        for character in FONT_CHARACTERS.iter() {
            print_character(character);
        }
    }
}
