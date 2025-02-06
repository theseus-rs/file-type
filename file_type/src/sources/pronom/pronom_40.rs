use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_40: FileFormat = FileFormat {
    id: 40,
    source_type: SourceType::Pronom,
    name: "Tab-separated Values",
    extensions: &["tsv", "tab"],
    media_types: &["text/tab-separated-values"],
    signatures: &[],
    related_formats: &[],
};
