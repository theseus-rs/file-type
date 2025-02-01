use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_189: FileFormat = FileFormat {
    id: 910,
    puid: "fmt/189",
    name: "Microsoft Office Open XML",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x50, 0x4B, 0x03, 0x04]),
                    Token::WildcardCount(26),
                    Token::Literal(&[
                        0x5B, 0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x5F, 0x54, 0x79, 0x70,
                        0x65, 0x73, 0x5D, 0x2E, 0x78, 0x6D, 0x6C, 0x20, 0xA2,
                    ]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x50, 0x4B, 0x01, 0x02]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x50, 0x4B, 0x05, 0x06]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 382,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
