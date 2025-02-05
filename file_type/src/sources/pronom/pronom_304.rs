use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_304: FileFormat = FileFormat {
    id: 304,
    source_type: SourceType::Pronom,
    name: "GEM Metafile Format",
    extensions: &["gem"],
    media_types: &[],
    signatures: &[
        Signature {
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
        Signature {
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
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 1_329,
    }],
};
