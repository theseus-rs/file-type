use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858864: FileFormat = FileFormat {
    id: 105_858_864,
    puid: "wikidata/105858864",
    name: "MGR bitmap (old, 8-bit, 16-bit aligned)",
    extensions: &["mgr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7A, 0x79])],
            },
        }],
    }],
    related_formats: &[],
};
