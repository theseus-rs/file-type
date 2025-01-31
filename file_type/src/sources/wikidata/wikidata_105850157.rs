use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850157: FileFormat = FileFormat {
    id: 105_850_157,
    puid: "wikidata/105850157",
    name: "FGT virus infected 16-bit COM executable",
    extensions: &["com", "img"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x53, 0x51, 0x52, 0x56, 0x57])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x53, 0x51, 0x52, 0x56, 0x57])],
                },
            }],
        },
    ],
    related_formats: &[],
};
