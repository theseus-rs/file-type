use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_430685397: FileFormat = FileFormat {
    id: 430_685_397,
    source_type: SourceType::Httpd,
    name: "apple diskimage",
    extensions: &["dmg"],
    media_types: &["application/x-apple-diskimage"],
    signatures: &[],
    related_formats: &[],
};
