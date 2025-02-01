use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855184: FileFormat = FileFormat {
    id: 105_855_184,
    puid: "wikidata/105855184",
    name: "Firebird database",
    extensions: &["fdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x00, 0x39, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
