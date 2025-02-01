use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858671: FileFormat = FileFormat {
    id: 105_858_671,
    puid: "wikidata/105858671",
    name: "XL-Paint MAX bitmap",
    extensions: &["max", "xlp"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x4C, 0x50, 0x4D])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x4C, 0x50, 0x4D])],
                },
            }],
        },
    ],
    related_formats: &[],
};
