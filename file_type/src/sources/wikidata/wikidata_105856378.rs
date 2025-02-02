use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856378: FileFormat = FileFormat {
    id: 105_856_378,
    source_type: SourceType::Wikidata,
    name: "Ci GAMES game data archive",
    extensions: &["dpk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x50, 0x4B, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
