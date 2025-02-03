use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28445585: FileFormat = FileFormat {
    id: 28_445_585,
    source_type: SourceType::Wikidata,
    name: "Application Label Index",
    extensions: &["axc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
