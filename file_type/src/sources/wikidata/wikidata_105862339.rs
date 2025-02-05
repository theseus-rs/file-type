use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862339: FileFormat = FileFormat {
    id: 105_862_339,
    source_type: SourceType::Wikidata,
    name: "MindMup Mindmap",
    extensions: &["mup"],
    media_types: &["application/json"],
    signatures: &[],
    related_formats: &[],
};
