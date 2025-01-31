use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_736: FileFormat = FileFormat {
    id: 1_535,
    puid: "fmt/736",
    name: "ClarisWorks",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x01]),
                    Token::WildcardCount(3),
                    Token::Literal(&[0x42, 0x4F, 0x42, 0x4F]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_536,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
