use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2136: FileFormat = FileFormat {
    id: 2_136,
    source_type: SourceType::Pronom,
    name: "QuarkXPress Document",
    extensions: &["qxd", "qxt", "qwd"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(2),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x4D, 0x58, 0x50, 0x52]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x3F]),
                    ],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(2),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x49, 0x49, 0x58, 0x50, 0x52]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x3F]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 255,
    }],
};
