use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858908: FileFormat = FileFormat {
    id: 105_858_908,
    source_type: SourceType::Wikidata,
    name: "Big Crunch compressed file",
    extensions: &["bc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2A, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
