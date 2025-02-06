use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4029535853: FileFormat = FileFormat {
    id: 4_029_535_853,
    source_type: SourceType::Httpd,
    name: "webp",
    extensions: &["webp"],
    media_types: &["image/webp"],
    signatures: &[],
    related_formats: &[],
};
