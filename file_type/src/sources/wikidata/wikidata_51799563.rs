use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51799563: FileFormat = FileFormat {
    id: 51_799_563,
    source_type: SourceType::Wikidata,
    name: "Quattro Pro Spreadsheet for DOS, version 5",
    extensions: &["wkq", "wq2"],
    media_types: &["application/octet-stream", "application/x-quattro-pro"],
    internal_signatures: &[
        InternalSignature {
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
        InternalSignature {
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
        InternalSignature {
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
        InternalSignature {
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
