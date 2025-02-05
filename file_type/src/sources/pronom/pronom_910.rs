use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_910: FileFormat = FileFormat {
    id: 910,
    source_type: SourceType::Pronom,
    name: "Microsoft Office Open XML",
    extensions: &[],
    media_types: &[],
    signatures: &[Signature {
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
        relationship_type: RelationshipType::HasPriorityOver,
        id: 382,
    }],
};
