use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857496: FileFormat = FileFormat {
    id: 105_857_496,
    puid: "wikidata/105857496",
    name: "Aladdin 4D Font",
    extensions: &["4df", "4dff"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x54, 0x33, 0x44])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x54, 0x33, 0x44])],
                },
            }],
        },
    ],
    related_formats: &[],
};
