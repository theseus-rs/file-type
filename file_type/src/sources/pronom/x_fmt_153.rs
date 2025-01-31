use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_153: FileFormat = FileFormat {
    id: 215,
    puid: "x-fmt/153",
    name: "Microsoft Windows Enhanced Metafile",
    extensions: &["emf"],
    media_types: &["image/emf"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x01, 0x00, 0x00, 0x00, 0x58, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(32),
                        Token::Literal(&[0x20, 0x45, 0x4D, 0x46, 0x00, 0x00, 0x01, 0x00]),
                        Token::WildcardCount(16),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
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
                        Token::Literal(&[0x01, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(36),
                        Token::Literal(&[0x20, 0x45, 0x4D, 0x46, 0x00, 0x00, 0x01, 0x00]),
                        Token::WildcardCount(16),
                        Token::Literal(&[0x58, 0x00, 0x00, 0x00]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[
        RelatedFormat {
            id: 1_089,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_090,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
    ],
};
