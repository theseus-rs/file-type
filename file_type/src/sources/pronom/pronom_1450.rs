use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1450: FileFormat = FileFormat {
    id: 1_450,
    source_type: SourceType::Pronom,
    name: "QuarkXPress Project",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(2),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x4D, 0x58, 0x50, 0x52]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x49]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(2),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x49, 0x49, 0x58, 0x50, 0x52]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x49]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 255,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_484,
        },
    ],
};
