use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856861: FileFormat = FileFormat {
    id: 105_856_861,
    source_type: SourceType::Wikidata,
    name: "American Conquest 2 game data archvie",
    extensions: &["gsc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x53, 0x43, 0x66, 0x6D, 0x74])],
            },
        }],
    }],
    related_formats: &[],
};
