use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858016: FileFormat = FileFormat {
    id: 105_858_016,
    puid: "wikidata/105858016",
    name: "HD-Copy disk image",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0x18, 0x07])],
            },
        }],
    }],
    related_formats: &[],
};
