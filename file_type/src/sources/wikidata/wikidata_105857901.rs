use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857901: FileFormat = FileFormat {
    id: 105_857_901,
    puid: "wikidata/105857901",
    name: "QCOW disk image (gen)",
    extensions: &["img", "qcow"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x51, 0x46, 0x49, 0xFB])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x51, 0x46, 0x49, 0xFB])],
                },
            }],
        },
    ],
    related_formats: &[],
};
