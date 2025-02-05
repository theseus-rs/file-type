use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_747778669: FileFormat = FileFormat {
    id: 747_778_669,
    source_type: SourceType::Httpd,
    name: "visionary",
    extensions: &["vis"],
    media_types: &["application/vnd.visionary"],
    signatures: &[],
    related_formats: &[],
};
