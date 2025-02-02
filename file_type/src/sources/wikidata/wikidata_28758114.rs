use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28758114: FileFormat = FileFormat {
    id: 28_758_114,
    source_type: SourceType::Wikidata,
    name: "Internet Shortcut",
    extensions: &["url"],
    media_types: &["application/x-mswinurl"],
    internal_signatures: &[],
    related_formats: &[],
};
