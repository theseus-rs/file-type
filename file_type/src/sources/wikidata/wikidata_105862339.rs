use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862339: FileFormat = FileFormat {
    id: 105_862_339,
    source_type: SourceType::Wikidata,
    name: "MindMup Mindmap",
    extensions: &["mup"],
    media_types: &["application/json"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7B])],
            },
        }],
    }],
    related_formats: &[],
};
