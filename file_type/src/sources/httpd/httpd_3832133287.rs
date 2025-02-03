use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3832133287: FileFormat = FileFormat {
    id: 3_832_133_287,
    source_type: SourceType::Httpd,
    name: "xv xml",
    extensions: &["mxml", "xhvml", "xvml", "xvm"],
    media_types: &["application/xv+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
