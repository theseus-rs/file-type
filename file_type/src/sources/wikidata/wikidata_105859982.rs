use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859982: FileFormat = FileFormat {
    id: 105_859_982,
    source_type: SourceType::Wikidata,
    name: "The Software Toolworks resources archive",
    extensions: &["lst", "res", "v2l", "vgh"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x49, 0x43, 0x4B, 0x42, 0x4F, 0x00, 0x76,
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
                        0x52, 0x49, 0x43, 0x4B, 0x42, 0x4F, 0x00, 0x76,
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
                        0x52, 0x49, 0x43, 0x4B, 0x42, 0x4F, 0x00, 0x76,
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
                        0x52, 0x49, 0x43, 0x4B, 0x42, 0x4F, 0x00, 0x76,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
