use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206957: FileFormat = FileFormat {
    id: 28_206_957,
    puid: "wikidata/28206957",
    name: "PhotoParade slideshow",
    extensions: &["4pp", "php"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x18, 0x58, 0x50, 0x42, 0x21,
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
                        0x00, 0x00, 0x00, 0x18, 0x58, 0x50, 0x42, 0x21,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
