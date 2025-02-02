use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866258: FileFormat = FileFormat {
    id: 105_866_258,
    source_type: SourceType::Wikidata,
    name: "Philips Music Creator data (generic)",
    extensions: &["acc", "rlt", "sam", "seq", "spt", "voc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x55, 0x53, 0x49, 0x43, 0x42, 0x4F, 0x58, 0x00,
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
                        0x4D, 0x55, 0x53, 0x49, 0x43, 0x42, 0x4F, 0x58, 0x00,
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
                        0x4D, 0x55, 0x53, 0x49, 0x43, 0x42, 0x4F, 0x58, 0x00,
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
                        0x4D, 0x55, 0x53, 0x49, 0x43, 0x42, 0x4F, 0x58, 0x00,
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
                        0x4D, 0x55, 0x53, 0x49, 0x43, 0x42, 0x4F, 0x58, 0x00,
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
                        0x4D, 0x55, 0x53, 0x49, 0x43, 0x42, 0x4F, 0x58, 0x00,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
