use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858819: FileFormat = FileFormat {
    id: 105_858_819,
    puid: "wikidata/105858819",
    name: "ByteMap font format",
    extensions: &["bmf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xE1, 0xE6, 0xD5, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
