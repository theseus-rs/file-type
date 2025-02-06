use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853919: FileFormat = FileFormat {
    id: 105_853_919,
    source_type: SourceType::Wikidata,
    name: "6pack compressed data",
    extensions: &["6pack", "6pk"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x89, 0x36, 0x50, 0x4B, 0x0D, 0x0A, 0x1A, 0x0A,
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
                        0x89, 0x36, 0x50, 0x4B, 0x0D, 0x0A, 0x1A, 0x0A,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
