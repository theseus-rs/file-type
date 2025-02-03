use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856334: FileFormat = FileFormat {
    id: 105_856_334,
    source_type: SourceType::Wikidata,
    name: "Lineage Eternal game data archive",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x98, 0x41, 0x4E, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
