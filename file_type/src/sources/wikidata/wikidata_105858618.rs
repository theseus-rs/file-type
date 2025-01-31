use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858618: FileFormat = FileFormat {
    id: 105_858_618,
    puid: "wikidata/105858618",
    name: "Blassic source (binary)",
    extensions: &["bas", "blc"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x6C, 0x61, 0x73, 0x73, 0x69, 0x63, 0x00,
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
                        0x42, 0x6C, 0x61, 0x73, 0x73, 0x69, 0x63, 0x00,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
