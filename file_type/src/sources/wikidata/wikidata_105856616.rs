use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856616: FileFormat = FileFormat {
    id: 105_856_616,
    source_type: SourceType::Wikidata,
    name: "World of Warcraft WDC1 database",
    extensions: &["db2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x44, 0x43, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
