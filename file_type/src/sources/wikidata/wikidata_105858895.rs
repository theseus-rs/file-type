use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858895: FileFormat = FileFormat {
    id: 105_858_895,
    source_type: SourceType::Wikidata,
    name: "Electronic Arts Bundle game data archive",
    extensions: &["bundle"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
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
