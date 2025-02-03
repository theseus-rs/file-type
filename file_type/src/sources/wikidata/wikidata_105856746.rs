use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856746: FileFormat = FileFormat {
    id: 105_856_746,
    source_type: SourceType::Wikidata,
    name: "Unreal Engine Plugin",
    extensions: &["uplugin"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7B, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
