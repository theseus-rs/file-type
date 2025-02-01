use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858925: FileFormat = FileFormat {
    id: 105_858_925,
    puid: "wikidata/105858925",
    name: "Xerox EDMICS-MMR bitmap",
    extensions: &["ed", "mmr"],
    media_types: &[
        "image/vnd.fujixerox.edmics-mmr",
        "image/vnd.fujixerox.edmics-mmr",
    ],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xFF, 0x00, 0x01])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xFF, 0x00, 0x01])],
                },
            }],
        },
    ],
    related_formats: &[],
};
