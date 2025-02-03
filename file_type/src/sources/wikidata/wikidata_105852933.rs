use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852933: FileFormat = FileFormat {
    id: 105_852_933,
    source_type: SourceType::Wikidata,
    name: "Crimson Fields level (with rem)",
    extensions: &["src"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
