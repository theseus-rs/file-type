use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860039: FileFormat = FileFormat {
    id: 105_860_039,
    puid: "wikidata/105860039",
    name: "DreamForge video",
    extensions: &["dfa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x46, 0x49, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
