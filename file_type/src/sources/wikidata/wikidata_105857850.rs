use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857850: FileFormat = FileFormat {
    id: 105_857_850,
    puid: "wikidata/105857850",
    name: "ANDOS disk image",
    extensions: &["bkd", "dsk"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xA0, 0x00, 0x1E, 0x01, 0x41, 0x4E, 0x44, 0x4F, 0x53,
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
                        0xA0, 0x00, 0x1E, 0x01, 0x41, 0x4E, 0x44, 0x4F, 0x53,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
