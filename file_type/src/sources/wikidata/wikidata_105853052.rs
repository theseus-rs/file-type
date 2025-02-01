use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853052: FileFormat = FileFormat {
    id: 105_853_052,
    puid: "wikidata/105853052",
    name: "TIENCR format",
    extensions: &["senc", "tiencr"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x49, 0x45, 0x4E, 0x43, 0x52])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x49, 0x45, 0x4E, 0x43, 0x52])],
                },
            }],
        },
    ],
    related_formats: &[],
};
