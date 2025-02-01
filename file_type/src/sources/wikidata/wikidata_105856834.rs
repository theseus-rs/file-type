use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856834: FileFormat = FileFormat {
    id: 105_856_834,
    puid: "wikidata/105856834",
    name: "HEC-RAS Geometry file",
    extensions: &["g01", "g02", "g99"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x65, 0x6F, 0x6D, 0x20, 0x54, 0x69, 0x74, 0x6C, 0x65, 0x3D,
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
                        0x47, 0x65, 0x6F, 0x6D, 0x20, 0x54, 0x69, 0x74, 0x6C, 0x65, 0x3D,
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
                        0x47, 0x65, 0x6F, 0x6D, 0x20, 0x54, 0x69, 0x74, 0x6C, 0x65, 0x3D,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
