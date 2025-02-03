use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4738532386085573748: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "visionary",
    extensions: &["vis"],
    media_types: &["application/vnd.visionary"],
    internal_signatures: &[],
    related_formats: &[],
};
