use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857032: FileFormat = FileFormat {
    id: 105_857_032,
    source_type: SourceType::Wikidata,
    name: "MS Age of Empires II: The Conquerors Expansion v1.0 Saved Game",
    extensions: &["gax"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEC, 0xBD])],
            },
        }],
    }],
    related_formats: &[],
};
