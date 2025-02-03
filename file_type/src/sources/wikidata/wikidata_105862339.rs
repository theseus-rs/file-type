use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862339: FileFormat = FileFormat {
    id: 105_862_339,
    source_type: SourceType::Wikidata,
    name: "MindMup Mindmap",
    extensions: &["mup"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
