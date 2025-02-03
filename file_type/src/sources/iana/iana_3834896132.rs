use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3834896132: FileFormat = FileFormat {
    id: 3_834_896_132,
    source_type: SourceType::Iana,
    name: "json-seq",
    extensions: &[],
    media_types: &["application/json-seq"],
    internal_signatures: &[],
    related_formats: &[],
};
