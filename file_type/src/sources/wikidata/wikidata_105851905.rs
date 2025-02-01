use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851905: FileFormat = FileFormat {
    id: 105_851_905,
    puid: "wikidata/105851905",
    name: "Smali assembly source",
    extensions: &["smali"],
    media_types: &["text/smali"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2E, 0x63, 0x6C, 0x61, 0x73, 0x73, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
