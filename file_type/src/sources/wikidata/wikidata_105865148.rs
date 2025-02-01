use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865148: FileFormat = FileFormat {
    id: 105_865_148,
    puid: "wikidata/105865148",
    name: "ASIC Project/Configuration",
    extensions: &["cfg", "prj"],
    media_types: &["text/plain", "text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x34, 0x0D, 0x0A, 0x4F, 0x42, 0x4A, 0x3D])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x34, 0x0D, 0x0A, 0x4F, 0x42, 0x4A, 0x3D])],
                },
            }],
        },
    ],
    related_formats: &[],
};
