use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860643: FileFormat = FileFormat {
    id: 105_860_643,
    puid: "wikidata/105860643",
    name: "Richard's Bridge Notation",
    extensions: &["rbn"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x20, 0x52, 0x42, 0x4E, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
