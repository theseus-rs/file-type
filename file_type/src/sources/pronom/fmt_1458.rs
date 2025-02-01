use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1458: FileFormat = FileFormat {
    id: 2_281,
    puid: "fmt/1458",
    name: "Arts & Letters Graphics File",
    extensions: &["ged"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x72, 0x74, 0x73, 0x20, 0x26, 0x20, 0x4C, 0x65, 0x74, 0x74, 0x65,
                        0x72, 0x73, 0x20,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x26, 0x4C, 0x2D, 0x00, 0x41, 0x72, 0x74, 0x73, 0x20, 0x26, 0x20,
                        0x4C, 0x65, 0x74, 0x74, 0x65, 0x72, 0x73, 0x20,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
