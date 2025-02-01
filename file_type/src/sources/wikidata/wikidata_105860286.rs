use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860286: FileFormat = FileFormat {
    id: 105_860_286,
    puid: "wikidata/105860286",
    name: "Roblox Model",
    extensions: &["rbxm", "rbxmx"],
    media_types: &["text/xml", "text/xml"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x72, 0x6F, 0x62, 0x6C, 0x6F, 0x78])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x72, 0x6F, 0x62, 0x6C, 0x6F, 0x78])],
                },
            }],
        },
    ],
    related_formats: &[],
};
