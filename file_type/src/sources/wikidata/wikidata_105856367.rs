use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856367: FileFormat = FileFormat {
    id: 105_856_367,
    source_type: SourceType::Wikidata,
    name: "Normality game data archive",
    extensions: &["das"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x41, 0x53, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
