use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_921: FileFormat = FileFormat {
    id: 921,
    source_type: SourceType::Pronom,
    name: "Adobe InDesign Document",
    extensions: &["indd", "ind", "indt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x06, 0x06, 0xED, 0xF5, 0xD8, 0x1D, 0x46, 0xE5, 0xBD, 0x31, 0xEF, 0xE7,
                        0xFE, 0x74, 0xB7, 0x1D, 0x44, 0x4F, 0x43, 0x55, 0x4D, 0x45, 0x4E, 0x54,
                    ]),
                    Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x02])]]),
                    Token::WildcardCount(4),
                    Token::Any(&[
                        &[Token::Literal(&[0x03, 0x00, 0x00, 0x00])],
                        &[Token::Literal(&[0x00, 0x00, 0x00, 0x03])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 865,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_336,
        },
    ],
};
