use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7140578712814967016: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "cdmi queue",
    extensions: &["cdmiq"],
    media_types: &["application/cdmi-queue"],
    internal_signatures: &[],
    related_formats: &[],
};
