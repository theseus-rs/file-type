use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1467: FileFormat = FileFormat {
    id: 1_467,
    source_type: SourceType::Pronom,
    name: "Olympus RAW",
    extensions: &["orf"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4D, 0x4F, 0x52, 0x00, 0x00, 0x00, 0x08,
                    ])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x49, 0x52, 0x4F, 0x08, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x49, 0x52, 0x53, 0x08, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
