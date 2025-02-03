use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10581694100561422952: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms powerpoint",
    extensions: &["ppt", "pps", "pot"],
    media_types: &["application/vnd.ms-powerpoint"],
    internal_signatures: &[],
    related_formats: &[],
};
