use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_122: FileFormat = FileFormat {
    id: 122,
    source_type: SourceType::Pronom,
    name: "Macintosh PICT Image",
    extensions: &["pct", "pict"],
    media_types: &["image/x-pict"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x44, 0x52, 0x57, 0x47]),
                        Token::Any(&[
                            &[Token::Literal(&[0x4D, 0x44])],
                            &[Token::Literal(&[0x44, 0x32])],
                        ]),
                        Token::WildcardCount(516),
                        Token::Literal(&[0x11, 0x01]),
                    ],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(516),
                        Token::Literal(&[0x11, 0x01]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 1_086,
    }],
};
