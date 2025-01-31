use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1260: FileFormat = FileFormat {
    id: 2_078,
    puid: "fmt/1260",
    name: "SketchUp Document",
    extensions: &["skp", "skb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0xFF, 0xFE, 0xFF, 0x0E, 0x53, 0x00, 0x6B, 0x00, 0x65, 0x00, 0x74, 0x00,
                        0x63, 0x00, 0x68, 0x00, 0x55, 0x00, 0x70, 0x00, 0x20, 0x00, 0x4D, 0x00,
                        0x6F, 0x00, 0x64, 0x00, 0x65, 0x00, 0x6C, 0x00, 0xFF, 0xFE, 0xFF,
                    ]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0x7B, 0x00, 0x32]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 866,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 867,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
