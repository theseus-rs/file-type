use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1813270382: FileFormat = FileFormat {
    id: 1_813_270_382,
    source_type: SourceType::Httpd,
    name: "xslt xml",
    extensions: &["xslt"],
    media_types: &["application/xslt+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
