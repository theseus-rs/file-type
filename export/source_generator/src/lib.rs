mod generator;
mod source;

use file_type::format::{ByteSequence, FileFormat, Regex, RelatedFormat, Signature, Token};

pub use generator::generate;
pub use source::Source;

impl Source for ByteSequence {
    fn to_source(&self) -> String {
        let offset = match &self.offset {
            Some(offset) => format!("Some({})", offset.to_source()),
            None => "None".to_string(),
        };

        format!(
            "ByteSequence {{ position_type: PositionType::{:?}, offset: {}, regex: {} }}",
            self.position_type,
            offset,
            self.regex.to_source(),
        )
    }
}

impl Source for FileFormat {
    fn to_source(&self) -> String {
        let extensions = self
            .extensions
            .iter()
            .map(|extension| format!("{extension:?}"))
            .collect::<Vec<String>>()
            .join(", ");
        let media_types = self
            .media_types
            .iter()
            .map(|media_type| format!("{media_type:?}"))
            .collect::<Vec<String>>()
            .join(", ");
        format!(
            "FileFormat {{ id: {}, source_type: SourceType::{:?}, name: {:?}, extensions: &[{}], media_types: &[{}], signatures: &[{}], related_formats: &[{}] }}",
            self.id.to_source(),
            self.source_type,
            self.name,
            extensions,
            media_types,
            self.signatures.iter().map(Source::to_source).collect::<Vec<String>>().join(", "),
            self.related_formats.iter().map(Source::to_source).collect::<Vec<String>>().join(", "),
        )
    }
}

impl Source for Regex {
    fn to_source(&self) -> String {
        let tokens = self
            .tokens
            .iter()
            .map(Source::to_source)
            .collect::<Vec<String>>()
            .join(", ");
        format!("Regex {{ tokens: &[{tokens}] }}")
    }
}

impl Source for RelatedFormat {
    fn to_source(&self) -> String {
        format!(
            "RelatedFormat {{ relationship_type: RelationshipType::{:?}, id: {} }}",
            self.relationship_type,
            self.id.to_source(),
        )
    }
}

impl Source for Signature {
    fn to_source(&self) -> String {
        format!(
            "Signature {{ byte_sequences: &[{}] }}",
            self.byte_sequences
                .iter()
                .map(Source::to_source)
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Source for Token {
    fn to_source(&self) -> String {
        match self {
            Token::Any(any_tokens) => {
                let mut source_tokens = Vec::new();
                for tokens in *any_tokens {
                    let tokens = tokens
                        .iter()
                        .map(Source::to_source)
                        .collect::<Vec<String>>()
                        .join(", ");
                    source_tokens.push(format!("&[{tokens}]"));
                }
                format!("Token::Any(&[{}])", source_tokens.join(", "))
            }
            Token::AnyWildcard => "Token::AnyWildcard".to_string(),
            Token::Literal(bytes) => {
                format!("Token::Literal(&{})", bytes.to_source())
            }
            Token::NotLiteral(bytes) => {
                format!("Token::NotLiteral(&{})", bytes.to_source())
            }
            Token::NotRange(start, end) => {
                format!(
                    "Token::NotRange(&{}, &{})",
                    start.to_source(),
                    end.to_source()
                )
            }
            Token::Range(start, end) => {
                format!("Token::Range(&{}, &{})", start.to_source(), end.to_source())
            }
            Token::SingleWildcard => "Token::SingleWildcard".to_string(),
            Token::WildcardCount(count) => {
                format!("Token::WildcardCount({})", (*count).to_source())
            }
            Token::WildcardCountRange(min, max) => {
                format!(
                    "Token::WildcardCountRange({}, {})",
                    (*min).to_source(),
                    (*max).to_source()
                )
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use file_type::format::{ByteSequence, PositionType, RelationshipType, SourceType};

    #[test]
    fn test_byte_sequence_to_source() {
        let regex = Regex::new("*").expect("Invalid regex");
        let byte_sequence = ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex,
        };
        assert_eq!(
            byte_sequence.to_source(),
            "ByteSequence { position_type: PositionType::BOF, offset: Some(0), regex: Regex { tokens: &[Token::AnyWildcard] } }"
        );
    }

    #[test]
    fn test_file_format_to_source() {
        let file_format = FileFormat {
            id: 664,
            source_type: SourceType::Default,
            name: "Portable Network Graphics",
            extensions: &["png"],
            media_types: &["image/png"],
            signatures: &[],
            related_formats: &[],
        };

        assert_eq!(
            file_format.to_source(),
            "FileFormat { id: 664, source_type: SourceType::Default, name: \"Portable Network Graphics\", extensions: &[\"png\"], media_types: &[\"image/png\"], signatures: &[], related_formats: &[] }"
        );
    }

    #[test]
    fn test_regex_source() -> file_type::Result<()> {
        let pattern = "(01)";
        let regex = Regex::new(pattern)?;
        assert_eq!(
            regex.to_source(),
            "Regex { tokens: &[Token::Any(&[&[Token::Literal(&[0x01])]])] }"
        );
        Ok(())
    }

    #[test]
    fn test_related_format_to_source() {
        let related_format = RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1617,
        };
        assert_eq!(
            related_format.to_source(),
            "RelatedFormat { relationship_type: RelationshipType::HasPriorityOver, id: 1_617 }"
        );
    }

    #[test]
    fn test_signature_to_source() {
        let internal_signature = Signature {
            byte_sequences: &[],
        };
        assert_eq!(
            internal_signature.to_source(),
            "Signature { byte_sequences: &[] }"
        );
    }

    #[test]
    fn test_token_source_any() {
        let token = Token::Any(&[&[Token::Literal(&[0x01])]]);
        assert_eq!(
            token.to_source(),
            "Token::Any(&[&[Token::Literal(&[0x01])]])"
        );
    }

    #[test]
    fn test_token_source_any_multiple() {
        let token = Token::Any(&[
            &[Token::Literal(&[0x01, 0x02])],
            &[Token::Range(&[0x03], &[0x04])],
            &[
                Token::Literal(&[0x05, 0x06]),
                Token::Range(&[0x07], &[0x08]),
            ],
        ]);
        assert_eq!(
            token.to_source(),
            "Token::Any(&[&[Token::Literal(&[0x01, 0x02])], &[Token::Range(&[0x03], &[0x04])], &[Token::Literal(&[0x05, 0x06]), Token::Range(&[0x07], &[0x08])]])",
        );
    }

    #[test]
    fn test_token_source_any_wildcard() {
        let token = Token::AnyWildcard;
        assert_eq!(token.to_source(), "Token::AnyWildcard");
    }

    #[test]
    fn test_token_source_literal_sequence() {
        let token = Token::Literal(&[0x01, 0x02, 0x03]);
        assert_eq!(token.to_source(), "Token::Literal(&[0x01, 0x02, 0x03])");
    }

    #[test]
    fn test_token_source_not_literal_sequence() {
        let token = Token::NotLiteral(&[0x01]);
        assert_eq!(token.to_source(), "Token::NotLiteral(&[0x01])");
    }

    #[test]
    fn test_token_source_not_literal_sequence_multiple() {
        let token = Token::NotLiteral(&[0x01, 0x02, 0x03]);
        assert_eq!(token.to_source(), "Token::NotLiteral(&[0x01, 0x02, 0x03])");
    }

    #[test]
    fn test_token_source_not_range() {
        let token = Token::NotRange(&[0x00], &[0xFF]);
        assert_eq!(token.to_source(), "Token::NotRange(&[0x00], &[0xFF])");
    }

    #[test]
    fn test_token_source_range() {
        let token = Token::Range(&[0x00], &[0xFF]);
        assert_eq!(token.to_source(), "Token::Range(&[0x00], &[0xFF])");
    }

    #[test]
    fn test_token_source_single_wildcard() {
        let token = Token::SingleWildcard;
        assert_eq!(token.to_source(), "Token::SingleWildcard");
    }

    #[test]
    fn test_token_source_wildcard_count() {
        let token = Token::WildcardCount(1_234);
        assert_eq!(token.to_source(), "Token::WildcardCount(1_234)");
    }

    #[test]
    fn test_token_source_wildcard_count_range() {
        let token = Token::WildcardCountRange(1, 2);
        assert_eq!(token.to_source(), "Token::WildcardCountRange(1, 2)");
    }

    #[test]
    fn test_token_source_wildcard_count_range_max() {
        let token = Token::WildcardCountRange(1234, usize::MAX);
        // The max value is u32::MAX to support 32-bit systems
        assert_eq!(
            token.to_source(),
            "Token::WildcardCountRange(1_234, 4_294_967_295)"
        );
    }
}
