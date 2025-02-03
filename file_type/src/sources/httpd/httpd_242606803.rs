use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_242606803: FileFormat = FileFormat {
    id: 242_606_803,
    source_type: SourceType::Httpd,
    name: "7z compressed",
    extensions: &["7z"],
    media_types: &["application/x-7z-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
