use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851943: FileFormat = FileFormat {
    id: 105_851_943,
    puid: "wikidata/105851943",
    name: "Scid game moves info",
    extensions: &["sg3", "sg4"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x63, 0x69, 0x64, 0x2E, 0x73, 0x67, 0x00,
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
                        0x53, 0x63, 0x69, 0x64, 0x2E, 0x73, 0x67, 0x00,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
