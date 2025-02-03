use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3132406847: FileFormat = FileFormat {
    id: 3_132_406_847,
    source_type: SourceType::Httpd,
    name: "latex",
    extensions: &["latex"],
    media_types: &["application/x-latex"],
    internal_signatures: &[],
    related_formats: &[],
};
