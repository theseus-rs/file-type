use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851875: FileFormat = FileFormat {
    id: 105_851_875,
    puid: "wikidata/105851875",
    name: "Microsoft Word for DOS Style sheet",
    extensions: &["stx", "stx", "sty", "sty"],
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
                    tokens: &[Token::Literal(&[0x31, 0xBE, 0x02, 0x00, 0x00, 0xAB])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0xBE, 0x02, 0x00, 0x00, 0xAB])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0xBE, 0x02, 0x00, 0x00, 0xAB])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0xBE, 0x02, 0x00, 0x00, 0xAB])],
                },
            }],
        },
    ],
    related_formats: &[],
};
