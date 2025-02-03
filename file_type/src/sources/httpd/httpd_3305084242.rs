use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3305084242: FileFormat = FileFormat {
    id: 3_305_084_242,
    source_type: SourceType::Httpd,
    name: "dssc xml",
    extensions: &["xdssc"],
    media_types: &["application/dssc+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
