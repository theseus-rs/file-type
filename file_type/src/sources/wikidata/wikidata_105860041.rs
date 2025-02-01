use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860041: FileFormat = FileFormat {
    id: 105_860_041,
    puid: "wikidata/105860041",
    name: "FLIC FLC video",
    extensions: &["flc"],
    media_types: &["video/flc"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x12, 0xAF])],
            },
        }],
    }],
    related_formats: &[],
};
