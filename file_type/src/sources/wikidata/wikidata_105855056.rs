use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855056: FileFormat = FileFormat {
    id: 105_855_056,
    puid: "wikidata/105855056",
    name: "Audio Sculpture module",
    extensions: &["adsc", "as"],
    media_types: &["audio/x-mod", "audio/x-mod"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x75, 0x64, 0x69, 0x6F, 0x53, 0x63, 0x75, 0x6C, 0x70, 0x74, 0x75,
                        0x72, 0x65, 0x31, 0x30,
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
                        0x41, 0x75, 0x64, 0x69, 0x6F, 0x53, 0x63, 0x75, 0x6C, 0x70, 0x74, 0x75,
                        0x72, 0x65, 0x31, 0x30,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
