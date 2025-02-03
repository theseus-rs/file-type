use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966959: FileFormat = FileFormat {
    id: 27_966_959,
    source_type: SourceType::Wikidata,
    name: "YM",
    extensions: &["ym", "ymst"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x59, 0x4D, 0x53, 0x54, 0x80, 0x00, 0x59])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x59, 0x4D, 0x36, 0x21, 0x4C, 0x65, 0x4F, 0x6E, 0x41, 0x72, 0x44, 0x21,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x59, 0x4D, 0x53, 0x54, 0x80, 0x00, 0x59])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x59, 0x4D, 0x36, 0x21, 0x4C, 0x65, 0x4F, 0x6E, 0x41, 0x72, 0x44, 0x21,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
