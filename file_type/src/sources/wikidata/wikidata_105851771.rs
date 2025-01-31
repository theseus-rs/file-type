use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851771: FileFormat = FileFormat {
    id: 105_851_771,
    puid: "wikidata/105851771",
    name: "AY STRC chiptune",
    extensions: &["strc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x58, 0x41, 0x59, 0x53, 0x54, 0x52, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
