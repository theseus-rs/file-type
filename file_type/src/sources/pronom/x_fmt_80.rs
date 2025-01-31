use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_80: FileFormat = FileFormat {
    id: 122,
    puid: "x-fmt/80",
    name: "Macintosh PICT Image",
    extensions: &["pct", "pict"],
    media_types: &["image/x-pict"],
    internal_signatures: &[
        InternalSignature {
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
        InternalSignature {
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
        id: 1_086,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
