use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28770330: FileFormat = FileFormat {
    id: 28_770_330,
    puid: "wikidata/28770330",
    name: "LightWave Scene",
    extensions: &["lws", "scn"],
    media_types: &["image/x-lws", "image/x-lws"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x57, 0x53, 0x43])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x57, 0x53, 0x43])],
                },
            }],
        },
    ],
    related_formats: &[],
};
