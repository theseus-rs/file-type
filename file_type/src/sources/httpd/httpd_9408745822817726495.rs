use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9408745822817726495: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "tab separated values",
    extensions: &["tsv"],
    media_types: &["text/tab-separated-values"],
    internal_signatures: &[],
    related_formats: &[],
};
