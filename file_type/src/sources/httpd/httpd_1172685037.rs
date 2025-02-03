use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1172685037: FileFormat = FileFormat {
    id: 1_172_685_037,
    source_type: SourceType::Httpd,
    name: "sv4cpio",
    extensions: &["sv4cpio"],
    media_types: &["application/x-sv4cpio"],
    internal_signatures: &[],
    related_formats: &[],
};
