use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856622: FileFormat = FileFormat {
    id: 105_856_622,
    source_type: SourceType::Wikidata,
    name: "WAD3 game data",
    extensions: &["wad"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x41, 0x44, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
