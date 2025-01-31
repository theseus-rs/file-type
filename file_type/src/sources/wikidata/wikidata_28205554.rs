use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205554: FileFormat = FileFormat {
    id: 28_205_554,
    puid: "wikidata/28205554",
    name: "Nokia Logo Manager bitmap",
    extensions: &["nlm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x4C, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
