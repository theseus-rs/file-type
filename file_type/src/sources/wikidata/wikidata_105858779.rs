use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858779: FileFormat = FileFormat {
    id: 105_858_779,
    puid: "wikidata/105858779",
    name: "QuickTime Image Format bitmap (alternate)",
    extensions: &["qif", "qtif"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x69, 0x64, 0x73, 0x63])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x69, 0x64, 0x73, 0x63])],
                },
            }],
        },
    ],
    related_formats: &[],
};
