use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856386: FileFormat = FileFormat {
    id: 105_856_386,
    puid: "wikidata/105856386",
    name: "Chromadrome 2 game data archive",
    extensions: &["dam"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x5A])],
            },
        }],
    }],
    related_formats: &[],
};
