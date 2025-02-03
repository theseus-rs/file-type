use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855416: FileFormat = FileFormat {
    id: 105_855_416,
    source_type: SourceType::Wikidata,
    name: "Playmation Figure",
    extensions: &["fig"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x49, 0x47, 0x55, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
