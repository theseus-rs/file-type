use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16669843789120121892: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms excel",
    extensions: &["xls", "xlm", "xla", "xlc", "xlt", "xlw"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[],
    related_formats: &[],
};
