use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858226: FileFormat = FileFormat {
    id: 105_858_226,
    puid: "wikidata/105858226",
    name: "EnCase hash Map (v4)",
    extensions: &["enmap"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x4E, 0x4D, 0x41, 0x50, 0x20, 0x56, 0x34, 0x0B, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
