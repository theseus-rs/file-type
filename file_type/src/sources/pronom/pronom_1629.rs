use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1629: FileFormat = FileFormat {
    id: 1_629,
    source_type: SourceType::Pronom,
    name: "MATLAB Mat File",
    extensions: &["mat", "fig"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x41, 0x54, 0x4C, 0x41, 0x42, 0x20, 0x37, 0x2E, 0x33]),
                    Token::WildcardCount(114),
                    Token::Any(&[
                        &[Token::Literal(&[0x00, 0x02, 0x49, 0x4D])],
                        &[Token::Literal(&[0x02, 0x00, 0x4D, 0x49])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_375,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_606,
        },
    ],
};
