use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_930957075: FileFormat = FileFormat {
    id: 930_957_075,
    source_type: SourceType::Httpd,
    name: "mynfc",
    extensions: &["taglet"],
    media_types: &["application/vnd.mynfc"],
    signatures: &[],
    related_formats: &[],
};
