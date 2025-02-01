use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757986: FileFormat = FileFormat {
    id: 28_757_986,
    puid: "wikidata/28757986",
    name: "IPS manifest",
    extensions: &["mf", "p5m"],
    media_types: &["text/plain", "text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x73, 0x65, 0x74, 0x20, 0x6E, 0x61, 0x6D, 0x65, 0x3D, 0x70, 0x6B, 0x67,
                        0x2E,
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
                        0x73, 0x65, 0x74, 0x20, 0x6E, 0x61, 0x6D, 0x65, 0x3D, 0x70, 0x6B, 0x67,
                        0x2E,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
