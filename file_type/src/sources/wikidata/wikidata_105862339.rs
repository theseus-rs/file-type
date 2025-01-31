use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862339: FileFormat = FileFormat {
    id: 105_862_339,
    puid: "wikidata/105862339",
    name: "MindMup Mindmap",
    extensions: &["mup"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
