use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858895: FileFormat = FileFormat {
    id: 105_858_895,
    puid: "wikidata/105858895",
    name: "Electronic Arts Bundle game data archive",
    extensions: &["bundle"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x62, 0x6E, 0x64, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
