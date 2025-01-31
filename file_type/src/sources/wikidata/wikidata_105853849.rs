use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853849: FileFormat = FileFormat {
    id: 105_853_849,
    puid: "wikidata/105853849",
    name: "AWeb Library/module",
    extensions: &["aweblib"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x03, 0xF3])],
            },
        }],
    }],
    related_formats: &[],
};
