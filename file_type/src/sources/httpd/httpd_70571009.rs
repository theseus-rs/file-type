use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_70571009: FileFormat = FileFormat {
    id: 70_571_009,
    source_type: SourceType::Httpd,
    name: "dssc der",
    extensions: &["dssc"],
    media_types: &["application/dssc+der"],
    internal_signatures: &[],
    related_formats: &[],
};
