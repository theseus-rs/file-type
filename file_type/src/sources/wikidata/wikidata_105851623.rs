use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851623: FileFormat = FileFormat {
    id: 105_851_623,
    puid: "wikidata/105851623",
    name: "Wii TPL images container",
    extensions: &["tpl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x20, 0xAF, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
