use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5776032609182544687: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "cdmi container",
    extensions: &["cdmic"],
    media_types: &["application/cdmi-container"],
    internal_signatures: &[],
    related_formats: &[],
};
