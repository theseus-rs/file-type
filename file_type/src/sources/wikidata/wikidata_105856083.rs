use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856083: FileFormat = FileFormat {
    id: 105_856_083,
    puid: "wikidata/105856083",
    name: "TI-99 V9T9 Sector Dump Format",
    extensions: &["dsk", "tidisk"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x09, 0x44, 0x53, 0x4B])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x09, 0x44, 0x53, 0x4B])],
                },
            }],
        },
    ],
    related_formats: &[],
};
