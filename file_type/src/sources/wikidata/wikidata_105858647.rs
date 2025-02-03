use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858647: FileFormat = FileFormat {
    id: 105_858_647,
    source_type: SourceType::Wikidata,
    name: "Bradford Font (v2)",
    extensions: &["bf2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x46, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
