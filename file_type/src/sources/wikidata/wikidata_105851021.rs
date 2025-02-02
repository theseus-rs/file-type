use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851021: FileFormat = FileFormat {
    id: 105_851_021,
    source_type: SourceType::Wikidata,
    name: "Crystal Dynamics game data archive",
    extensions: &["tiger"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x44, 0x52, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
