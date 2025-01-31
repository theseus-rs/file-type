use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_542: FileFormat = FileFormat {
    id: 1_329,
    puid: "fmt/542",
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
                        Token::Literal(&[0xFF, 0xFF, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x18])], &[Token::Literal(&[0x0E])]]),
                        Token::Literal(&[0x00, 0x65, 0x00]),
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
                        Token::Literal(&[0xFF, 0xFF]),
                        Token::Any(&[&[Token::Literal(&[0x18])], &[Token::Literal(&[0x0E])]]),
                        Token::Literal(&[0x00, 0x65, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x02])], &[Token::Literal(&[0x00])]]),
                        Token::Literal(&[0x00]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[
        RelatedFormat {
            id: 1_330,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 304,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
