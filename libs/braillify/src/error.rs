use std::fmt;

#[derive(Debug, Clone)]
pub enum BraillifyError {
    InvalidCharacter { character: char, position: Option<usize>, context: String },
    InvalidKoreanCharacter { character: char, position: Option<usize> },
    InvalidKoreanChoseong { character: char, position: Option<usize> },
    InvalidKoreanJungseong { character: char, position: Option<usize> },
    InvalidKoreanJongseong { character: char, position: Option<usize> },
    InvalidKoreanPart { character: char, position: Option<usize> },
    InvalidEnglishCharacter { character: char, position: Option<usize> },
    InvalidNumberCharacter { character: char, position: Option<usize> },
    InvalidSymbolCharacter { character: char, position: Option<usize> },
    InvalidMathSymbolCharacter { character: char, position: Option<usize> },
    InvalidShortcutCharacter { character: char, position: Option<usize> },
    InvalidFractionPart { part_name: String, character: char, position: Option<usize> },
    InputTooLong { length: usize, max_length: usize },
    FractionParseError { input: String, error: String },
    Other { message: String, context: String },
}

impl fmt::Display for BraillifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BraillifyError::InvalidCharacter { character, position, context } => {
                match position {
                    Some(pos) => write!(f, "Invalid character '{}' at position {}, context: {}", character, pos, context),
                    None => write!(f, "Invalid character '{}', context: {}", character, context),
                }
            },
            BraillifyError::InvalidKoreanCharacter { character, position } => {
                match position {
                    Some(pos) => write!(f, "Invalid Korean character '{}' at position {}", character, pos),
                    None => write!(f, "Invalid Korean character '{}'", character),
                }
            },
            BraillifyError::InvalidKoreanChoseong { character, position } => {
                match position {
                    Some(pos) => write!(f, "Invalid Korean choseong character '{}' at position {}", character, pos),
                    None => write!(f, "Invalid Korean choseong character '{}'", character),
                }
            },
            BraillifyError::InvalidKoreanJungseong { character, position } => {
                match position {
                    Some(pos) => write!(f, "Invalid Korean jungseong character '{}' at position {}", character, pos),
                    None => write!(f, "Invalid Korean jungseong character '{}'", character),
                }
            },
            BraillifyError::InvalidKoreanJongseong { character, position } => {
                match position {
                    Some(pos) => write!(f, "Invalid Korean jongseong character '{}' at position {}", character, pos),
                    None => write!(f, "Invalid Korean jongseong character '{}'", character),
                }
            },
            BraillifyError::InvalidKoreanPart { character, position } => {
                match position {
                    Some(pos) => write!(f, "Invalid Korean part character '{}' at position {}", character, pos),
                    None => write!(f, "Invalid Korean part character '{}'", character),
                }
            },
            BraillifyError::InvalidEnglishCharacter { character, position } => {
                match position {
                    Some(pos) => write!(f, "Invalid English character '{}' at position {}", character, pos),
                    None => write!(f, "Invalid English character '{}'", character),
                }
            },
            BraillifyError::InvalidNumberCharacter { character, position } => {
                match position {
                    Some(pos) => write!(f, "Invalid number character '{}' at position {}", character, pos),
                    None => write!(f, "Invalid number character '{}'", character),
                }
            },
            BraillifyError::InvalidSymbolCharacter { character, position } => {
                match position {
                    Some(pos) => write!(f, "Invalid symbol character '{}' at position {}", character, pos),
                    None => write!(f, "Invalid symbol character '{}'", character),
                }
            },
            BraillifyError::InvalidMathSymbolCharacter { character, position } => {
                match position {
                    Some(pos) => write!(f, "Invalid math symbol character '{}' at position {}", character, pos),
                    None => write!(f, "Invalid math symbol character '{}'", character),
                }
            },
            BraillifyError::InvalidShortcutCharacter { character, position } => {
                match position {
                    Some(pos) => write!(f, "Invalid shortcut character '{}' at position {}", character, pos),
                    None => write!(f, "Invalid shortcut character '{}'", character),
                }
            },
            BraillifyError::InvalidFractionPart { part_name, character, position } => {
                match position {
                    Some(pos) => write!(f, "Invalid {} part (non-ascii digit): '{}' at position {}", part_name, character, pos),
                    None => write!(f, "Invalid {} part (non-ascii digit): '{}'", part_name, character),
                }
            },
            BraillifyError::InputTooLong { length, max_length } => {
                write!(f, "Input text length ({}) exceeds maximum allowed length ({})", length, max_length)
            },
            BraillifyError::FractionParseError { input, error } => {
                write!(f, "Fraction parse error for '{}': {}", input, error)
            },
            BraillifyError::Other { message, context } => {
                write!(f, "Error: {} (context: {})", message, context)
            },
        }
    }
}

impl std::error::Error for BraillifyError {}