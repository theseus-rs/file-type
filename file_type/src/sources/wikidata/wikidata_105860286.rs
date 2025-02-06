use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860286: FileFormat = FileFormat {
    id: 105_860_286,
    source_type: SourceType::Wikidata,
    name: "Roblox Model",
    extensions: &["rbxm", "rbxmx"],
    media_types: &["text/xml"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x72, 0x6F, 0x62, 0x6C, 0x6F, 0x78])],
                },
            }],
        },
        Signature {
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
