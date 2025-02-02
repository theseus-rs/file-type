use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854466: FileFormat = FileFormat {
    id: 105_854_466,
    source_type: SourceType::Wikidata,
    name: "asciicast (v2)",
    extensions: &["cast"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7B, 0x22])],
            },
        }],
    }],
    related_formats: &[],
};
