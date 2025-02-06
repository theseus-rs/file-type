use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206957: FileFormat = FileFormat {
    id: 28_206_957,
    source_type: SourceType::Wikidata,
    name: "PhotoParade slideshow",
    extensions: &["4pp", "php"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
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
        Signature {
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
