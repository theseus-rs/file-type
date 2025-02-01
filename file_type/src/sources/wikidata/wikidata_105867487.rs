use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867487: FileFormat = FileFormat {
    id: 105_867_487,
    puid: "wikidata/105867487",
    name: "64NET container",
    extensions: &["n64"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x36, 0x34, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
