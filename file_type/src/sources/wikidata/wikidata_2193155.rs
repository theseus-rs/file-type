use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2193155: FileFormat = FileFormat {
    id: 2_193_155,
    puid: "wikidata/2193155",
    name: "Java class file",
    extensions: &[
        "class", "class", "class", "class", "class", "class", "class",
    ],
    media_types: &[
        "application/java",
        "application/java-byte-code",
        "application/java-vm",
        "application/x-httpd-java",
        "application/x-java",
        "application/x-java-class",
        "application/x-java-vm",
    ],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE])],
                },
            }],
        },
        InternalSignature {
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
