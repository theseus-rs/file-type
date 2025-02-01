use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_11241282: FileFormat = FileFormat {
    id: 11_241_282,
    puid: "wikidata/11241282",
    name: "RPM",
    extensions: &["rpm", "rpm", "rpm", "rpm", "rpm", "rpm"],
    media_types: &[
        "application/x-redhat-package-manager",
        "application/x-redhat-package-manager",
        "application/x-redhat-package-manager",
        "application/x-rpm",
        "application/x-rpm",
        "application/x-rpm",
    ],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xED, 0xAB, 0xEE, 0xDB])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xED, 0xAB, 0xEE, 0xDB])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xED, 0xAB, 0xEE, 0xDB])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xED, 0xAB, 0xEE, 0xDB])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xED, 0xAB, 0xEE, 0xDB])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xED, 0xAB, 0xEE, 0xDB])],
                },
            }],
        },
    ],
    related_formats: &[],
};
