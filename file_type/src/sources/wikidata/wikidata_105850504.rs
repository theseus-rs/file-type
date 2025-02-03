use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850504: FileFormat = FileFormat {
    id: 105_850_504,
    source_type: SourceType::Wikidata,
    name: "Civilization V saved game",
    extensions: &["civ5save"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x49, 0x56, 0x35])],
            },
        }],
    }],
    related_formats: &[],
};
