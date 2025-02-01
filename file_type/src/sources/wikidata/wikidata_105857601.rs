use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857601: FileFormat = FileFormat {
    id: 105_857_601,
    puid: "wikidata/105857601",
    name: "ADFS disk image",
    extensions: &["adf", "adfs", "adl"],
    media_types: &[
        "application/octet-stream",
        "application/octet-stream",
        "application/octet-stream",
    ],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x75, 0x67, 0x6F])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x75, 0x67, 0x6F])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x75, 0x67, 0x6F])],
                },
            }],
        },
    ],
    related_formats: &[],
};
