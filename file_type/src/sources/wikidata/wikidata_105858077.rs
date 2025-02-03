use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858077: FileFormat = FileFormat {
    id: 105_858_077,
    source_type: SourceType::Wikidata,
    name: "IsoDraw Document",
    extensions: &["iso"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x53, 0x4F, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
