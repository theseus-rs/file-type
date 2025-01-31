use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858737: FileFormat = FileFormat {
    id: 105_858_737,
    puid: "wikidata/105858737",
    name: "ImageMagick Machine independent File Format bitmap",
    extensions: &["mif", "miff"],
    media_types: &["image/x-miff", "image/x-miff"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x64, 0x3D, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x4D, 0x61, 0x67, 0x69, 0x63,
                        0x6B,
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
                        0x64, 0x3D, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x4D, 0x61, 0x67, 0x69, 0x63,
                        0x6B,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
