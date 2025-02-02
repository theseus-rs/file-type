use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2701: FileFormat = FileFormat {
    id: 2_701,
    source_type: SourceType::Pronom,
    name: "General Purpose RAW",
    extensions: &["gpr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                    Token::WildcardCountRange(0, 4_080),
                    Token::Literal(&[
                        0x12, 0xC6, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x03, 0x00, 0x00,
                    ]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x56, 0x43, 0x2D, 0x35]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_225,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 1_225,
        },
    ],
};
