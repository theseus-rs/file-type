use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3181405020922865967: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "rip",
    extensions: &["rip"],
    media_types: &["audio/vnd.rip"],
    internal_signatures: &[],
    related_formats: &[],
};
