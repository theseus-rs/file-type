use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859193: FileFormat = FileFormat {
    id: 105_859_193,
    source_type: SourceType::Wikidata,
    name: "Beasts and Bumpkins game data archive",
    extensions: &["box"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x4F, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
