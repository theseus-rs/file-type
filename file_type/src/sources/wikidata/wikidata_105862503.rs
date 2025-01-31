use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862503: FileFormat = FileFormat {
    id: 105_862_503,
    puid: "wikidata/105862503",
    name: "Drum Traker module",
    extensions: &["dtl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x54, 0x4C, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
