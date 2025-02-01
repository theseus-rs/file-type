use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_215: FileFormat = FileFormat {
    id: 304,
    puid: "x-fmt/215",
    name: "GEM Metafile Format",
    extensions: &["gem"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xFF, 0xFF, 0x00, 0x18, 0x00, 0x64, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x02])], &[Token::Literal(&[0x00])]]),
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
                        Token::Literal(&[0xFF, 0xFF, 0x18, 0x00, 0x64, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x02])], &[Token::Literal(&[0x00])]]),
                        Token::Literal(&[0x00]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[RelatedFormat {
        id: 1_329,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
