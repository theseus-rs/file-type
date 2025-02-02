use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854413: FileFormat = FileFormat {
    id: 105_854_413,
    source_type: SourceType::Wikidata,
    name: "ZOO compressed archive (strict)",
    extensions: &["zoo"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xDC, 0xA7, 0xC4, 0xFD])],
            },
        }],
    }],
    related_formats: &[],
};
