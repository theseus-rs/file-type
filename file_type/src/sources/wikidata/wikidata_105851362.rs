use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851362: FileFormat = FileFormat {
    id: 105_851_362,
    puid: "wikidata/105851362",
    name: "7DTD prefabs",
    extensions: &["tts"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x74, 0x74, 0x73, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
