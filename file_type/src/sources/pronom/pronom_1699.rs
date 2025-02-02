use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1699: FileFormat = FileFormat {
    id: 1_699,
    source_type: SourceType::Pronom,
    name: "JEOL NMR Spectroscopy",
    extensions: &["jdf"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4A, 0x45, 0x4F, 0x4C, 0x2E, 0x4E, 0x4D, 0x52,
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
                        0x52, 0x4D, 0x4E, 0x2E, 0x4C, 0x4F, 0x45, 0x4A,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
