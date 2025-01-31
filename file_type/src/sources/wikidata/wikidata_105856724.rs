use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856724: FileFormat = FileFormat {
    id: 105_856_724,
    puid: "wikidata/105856724",
    name: "Yamaha Tyros4 custom voice",
    extensions: &["uvd", "uvn"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x56, 0x48, 0x44, 0x00, 0x00, 0x00, 0x50, 0x54, 0x79, 0x72, 0x6F,
                        0x73, 0x34,
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
                        0x58, 0x56, 0x48, 0x44, 0x00, 0x00, 0x00, 0x50, 0x54, 0x79, 0x72, 0x6F,
                        0x73, 0x34,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
