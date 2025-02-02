use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_918: FileFormat = FileFormat {
    id: 918,
    source_type: SourceType::Pronom,
    name: "Digital Moving Picture Exchange Bitmap",
    extensions: &["dpx"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x53, 0x44, 0x50, 0x58]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x56, 0x31, 0x2E, 0x30]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x58, 0x50, 0x44, 0x53]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x56, 0x31, 0x2E, 0x30]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 1_328,
    }],
};
