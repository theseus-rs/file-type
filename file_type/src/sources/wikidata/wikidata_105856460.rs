use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856460: FileFormat = FileFormat {
    id: 105_856_460,
    puid: "wikidata/105856460",
    name: "Windows Imaging Format (generic)",
    extensions: &["esd", "swm", "wim"],
    media_types: &[
        "application/x-ms-wim",
        "application/x-ms-wim",
        "application/x-ms-wim",
    ],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x53, 0x57, 0x49, 0x4D, 0x00, 0x00])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x53, 0x57, 0x49, 0x4D, 0x00, 0x00])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x53, 0x57, 0x49, 0x4D, 0x00, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};
