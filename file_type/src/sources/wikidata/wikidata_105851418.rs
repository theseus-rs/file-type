use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851418: FileFormat = FileFormat {
    id: 105_851_418,
    puid: "wikidata/105851418",
    name: "Terragen terrain",
    extensions: &["ter", "terrain"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x45, 0x52, 0x52, 0x41, 0x47, 0x45, 0x4E, 0x54, 0x45, 0x52, 0x52,
                        0x41, 0x49, 0x4E, 0x20,
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
                        0x54, 0x45, 0x52, 0x52, 0x41, 0x47, 0x45, 0x4E, 0x54, 0x45, 0x52, 0x52,
                        0x41, 0x49, 0x4E, 0x20,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
