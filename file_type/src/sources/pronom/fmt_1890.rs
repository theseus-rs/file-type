use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1890: FileFormat = FileFormat {
    id: 2_746,
    puid: "fmt/1890",
    name: "Memory Stick Voice File (MSV)",
    extensions: &["msv"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x53, 0x5F, 0x56, 0x4F, 0x49, 0x43, 0x45]),
                    Token::WildcardCount(4),
                    Token::Literal(&[
                        0x01, 0x00, 0x00, 0x00, 0x53, 0x4F, 0x4E, 0x59, 0x20, 0x43, 0x4F, 0x52,
                        0x50, 0x4F, 0x52, 0x41, 0x54, 0x49, 0x4F, 0x4E,
                    ]),
                    Token::WildcardCount(28),
                    Token::Literal(&[0x00]),
                    Token::Any(&[&[Token::Literal(&[0x05])], &[Token::Literal(&[0x09])]]),
                    Token::Literal(&[0x00, 0x01]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_259,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
