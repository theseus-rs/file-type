use file_type::format::{Regex, Signature, Token};

pub trait TestSignature {
    /// Generate the source code.
    fn to_test_signature(&self) -> Vec<u8>;
}

impl TestSignature for Signature {
    /// Generate matching test data for the signature
    #[must_use]
    fn to_test_signature(&self) -> Vec<u8> {
        let mut signature = Vec::new();
        for byte_sequence in self.byte_sequences {
            let offset = byte_sequence.offset.unwrap_or(0);
            signature.resize(signature.len() + offset, 0x00);
            signature.extend(byte_sequence.regex.to_test_signature());
        }
        signature
    }
}

impl TestSignature for Regex {
    /// Generate matching test data for the regex
    #[must_use]
    fn to_test_signature(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for token in self.tokens {
            data.extend(token.to_test_signature());
        }
        data
    }
}

impl TestSignature for Token {
    /// Generate matching test data for the token
    #[must_use]
    fn to_test_signature(&self) -> Vec<u8> {
        match self {
            Token::Any(any_tokens) => {
                let mut data = Vec::new();
                for tokens in *any_tokens {
                    for token in *tokens {
                        data.extend(token.to_test_signature());
                    }
                }
                data
            }
            Token::AnyWildcard => vec![0x01, 0x02, 0x03],
            Token::Literal(bytes) => bytes.to_vec(),
            Token::NotLiteral(bytes) => {
                let mut data = Vec::new();
                for byte in *bytes {
                    data.push(byte.wrapping_add(1));
                }
                data
            }
            Token::NotRange(_start, end) => {
                let mut data = Vec::new();
                for byte in *end {
                    data.push(byte.wrapping_add(1));
                }
                data
            }
            Token::Range(start, _end) => start.to_vec(),
            Token::SingleWildcard => vec![0x00],
            Token::WildcardCount(count) => vec![0x00; *count],
            Token::WildcardCountRange(min, _max) => vec![0x00; *min],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_test_data() -> file_type::Result<()> {
        let regex = Regex::new("0102[!03]{1-2}07??42")?;
        assert_eq!(
            regex.to_test_signature(),
            vec![0x01, 0x02, 0x04, 0x00, 0x07, 0x00, 0x42]
        );
        Ok(())
    }

    #[test]
    fn test_token_test_data_any() {
        let token = Token::Any(&[&[Token::Literal(&[0x01])]]);
        assert_eq!(token.to_test_signature(), vec![0x01]);
    }

    #[test]
    fn test_token_test_data_any_multiple() {
        let token = Token::Any(&[
            &[Token::Literal(&[0x01, 0x02])],
            &[Token::Range(&[0x03], &[0x04])],
            &[
                Token::Literal(&[0x05, 0x06]),
                Token::Range(&[0x07], &[0x08]),
            ],
        ]);
        assert_eq!(
            token.to_test_signature(),
            vec![0x01, 0x02, 0x03, 0x05, 0x06, 0x07]
        );
    }

    #[test]
    fn test_token_test_data_any_wildcard() {
        let token = Token::AnyWildcard;
        assert_eq!(token.to_test_signature(), vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_token_test_data_literal_sequence() {
        let token = Token::Literal(&[0x01, 0x02, 0x03]);
        assert_eq!(token.to_test_signature(), vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_token_test_data_not_literal_sequence() {
        let token = Token::NotLiteral(&[0x01]);
        assert_eq!(token.to_test_signature(), vec![0x02]);
    }

    #[test]
    fn test_token_test_data_not_literal_sequence_multiple() {
        let token = Token::NotLiteral(&[0x01, 0x02, 0x03]);
        assert_eq!(token.to_test_signature(), vec![0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_token_test_data_not_range() {
        let token = Token::NotRange(&[0x00], &[0x41]);
        assert_eq!(token.to_test_signature(), vec![0x42]);
    }

    #[test]
    fn test_token_test_data_range() {
        let token = Token::Range(&[0x00], &[0xFF]);
        assert_eq!(token.to_test_signature(), vec![0x00]);
    }

    #[test]
    fn test_token_test_data_single_wildcard() {
        let token = Token::SingleWildcard;
        assert_eq!(token.to_test_signature(), vec![0x00]);
    }

    #[test]
    fn test_token_test_data_wildcard_count() {
        let token = Token::WildcardCount(1);
        assert_eq!(token.to_test_signature(), vec![0x00]);
    }

    #[test]
    fn test_token_test_data_wildcard_count_range() {
        let token = Token::WildcardCountRange(2, 4);
        assert_eq!(token.to_test_signature(), vec![0x00, 0x00]);
    }
}
