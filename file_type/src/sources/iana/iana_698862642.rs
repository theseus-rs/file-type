use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_698862642: FileFormat = FileFormat {
    id: 698_862_642,
    source_type: SourceType::Iana,
    name: "vnd.d3m-problem",
    extensions: &[],
    media_types: &["application/vnd.d3m-problem"],
    internal_signatures: &[],
    related_formats: &[],
};
