use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855697: FileFormat = FileFormat {
    id: 105_855_697,
    source_type: SourceType::Wikidata,
    name: "Origin Pack game data archive",
    extensions: &["opk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x50, 0x4B, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
