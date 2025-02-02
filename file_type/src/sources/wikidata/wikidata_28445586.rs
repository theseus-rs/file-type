use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28445586: FileFormat = FileFormat {
    id: 28_445_586,
    source_type: SourceType::Wikidata,
    name: "Application Label Temporary",
    extensions: &["axc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
