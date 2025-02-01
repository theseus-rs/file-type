use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856960: FileFormat = FileFormat {
    id: 105_856_960,
    puid: "wikidata/105856960",
    name: "American Conquest game data archive",
    extensions: &["gp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x50, 0x41, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
