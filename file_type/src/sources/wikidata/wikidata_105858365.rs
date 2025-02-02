use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858365: FileFormat = FileFormat {
    id: 105_858_365,
    source_type: SourceType::Wikidata,
    name: "E-Tracker chiptune",
    extensions: &["cop", "et", "etc", "t"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x54, 0x72, 0x61, 0x63, 0x6B, 0x65, 0x72, 0x20, 0x28, 0x43, 0x29,
                        0x20, 0x42, 0x59, 0x20, 0x45, 0x53, 0x49, 0x2E,
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
                        0x45, 0x54, 0x72, 0x61, 0x63, 0x6B, 0x65, 0x72, 0x20, 0x28, 0x43, 0x29,
                        0x20, 0x42, 0x59, 0x20, 0x45, 0x53, 0x49, 0x2E,
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
                        0x45, 0x54, 0x72, 0x61, 0x63, 0x6B, 0x65, 0x72, 0x20, 0x28, 0x43, 0x29,
                        0x20, 0x42, 0x59, 0x20, 0x45, 0x53, 0x49, 0x2E,
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
                        0x45, 0x54, 0x72, 0x61, 0x63, 0x6B, 0x65, 0x72, 0x20, 0x28, 0x43, 0x29,
                        0x20, 0x42, 0x59, 0x20, 0x45, 0x53, 0x49, 0x2E,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
