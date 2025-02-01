use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858731: FileFormat = FileFormat {
    id: 105_858_731,
    puid: "wikidata/105858731",
    name: "MGR bitmap (old, 8-bit, 16-bit aligned, alt)",
    extensions: &["mgr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x78, 0x79])],
            },
        }],
    }],
    related_formats: &[],
};
