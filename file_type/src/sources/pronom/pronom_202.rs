use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_202: FileFormat = FileFormat {
    id: 202,
    source_type: SourceType::Pronom,
    name: "Calendar Creator Plus Data File",
    extensions: &["cce"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xFF, 0xCC]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x29]),
                    Token::WildcardCount(43),
                    Token::Literal(&[0x65, 0x00, 0x7F, 0x01]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 2_113,
    }],
};
