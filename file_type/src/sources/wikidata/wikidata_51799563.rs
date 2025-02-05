use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51799563: FileFormat = FileFormat {
    id: 51_799_563,
    source_type: SourceType::Wikidata,
    name: "Quattro Pro Spreadsheet for DOS, version 5",
    extensions: &["wkq", "wq2"],
    media_types: &["application/octet-stream", "application/x-quattro-pro"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x02, 0x00, 0x21, 0x51, 0xCC, 0x00, 0x02, 0x00,
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
                        0x00, 0x00, 0x02, 0x00, 0x21, 0x51, 0xCC, 0x00, 0x02, 0x00,
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
                        0x00, 0x00, 0x02, 0x00, 0x21, 0x51, 0xCC, 0x00, 0x02, 0x00,
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
                        0x00, 0x00, 0x02, 0x00, 0x21, 0x51, 0xCC, 0x00, 0x02, 0x00,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
