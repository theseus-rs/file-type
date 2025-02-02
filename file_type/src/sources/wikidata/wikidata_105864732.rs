use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864732: FileFormat = FileFormat {
    id: 105_864_732,
    source_type: SourceType::Wikidata,
    name: "Psychonauts game data archive",
    extensions: &["pkg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x50, 0x4B, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
