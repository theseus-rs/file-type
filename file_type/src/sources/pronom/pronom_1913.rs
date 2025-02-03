use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1913: FileFormat = FileFormat {
    id: 1_913,
    source_type: SourceType::Pronom,
    name: "Hierarchical File System",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(1_024),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x42, 0x44]),
                    Token::WildcardCount(12),
                    Token::Literal(&[0x00, 0x03]),
                    Token::WildcardCount(6),
                    Token::NotLiteral(&[]),
                    Token::Literal(&[0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
