use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3227546503: FileFormat = FileFormat {
    id: 3_227_546_503,
    source_type: SourceType::Httpd,
    name: "shockwave flash",
    extensions: &["swf"],
    media_types: &["application/x-shockwave-flash"],
    internal_signatures: &[],
    related_formats: &[],
};
