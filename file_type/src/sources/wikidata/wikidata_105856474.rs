use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856474: FileFormat = FileFormat {
    id: 105_856_474,
    puid: "wikidata/105856474",
    name: "WordUp document",
    extensions: &["wup"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x62, 0x21, 0x70])],
            },
        }],
    }],
    related_formats: &[],
};
