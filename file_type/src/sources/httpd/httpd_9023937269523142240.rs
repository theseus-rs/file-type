use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9023937269523142240: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms lrm",
    extensions: &["lrm"],
    media_types: &["application/vnd.ms-lrm"],
    internal_signatures: &[],
    related_formats: &[],
};
