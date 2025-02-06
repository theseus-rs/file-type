use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2520: FileFormat = FileFormat {
    id: 2_520,
    source_type: SourceType::Pronom,
    name: "EndNote Library",
    extensions: &["enl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x53, 0x51, 0x4C, 0x69, 0x74, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61,
                        0x74, 0x20, 0x33, 0x00,
                    ]),
                    Token::WildcardCountRange(1, 2_048),
                    Token::Literal(&[0x65, 0x6E, 0x6C, 0x5F, 0x72, 0x65, 0x66, 0x73]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_528,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_518,
        },
    ],
};
