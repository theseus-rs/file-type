use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1286: FileFormat = FileFormat {
    id: 1_286,
    source_type: SourceType::Pronom,
    name: "VivoActive",
    extensions: &["viv"],
    media_types: &["video/vnd-vivo"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(3),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0D, 0x0A, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x56, 0x69,
                        0x76, 0x6F, 0x2F, 0x31, 0x2E, 0x30, 0x30,
                    ])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(3),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0D, 0x0A, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x56, 0x69,
                        0x76, 0x6F, 0x2F, 0x32, 0x2E, 0x30, 0x30,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
