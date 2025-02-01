use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857238: FileFormat = FileFormat {
    id: 105_857_238,
    puid: "wikidata/105857238",
    name: "Hard Disk Menu System menu",
    extensions: &["000", "999"],
    media_types: &["text/plain", "text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x44, 0x4D, 0x53, 0x0D, 0x0A])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x44, 0x4D, 0x53, 0x0D, 0x0A])],
                },
            }],
        },
    ],
    related_formats: &[],
};
