use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856899: FileFormat = FileFormat {
    id: 105_856_899,
    puid: "wikidata/105856899",
    name: "Microsoft Word for DOS Glossary",
    extensions: &["glx", "glx", "gly", "gly"],
    media_types: &[
        "application/msword",
        "application/msword",
        "application/octet-stream",
        "application/octet-stream",
    ],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0xBE, 0x01, 0x00, 0x00, 0xAB])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0xBE, 0x01, 0x00, 0x00, 0xAB])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0xBE, 0x01, 0x00, 0x00, 0xAB])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0xBE, 0x01, 0x00, 0x00, 0xAB])],
                },
            }],
        },
    ],
    related_formats: &[],
};
