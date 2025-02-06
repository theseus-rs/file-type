use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2193155: FileFormat = FileFormat {
    id: 2_193_155,
    source_type: SourceType::Wikidata,
    name: "Java class file",
    extensions: &["class"],
    media_types: &[
        "application/java",
        "application/java-byte-code",
        "application/java-vm",
        "application/x-httpd-java",
        "application/x-java",
        "application/x-java-class",
        "application/x-java-vm",
    ],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE])],
                },
            }],
        },
    ],
    related_formats: &[],
};
