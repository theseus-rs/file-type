use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13251929814084281596: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "commonspace",
    extensions: &["csp"],
    media_types: &["application/vnd.commonspace"],
    internal_signatures: &[],
    related_formats: &[],
};
