use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855464: FileFormat = FileFormat {
    id: 105_855_464,
    puid: "wikidata/105855464",
    name: "HEC-RAS Flow file",
    extensions: &["f01", "f02", "f99"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x6C, 0x6F, 0x77, 0x20, 0x54, 0x69, 0x74, 0x6C, 0x65, 0x3D,
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
                        0x46, 0x6C, 0x6F, 0x77, 0x20, 0x54, 0x69, 0x74, 0x6C, 0x65, 0x3D,
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
                        0x46, 0x6C, 0x6F, 0x77, 0x20, 0x54, 0x69, 0x74, 0x6C, 0x65, 0x3D,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
