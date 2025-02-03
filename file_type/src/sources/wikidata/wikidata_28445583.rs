use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28445583: FileFormat = FileFormat {
    id: 28_445_583,
    source_type: SourceType::Wikidata,
    name: "Application Label Cache",
    extensions: &["axc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
