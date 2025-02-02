use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2127115815514234195: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "freearc",
    extensions: &["arc"],
    media_types: &["application/x-freearc"],
    internal_signatures: &[],
    related_formats: &[],
};
