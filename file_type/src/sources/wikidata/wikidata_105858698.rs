use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858698: FileFormat = FileFormat {
    id: 105_858_698,
    puid: "wikidata/105858698",
    name: "MGR bitmap (modern, 8bit aligned)",
    extensions: &["mgr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x79, 0x7A])],
            },
        }],
    }],
    related_formats: &[],
};
