use crate::{Error, Result};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Display;
use core::str::from_utf8;

pub const UNIDENTIFIED_KEY: u64 = 0x42;

/// A token to match against a byte stream
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Any(&'static [&'static [Token]]),
    AnyWildcard,
    Literal(&'static [u8]),
    NotLiteral(&'static [u8]),
    NotRange(&'static [u8], &'static [u8]),
    Range(&'static [u8], &'static [u8]),
    SingleWildcard,
    WildcardCount(usize),
    WildcardCountRange(usize, usize),
}

impl Display for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Token::Any(any_tokens) => {
                write!(f, "(")?;
                for (index, tokens) in any_tokens.iter().enumerate() {
                    if index > 0 {
                        write!(f, "|")?;
                    }
                    for token in *tokens {
                        write!(f, "{token}")?;
                    }
                }
                write!(f, ")")
            }
            Token::AnyWildcard => write!(f, "*"),
            Token::Literal(bytes) => {
                write!(
                    f,
                    "{}",
                    bytes
                        .iter()
                        .fold(String::new(), |acc, b| acc + &format!("{b:02X}"))
                )
            }
            Token::NotLiteral(bytes) => {
                write!(
                    f,
                    "[!{}]",
                    bytes
                        .iter()
                        .fold(String::new(), |acc, b| acc + &format!("{b:02X}"))
                )
            }
            Token::NotRange(start, end) => {
                let start = start
                    .iter()
                    .fold(String::new(), |acc, b| acc + &format!("{b:02X}"));
                let end = end
                    .iter()
                    .fold(String::new(), |acc, b| acc + &format!("{b:02X}"));
                write!(f, "[!{start}:{end}]")
            }
            Token::Range(start, end) => {
                let start = start
                    .iter()
                    .fold(String::new(), |acc, b| acc + &format!("{b:02X}"));
                let end = end
                    .iter()
                    .fold(String::new(), |acc, b| acc + &format!("{b:02X}"));
                write!(f, "[{start}:{end}]")
            }
            Token::SingleWildcard => write!(f, "??"),
            Token::WildcardCount(count) => write!(f, "{{{count}}}"),
            Token::WildcardCountRange(min, max) => {
                if *max == usize::MAX {
                    write!(f, "{{{min}-*}}")
                } else {
                    write!(f, "{{{min}-{max}}}")
                }
            }
        }
    }
}

/// A pattern to match against a byte stream
///
/// See: [automatic_format_identification](https://www.nationalarchives.gov.uk/aboutapps/fileformat/pdf/automatic_format_identification.pdf)
///
/// ```none
/// <byte_sequence> ::= <fixed_subsequence> | <byte_sequence>
/// <variable_wildcard> <fixed_subsequence>
/// <fixed_subsequence> ::= <chunk> | <fixed_subsequence> <chunk>
/// <chunk> ::= <byte_subsequence> | <wildcard>
/// <wild_card> ::= ‘??’ | ‘{‘ <integer> ‘}’ | ‘{‘ <integer> ‘-‘
/// <integer> ‘}’ | ‘(‘ <byte_subsequence> ‘|’ <byte_subsequence>
/// ‘)’ | ‘[‘ <byte_subsequence> ‘:’ <byte_subsequence> ‘]’ | ‘[!‘
/// <byte_subsequence> ‘:’ <byte_subsequence> ‘]’ | ‘[!‘
/// <byte_subsequence> ‘]’
/// <variable_wildcard> ::= ‘*’ | ‘{‘ <integer> ‘-‘ ‘*’ ‘}’
/// <byte_subsequence> ::= <byte> | <byte_subsequence> <byte>
/// <byte> ::= <hexadecimal_digit> <hexadecimal_digit>
/// <hexadecimal_digit> ::= <digit> | <letter>
/// <integer> ::= <digit> | <integer> <digit>
/// <digit> ::= ‘0’ | ‘1’ | ‘2’ | ‘3’ | ‘4’ | ‘5’ | ‘6’ | ‘7’ | ‘8’ | ‘9’
/// <letter> ::= ‘A’ | ‘B’ | ‘C’ | ‘D’ | ‘E’ | ‘F’
/// ```
///
/// The value of the byte sequence is defined as a sequence of hexadecimal values, optionally
/// incorporating any of the following regular expressions:
///
/// ```none
/// • ??: wildcard matching any pair of hexadecimal values (i.e. a single byte).
/// e.g.: 0x0A FF ?? FE would match 0x0A FF 6C FE or 0x0A FF 11 FE.
///
/// • *: wildcard matching any number of bytes (0 or more).
/// e.g.: 0x0A FF * FE would match 0x0A FF 6C FE or 0x0A FF 6C 11 FE.
///
/// • {n}: wildcard matching n bytes, where n is an integer.
/// e.g.: 0x1C 20 {2} 4E 12 would match 0x1C 20 FF 15 4E 12.
///
/// • {m-n}: wildcard matching between m-n bytes inclusive, where m and n are integers or ‘*’.
/// e.g.: 0x03 {1-2} 4D would match 0x03 3C 4D or 0x03 3C 88 4D.
/// e.g.: 0x03 {2-*} 4D would match 0x03 3C 88 4D or 0x03 3C 88 3F 4D.
///
/// • (a|b): wildcard matching one from a list of values (e.g. a or b), where each value is a
/// hexadecimal byte sequence of arbitrary length containing no wildcards.
/// e.g.: 0x0E (FF|FE) 17 would match 0x0E FF 17 or 0x0E FE 17.
///
/// • [a:b]: wildcard matching any sequence of bytes which lies lexicographically between a
/// and b, inclusive (where both a and b are byte sequences of the same length, containing no
/// wildcards, and where a is less than b). The endian-ness of a and b are the same as the
/// endian-ness of the signature as a whole.
/// e.g. 0xFF [09:0B] FF would match 0xFF 09 FF, 0xFF 0A FF or 0xFF 0B FF.
///
/// • [!a]: wildcard matching any sequence of bytes other than a itself (where a is a byte
/// sequence containing no wildcards).
/// e.g. 0xFF [!09] FF would match 0xFF 0A FF, but not 0xFF 09 FF.
///
/// • [!a:b]: wildcard matching any sequence of bytes which does not lie lexicographically
/// between a and b, inclusive (where a and b are both byte sequences of the same length,
/// containing no wildcards, and where a is less than b).
/// e.g. 0xFF [!01:02] FF would match 0xFF 00 FF and 0xFF 03 FF, but not 0xFF 01
/// FF or 0xFF 02 FF.
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Regex {
    pub tokens: &'static [Token],
}

impl Regex {
    /// Create a new instance of the `Regex` struct for the given pattern
    ///
    /// # Errors
    /// if there is a syntax error in the pattern
    pub fn new<S: AsRef<str>>(pattern: S) -> Result<Self> {
        let pattern = pattern.as_ref();
        let bytes = pattern.as_bytes();
        let mut tokens = Vec::new();
        Self::tokenize(bytes, &mut tokens)?;
        Ok(Self {
            tokens: Box::leak(tokens.into_boxed_slice()),
        })
    }

    /// Calculate a key from the byte sequence.  The key is the first 8 bytes of the sequence
    /// converted to a u64.
    #[must_use]
    pub fn key_from_bytes(bytes: &[u8]) -> u64 {
        let mut array = [0u8; 8];
        for (i, &byte) in bytes.iter().take(8).enumerate() {
            array[i] = byte;
        }
        u64::from_be_bytes(array)
    }

    /// Get the key used to store the pattern in a map
    #[must_use]
    pub fn key(&self) -> u64 {
        match self.tokens.first() {
            Some(Token::Literal(bytes)) => Regex::key_from_bytes(bytes),
            _ => UNIDENTIFIED_KEY,
        }
    }

    /// Tokenize the pattern
    ///
    /// # Errors
    /// if there is a syntax error in the pattern
    #[expect(clippy::too_many_lines)]
    fn tokenize(bytes: &[u8], tokens: &mut Vec<Token>) -> Result<()> {
        let mut byte_index = 0;

        while byte_index < bytes.len() {
            match bytes[byte_index] {
                b'(' => {
                    let start = byte_index + 1;
                    let end = bytes[start..]
                        .iter()
                        .position(|b| *b == b')')
                        .map(|pos| start + pos)
                        .ok_or(Error::new(format!(
                            "Invalid pattern [{byte_index}]; expected ')' after '('"
                        )))?;
                    let values = bytes[start..end]
                        .split(|byte| *byte == b'|')
                        .map(<[u8]>::to_vec)
                        .collect::<Vec<Vec<u8>>>();
                    let mut any_tokens: Vec<&[Token]> = Vec::new();
                    for value in values {
                        let mut token_holder = Vec::new();
                        Self::tokenize(&value, &mut token_holder)?;
                        any_tokens.push(Box::leak(token_holder.clone().into_boxed_slice()));
                    }
                    tokens.push(Token::Any(Box::leak(any_tokens.into_boxed_slice())));
                    byte_index = end;
                }
                b'*' => tokens.push(Token::AnyWildcard),
                b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f' => {
                    let start = byte_index;
                    let end = bytes[start..]
                        .iter()
                        .position(|b| !b.is_ascii_hexdigit())
                        .map_or(bytes.len(), |pos| start + pos);
                    byte_index = end - 1;
                    let literal_bytes = hex_to_bytes(&bytes[start..end]);
                    tokens.push(Token::Literal(literal_bytes));
                }
                b'[' => {
                    let mut start = byte_index + 1;
                    let negated = if bytes[start] == b'!' {
                        start += 1;
                        true
                    } else {
                        false
                    };

                    let mut end = 0;
                    let mut separator = 0;

                    for (index, byte) in bytes[start..].iter().enumerate() {
                        if *byte == b':' {
                            separator = index;
                        } else if *byte == b']' {
                            end = start + index;
                            byte_index = end;
                            break;
                        }
                    }
                    if end == 0 {
                        return Err(Error::new(format!(
                            "Invalid pattern [{byte_index}]; expected ']' after '['"
                        )));
                    }
                    let slice = &bytes[start..end];

                    if negated {
                        if separator > 0 {
                            let start_range = &slice[0..separator];
                            let end_range = &slice[separator + 1..];
                            let start_range = hex_to_bytes(start_range);
                            let end_range = hex_to_bytes(end_range);
                            tokens.push(Token::NotRange(start_range, end_range));
                        } else {
                            let literal_bytes = hex_to_bytes(slice);
                            tokens.push(Token::NotLiteral(literal_bytes));
                        }
                    } else if separator > 0 {
                        let start_range = &slice[0..separator];
                        let end_range = &slice[separator + 1..];
                        let start_range = hex_to_bytes(start_range);
                        let end_range = hex_to_bytes(end_range);
                        tokens.push(Token::Range(start_range, end_range));
                    } else {
                        return Err(Error::new(format!(
                            "Invalid pattern [{byte_index}]; expected two literals in range"
                        )));
                    }
                }
                b'?' => {
                    byte_index += 1;
                    if byte_index >= bytes.len() || bytes[byte_index] != b'?' {
                        return Err(Error::new(format!(
                            "Invalid pattern [{byte_index}]; expected '?' after '?'"
                        )));
                    }
                    tokens.push(Token::SingleWildcard);
                }
                b'{' => {
                    let start = byte_index + 1;
                    let mut end = 0;
                    let mut separator = 0;

                    for (index, byte) in bytes[start..].iter().enumerate() {
                        if *byte == b'-' {
                            separator = index;
                        } else if *byte == b'}' {
                            end = start + index;
                            byte_index = end;
                            break;
                        }
                    }
                    if end == 0 {
                        return Err(Error::new(format!(
                            "Invalid pattern [{byte_index}]; expected '}}' after '{{'"
                        )));
                    }
                    let slice = &bytes[start..end];

                    if separator > 0 {
                        let min_range = &slice[0..separator];
                        let max_range = &slice[separator + 1..];
                        let min_value =
                            from_utf8(min_range).map_err(|error| Error::new(error.to_string()))?;
                        let max_value =
                            from_utf8(max_range).map_err(|error| Error::new(error.to_string()))?;
                        let min_count = min_value
                            .parse()
                            .map_err(|_| {
                                Error::new(format!(
                                    "Invalid pattern [{byte_index}]; expected integer between '{{' and '}}'"
                                ))
                            })?;
                        if max_value == "*" {
                            tokens.push(Token::WildcardCountRange(min_count, usize::MAX));
                        } else {
                            let max_count = max_value
                                .parse()
                                .map_err(|_| {
                                    Error::new(format!(
                                        "Invalid pattern [{byte_index}]; expected integer between '{{' and '}}'"
                                    ))
                                })?;
                            tokens.push(Token::WildcardCountRange(min_count, max_count));
                        }
                    } else {
                        let value = from_utf8(&bytes[start..end])
                            .map_err(|error| Error::new(error.to_string()))?;
                        let count = value
                            .parse()
                            .map_err(|_| {
                                Error::new(format!(
                                    "Invalid pattern [{byte_index}]; expected integer between '(' and ')'"
                                ))
                            })?;
                        tokens.push(Token::WildcardCount(count));
                    }
                }
                _ => {
                    return Err(Error::new(format!(
                        "Invalid pattern [{byte_index}]; unexpected character [{}]",
                        bytes[byte_index]
                    )));
                }
            }
            byte_index += 1;
        }
        Ok(())
    }

    /// Check if the haystack matches the pattern
    #[must_use]
    pub fn is_match(&self, haystack: &[u8]) -> bool {
        self.is_match_at(haystack, 0)
    }

    /// Check if the haystack matches the pattern at the given offset
    #[must_use]
    pub fn is_match_at(&self, haystack: &[u8], start: usize) -> bool {
        let (matched, _haystack_index) = Self::tokens_match_at(self.tokens, haystack, start);
        matched
    }

    /// Check if the haystack matches the pattern at the given offset
    #[expect(clippy::too_many_lines)]
    fn tokens_match_at(tokens: &[Token], haystack: &[u8], start_offset: usize) -> (bool, usize) {
        let mut token_index = 0;
        let mut haystack_index = start_offset;
        while let Some(token) = tokens.get(token_index) {
            match token {
                Token::Any(any_tokens) => {
                    if haystack_index >= haystack.len() {
                        return (false, 0);
                    }
                    let matched = any_tokens.iter().any(|any_tokens_list| {
                        let (matched, matched_haystack_index) =
                            Self::tokens_match_at(any_tokens_list, haystack, haystack_index);
                        if matched {
                            haystack_index = matched_haystack_index;
                            true
                        } else {
                            false
                        }
                    });
                    if !matched {
                        return (false, 0);
                    }
                }
                Token::AnyWildcard => {
                    if token_index == tokens.len() - 1 {
                        return (true, haystack_index);
                    }
                    let remaining_tokens = &tokens[token_index + 1..];
                    while haystack_index < haystack.len() {
                        let (matched, new_haystack_index) =
                            Self::tokens_match_at(remaining_tokens, haystack, haystack_index);
                        if matched {
                            return (true, new_haystack_index);
                        }
                        haystack_index += 1;
                    }
                    return (false, 0);
                }
                Token::Literal(bytes) => {
                    if haystack_index + bytes.len() > haystack.len() {
                        return (false, 0);
                    }
                    let data_slice = &haystack[haystack_index..haystack_index + bytes.len()];
                    if data_slice != *bytes {
                        return (false, 0);
                    }
                    haystack_index += bytes.len();
                }
                Token::NotLiteral(bytes) => {
                    if haystack_index + bytes.len() > haystack.len() {
                        return (false, 0);
                    }
                    let data_slice = &haystack[haystack_index..haystack_index + bytes.len()];
                    if data_slice == *bytes {
                        return (false, 0);
                    }
                    haystack_index += bytes.len();
                }
                Token::Range(start, end) => {
                    if haystack_index >= haystack.len() {
                        return (false, 0);
                    }
                    let value_length = start.len();
                    let value = &haystack[haystack_index..haystack_index + value_length];
                    if value.cmp(start).is_lt() || value.cmp(end).is_gt() {
                        return (false, 0);
                    }
                    haystack_index += 1;
                }
                Token::NotRange(start, end) => {
                    if haystack_index >= haystack.len() {
                        return (false, 0);
                    }
                    let value_length = start.len();
                    let value = &haystack[haystack_index..haystack_index + value_length];
                    if value.cmp(start).is_ge() && value.cmp(end).is_le() {
                        return (false, 0);
                    }
                    haystack_index += 1;
                }
                Token::SingleWildcard => {
                    if haystack_index >= haystack.len() {
                        return (false, 0);
                    }
                    haystack_index += 1;
                }
                Token::WildcardCount(count) => {
                    if haystack_index + count > haystack.len() {
                        return (false, 0);
                    }
                    haystack_index += count;
                }
                Token::WildcardCountRange(min_count, max_count) => {
                    let data_length = haystack.len();
                    if data_length < haystack_index + *min_count {
                        return (false, 0);
                    }
                    haystack_index += *min_count;
                    let max_index = if *max_count == usize::MAX {
                        data_length
                    } else {
                        haystack_index + *max_count
                    };
                    let remaining_tokens = &tokens[token_index + 1..];
                    while haystack_index < max_index {
                        let (matched, matched_haystack_index) =
                            Self::tokens_match_at(remaining_tokens, haystack, haystack_index);
                        if matched {
                            haystack_index += matched_haystack_index;
                            return (true, haystack_index);
                        }
                        haystack_index += 1;
                    }
                    return (false, 0);
                }
            }
            token_index += 1;
        }
        (true, haystack_index)
    }
}

/// Convert a byte array of hexadecimal values to a byte array
/// ```none
/// e.g. [0x30, 0x31] -> [0x01]
/// ```
fn hex_to_bytes(hex: &[u8]) -> &'static [u8] {
    let bytes = hex
        .chunks_exact(2)
        .filter_map(|chunk| {
            from_utf8(chunk)
                .ok()
                .and_then(|s| u8::from_str_radix(s, 16).ok())
        })
        .collect::<Vec<u8>>();
    Box::leak(bytes.into_boxed_slice())
}

impl Display for Regex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for token in self.tokens {
            write!(f, "{token}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_display_any() {
        let token = Token::Any(&[&[Token::Literal(&[0x01])]]);
        assert_eq!(token.to_string(), "(01)");
    }

    #[test]
    fn test_token_display_any_multiple() {
        let token = Token::Any(&[
            &[Token::Literal(&[0x01, 0x02])],
            &[Token::Range(&[0x03], &[0x04])],
            &[
                Token::Literal(&[0x05, 0x06]),
                Token::Range(&[0x07], &[0x08]),
            ],
        ]);
        assert_eq!(token.to_string(), "(0102|[03:04]|0506[07:08])");
    }

    #[test]
    fn test_token_display_any_wildcard() {
        let token = Token::AnyWildcard;
        assert_eq!(token.to_string(), "*");
    }

    #[test]
    fn test_token_display_literal_sequence() {
        let token = Token::Literal(&[0x01, 0x02, 0x03]);
        assert_eq!(token.to_string(), "010203");
    }

    #[test]
    fn test_token_display_not_literal_sequence() {
        let token = Token::NotLiteral(&[0x01]);
        assert_eq!(token.to_string(), "[!01]");
    }

    #[test]
    fn test_token_display_not_literal_sequence_multiple() {
        let token = Token::NotLiteral(&[0x01, 0x02, 0x03]);
        assert_eq!(token.to_string(), "[!010203]");
    }

    #[test]
    fn test_token_display_not_range() {
        let token = Token::NotRange(&[0x00], &[0xFF]);
        assert_eq!(token.to_string(), "[!00:FF]");
    }

    #[test]
    fn test_token_display_range() {
        let token = Token::Range(&[0x00], &[0xFF]);
        assert_eq!(token.to_string(), "[00:FF]");
    }

    #[test]
    fn test_token_display_single_wildcard() {
        let token = Token::SingleWildcard;
        assert_eq!(token.to_string(), "??");
    }

    #[test]
    fn test_token_display_wildcard_count() {
        let token = Token::WildcardCount(1);
        assert_eq!(token.to_string(), "{1}");
    }

    #[test]
    fn test_token_display_wildcard_count_range() {
        let token = Token::WildcardCountRange(1, 2);
        assert_eq!(token.to_string(), "{1-2}");
    }

    #[test]
    fn test_token_display_wildcard_count_range_max() {
        let token = Token::WildcardCountRange(1, usize::MAX);
        assert_eq!(token.to_string(), "{1-*}");
    }

    #[test]
    fn test_regex_key() -> Result<()> {
        assert_eq!(Regex::new("")?.key(), UNIDENTIFIED_KEY);
        assert_eq!(Regex::new("00")?.key(), 0);
        assert_eq!(Regex::new("0000000000000001")?.key(), 1);
        assert_eq!(Regex::new("01")?.key(), 72_057_594_037_927_936);
        assert_eq!(Regex::new("00000000000000FF")?.key(), u64::from(u8::MAX));
        assert_eq!(Regex::new("FF")?.key(), 18_374_686_479_671_623_680);
        assert_eq!(Regex::new("000000000000FFFF")?.key(), u64::from(u16::MAX));
        assert_eq!(Regex::new("FFFF")?.key(), 18_446_462_598_732_840_960);
        assert_eq!(Regex::new("00000000FFFFFFFF")?.key(), u64::from(u32::MAX));
        assert_eq!(Regex::new("FFFFFFFF")?.key(), 18_446_744_069_414_584_320);
        assert_eq!(Regex::new("FFFFFFFFFFFFFFFF")?.key(), u64::MAX);
        Ok(())
    }

    #[test]
    fn test_regex_display_all_tokens() -> Result<()> {
        let pattern = "(0102|0304|[05:06])*010203[!01][!00:FF][00:FF]??{1}{1-2}{42-*}";
        let regex = Regex::new(pattern)?;
        assert_eq!(regex.to_string(), pattern);
        Ok(())
    }

    fn matches_pattern(pattern: &str, data: &str) -> Result<bool> {
        let regex = Regex::new(pattern)?;
        let data = hex_to_bytes(data.as_bytes());
        let result = regex.is_match(data);
        Ok(result)
    }

    #[test]
    fn test_match_any() -> Result<()> {
        assert!(matches_pattern("(00)", "00")?);
        assert!(matches_pattern("(00|01)02", "0102")?);
        assert!(matches_pattern("01(00|02)", "0102")?);
        assert!(matches_pattern("0102(03|04)", "010203")?);
        assert!(matches_pattern("0102(03|FF)", "01020304")?);
        assert!(matches_pattern("(01|02|03|FF)", "FF")?);
        assert!(matches_pattern("(010203|FF)", "010203AA")?);
        assert!(matches_pattern("(010203|FF)", "FFAA")?);
        assert!(matches_pattern("(01[00:33]|FF)", "0102AA")?);
        assert!(!matches_pattern("(00|01)", "")?);
        assert!(!matches_pattern("(00|01)", "FF")?);
        assert!(!matches_pattern("01(02|03)", "01FF")?);
        assert!(!matches_pattern("01(02|03)03", "01FF03")?);
        assert!(!matches_pattern("0102(03|04|05)", "0102FF")?);
        assert!(!matches_pattern("(01[00:33]|00)", "01FFAA")?);

        // PRONOM examples
        assert!(matches_pattern("0E(FF|FE)17", "0EFF17")?);
        assert!(matches_pattern("0E(FF|FE)17", "0EFE17")?);
        Ok(())
    }

    #[test]
    fn test_match_any_wildcard() -> Result<()> {
        assert!(matches_pattern("*", "")?);
        assert!(matches_pattern("*", "00")?);
        assert!(matches_pattern("*02", "0102")?);
        assert!(matches_pattern("01*", "0102")?);
        assert!(matches_pattern("01*02", "0102")?);
        assert!(matches_pattern("0102*", "010203")?);
        assert!(matches_pattern("01*04", "01020304")?);
        assert!(!matches_pattern("*01", "FF")?);
        assert!(!matches_pattern("01*03", "0102FF")?);

        // PRONOM examples
        assert!(matches_pattern("0AFF*FE", "0AFF6CFE")?);
        assert!(matches_pattern("0AFF*FE", "0AFF11FE")?);
        Ok(())
    }

    #[test]
    fn test_match_literal() -> Result<()> {
        assert!(matches_pattern("00", "00")?);
        assert!(matches_pattern("0102", "0102")?);
        assert!(matches_pattern("010203", "010203")?);
        assert!(matches_pattern("010203", "01020304")?);
        assert!(!matches_pattern("00", "")?);
        assert!(!matches_pattern("00", "FF")?);
        assert!(!matches_pattern("0102", "01FF")?);
        assert!(!matches_pattern("010203", "01FF03")?);
        assert!(!matches_pattern("010203", "0102FF")?);
        Ok(())
    }

    #[test]
    fn test_match_not_literal_sequence() -> Result<()> {
        assert!(matches_pattern("[!00]", "FF")?);
        assert!(matches_pattern("[!00]02", "0102")?);
        assert!(matches_pattern("01[!00]", "01FF02")?);
        assert!(matches_pattern("0102[!00]", "01020304")?);
        assert!(matches_pattern("01[!00]03", "01FF03")?);
        assert!(matches_pattern("0102[!00]04", "01020304")?);
        assert!(!matches_pattern("[!00]", "")?);
        assert!(!matches_pattern("[!00]", "00")?);
        assert!(!matches_pattern("[!01]02", "01FF02")?);
        assert!(!matches_pattern("[!01]02", "01FF02")?);
        assert!(!matches_pattern("[!01]02", "01FF02")?);
        assert!(!matches_pattern("[!010203]FF", "010203FF")?);
        Ok(())
    }

    #[test]
    fn test_match_not_range() -> Result<()> {
        assert!(matches_pattern("[!AA:FF]", "01")?);
        assert!(matches_pattern("01[!AA:FF]", "0102")?);
        assert!(matches_pattern("[!AA:FF]01", "0101")?);
        assert!(matches_pattern("[!AA:FF]02", "010203")?);
        assert!(!matches_pattern("[!00:FF]", "")?);
        assert!(!matches_pattern("[!00:FF]", "00")?);
        assert!(!matches_pattern("[!00:FF]02", "0102")?);
        assert!(!matches_pattern("01[!02:03]", "0102")?);
        assert!(!matches_pattern("01[!01:02]", "0102")?);
        assert!(!matches_pattern("0102[!00:FF]", "010203")?);
        assert!(!matches_pattern("01[!00:FF]03", "010203")?);
        assert!(!matches_pattern("0102[!00:FF]04", "01020304")?);

        // PRONOM examples
        assert!(matches_pattern("FF[!01:02]FF", "FF00FF")?);
        assert!(matches_pattern("FF[!01:02]FF", "FF03FF")?);
        assert!(!matches_pattern("FF[!01:02]FF", "FF01FF")?);
        assert!(!matches_pattern("FF[!01:02]FF", "FF02FF")?);
        Ok(())
    }

    #[test]
    fn test_match_range() -> Result<()> {
        assert!(matches_pattern("[00:FF]", "00")?);
        assert!(matches_pattern("[00:FF]02", "0102")?);
        assert!(matches_pattern("01[02:03]", "0102")?);
        assert!(matches_pattern("01[01:02]", "0102")?);
        assert!(matches_pattern("0102[00:FF]", "010203")?);
        assert!(matches_pattern("01[00:FF]03", "010203")?);
        assert!(matches_pattern("0102[00:FF]04", "01020304")?);
        assert!(!matches_pattern("[00:FF]", "")?);
        assert!(!matches_pattern("[10:FF]", "01")?);
        assert!(!matches_pattern("01[00:FF]", "01")?);
        assert!(!matches_pattern("[00:FF]01", "01")?);
        assert!(!matches_pattern("[00:FF]01", "FF")?);

        // PRONOM examples
        assert!(matches_pattern("FF[09:0B]FF", "FF09FF")?);
        assert!(matches_pattern("FF[09:0B]FF", "FF0AFF")?);
        assert!(matches_pattern("FF[09:0B]FF", "FF0BFF")?);
        Ok(())
    }

    #[test]
    fn test_match_single_wildcard() -> Result<()> {
        assert!(matches_pattern("??", "00")?);
        assert!(matches_pattern("??02", "0102")?);
        assert!(matches_pattern("01??", "0102")?);
        assert!(matches_pattern("0102??", "010203")?);
        assert!(matches_pattern("??0203", "01020304")?);
        assert!(!matches_pattern("??", "")?);
        assert!(!matches_pattern("00??", "FF")?);

        // PRONOM examples
        assert!(matches_pattern("0AFF??FE", "0AFF6CFE")?);
        assert!(matches_pattern("0AFF??FE", "0AFF11FE")?);
        Ok(())
    }

    #[test]
    fn test_match_wildcard_count() -> Result<()> {
        assert!(matches_pattern("{1}", "00")?);
        assert!(matches_pattern("{1}02", "0102")?);
        assert!(matches_pattern("{2}", "0102")?);
        assert!(matches_pattern("01{1}", "0102")?);
        assert!(matches_pattern("01{1}03", "010203")?);
        assert!(matches_pattern("0102{1}", "010203")?);
        assert!(matches_pattern("01{2}04", "01020304")?);
        assert!(!matches_pattern("{1}", "")?);
        assert!(!matches_pattern("{2}", "01")?);
        assert!(!matches_pattern("01{1}", "01")?);
        assert!(!matches_pattern("{1}01", "01")?);

        // PRONOM examples
        assert!(matches_pattern("1C20{2}4E12", "1C20FF154E12")?);
        Ok(())
    }

    #[test]
    fn test_match_wildcard_count_range() -> Result<()> {
        assert!(matches_pattern("{1-2}", "00")?);
        assert!(matches_pattern("{1-2}02", "0102")?);
        assert!(matches_pattern("{1-2}03", "010203")?);
        assert!(matches_pattern("{2-*}", "01020304")?);
        assert!(matches_pattern("01{1-2}", "0102")?);
        assert!(matches_pattern("01{1-2}03", "010203")?);
        assert!(matches_pattern("0102{1-2}", "010203")?);
        assert!(matches_pattern("01{1-*}04", "01020304")?);
        assert!(matches_pattern("01{2-*}04", "01020304")?);
        assert!(!matches_pattern("{1-2}", "")?);
        assert!(!matches_pattern("{1-2}01", "01")?);
        assert!(!matches_pattern("01{1-2}", "01")?);
        assert!(!matches_pattern("01{1-2}02", "0102030405")?);

        // PRONOM examples
        assert!(matches_pattern("03{1-2}4D", "033C4D")?);
        assert!(matches_pattern("03{1-2}4D", "033C884D")?);
        assert!(matches_pattern("03{2-*}4D", "033C884D")?);
        assert!(matches_pattern("03{2-*}4D", "033C883F4D")?);
        Ok(())
    }

    #[test]
    fn test_invalid_open_any() {
        assert!(Regex::new("(01").is_err());
    }

    #[test]
    fn test_invalid_open_range() {
        assert!(Regex::new("[01").is_err());
    }

    #[test]
    fn test_invalid_range() {
        assert!(Regex::new("[01]").is_err());
    }

    #[test]
    fn test_invalid_single_wildcard() {
        assert!(Regex::new("?01").is_err());
    }

    #[test]
    fn test_invalid_single_wildcard_eol() {
        assert!(Regex::new("?").is_err());
    }

    #[test]
    fn test_invalid_open_wildcard() {
        assert!(Regex::new("{").is_err());
    }

    #[test]
    fn test_invalid_wildcard_minimum() {
        assert!(Regex::new("{FF}").is_err());
    }

    #[test]
    fn test_invalid_wildcard_minimum_separator() {
        assert!(Regex::new("{FF-1}").is_err());
    }

    #[test]
    fn test_invalid_wildcard_maximum() {
        assert!(Regex::new("{1-FF}").is_err());
    }

    #[test]
    fn test_invalid_characters() {
        for c in 0..=u8::MAX {
            let character = char::from_u32(u32::from(c)).unwrap_or_default();
            match character {
                '0'..='9' | 'A'..='F' | 'a'..='f' | '(' | '[' | '{' | '?' | '*' => {
                    continue;
                }
                _ => {
                    let pattern = character.to_string();
                    let regex = Regex::new(&pattern);
                    assert!(regex.is_err());
                }
            }
        }
    }
}
