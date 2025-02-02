use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_40: FileFormat = FileFormat {
    id: 40,
    source_type: SourceType::Pronom,
    name: "Tab-separated Values",
    extensions: &["tsv", "tab"],
    media_types: &["text/tab-separated-values"],
    internal_signatures: &[],
    related_formats: &[],
};
