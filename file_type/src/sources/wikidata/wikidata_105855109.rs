use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855109: FileFormat = FileFormat {
    id: 105_855_109,
    source_type: SourceType::Wikidata,
    name: "Ashlar-Vellum 3D models (generic)",
    extensions: &["ar", "co", "xe"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x41, 0x44, 0x44, 0x2D, 0x50, 0x52, 0x4F,
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
                        0x43, 0x41, 0x44, 0x44, 0x2D, 0x50, 0x52, 0x4F,
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
                        0x43, 0x41, 0x44, 0x44, 0x2D, 0x50, 0x52, 0x4F,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
