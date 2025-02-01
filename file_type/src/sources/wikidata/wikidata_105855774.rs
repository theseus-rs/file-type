use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855774: FileFormat = FileFormat {
    id: 105_855_774,
    puid: "wikidata/105855774",
    name: "Davilex Games game data format",
    extensions: &["idx", "img"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x61, 0x76, 0x69, 0x6C, 0x65, 0x78, 0x20, 0x47, 0x61, 0x6D, 0x65,
                        0x73, 0x20, 0x42, 0x56, 0x00,
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
                        0x44, 0x61, 0x76, 0x69, 0x6C, 0x65, 0x78, 0x20, 0x47, 0x61, 0x6D, 0x65,
                        0x73, 0x20, 0x42, 0x56, 0x00,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
