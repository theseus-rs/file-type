use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855632: FileFormat = FileFormat {
    id: 105_855_632,
    puid: "wikidata/105855632",
    name: "OOMMF Vector Field 0.0 / Simple Vector Field",
    extensions: &["ovf", "svf"],
    media_types: &["text/plain", "text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x4F, 0x4F, 0x4D, 0x4D, 0x46, 0x3A, 0x20,
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
                        0x23, 0x20, 0x4F, 0x4F, 0x4D, 0x4D, 0x46, 0x3A, 0x20,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
