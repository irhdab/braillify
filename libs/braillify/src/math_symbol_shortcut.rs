use phf::phf_map;

use crate::{error::BraillifyError, unicode::decode_unicode};

static SHORTCUT_MAP: phf::Map<char, &'static [u8]> = phf_map! {
    '+' => &[decode_unicode('⠢')],
    '−' => &[decode_unicode('⠔')],
    '×' => &[decode_unicode('⠡')],
    '÷' => &[decode_unicode('⠌'),decode_unicode('⠌')],
    '=' => &[decode_unicode('⠒'),decode_unicode('⠒')],
    '>' => &[decode_unicode('⠢'),decode_unicode('⠢')],
    '<' => &[decode_unicode('⠔'),decode_unicode('⠔')],
};

pub fn encode_char_math_symbol_shortcut(text: char) -> Result<&'static [u8], BraillifyError> {
    if let Some(code) = SHORTCUT_MAP.get(&text) {
        Ok(code)
    } else {
        Err(BraillifyError::InvalidMathSymbolCharacter { character: text, position: None })
    }
}

pub fn is_math_symbol_char(text: char) -> bool {
    SHORTCUT_MAP.contains_key(&text)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_is_math_symbol_char() {
        assert!(is_math_symbol_char('+'));
        assert!(is_math_symbol_char('−'));
        assert!(is_math_symbol_char('×'));
        assert!(is_math_symbol_char('÷'));
        assert!(is_math_symbol_char('='));
        assert!(is_math_symbol_char('>'));
        assert!(is_math_symbol_char('<'));
        assert!(!is_math_symbol_char('a'));
    }

    #[test]
    pub fn test_encode_char_math_symbol_shortcut() {
        assert_eq!(
            encode_char_math_symbol_shortcut('+').unwrap(),
            &[decode_unicode('⠢')]
        );
        assert_eq!(
            encode_char_math_symbol_shortcut('−').unwrap(),
            &[decode_unicode('⠔')]
        );
        assert_eq!(
            encode_char_math_symbol_shortcut('×').unwrap(),
            &[decode_unicode('⠡')]
        );
        assert_eq!(
            encode_char_math_symbol_shortcut('÷').unwrap(),
            &[decode_unicode('⠌'), decode_unicode('⠌')]
        );
        assert_eq!(
            encode_char_math_symbol_shortcut('=').unwrap(),
            &[decode_unicode('⠒'), decode_unicode('⠒')]
        );
        assert_eq!(
            encode_char_math_symbol_shortcut('>').unwrap(),
            &[decode_unicode('⠢'), decode_unicode('⠢')]
        );
        assert_eq!(
            encode_char_math_symbol_shortcut('<').unwrap(),
            &[decode_unicode('⠔'), decode_unicode('⠔')]
        );
        assert_eq!(
            encode_char_math_symbol_shortcut('a').unwrap_err(),
            "Invalid math symbol character"
        );
    }
}
