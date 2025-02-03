use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_74595847: FileFormat = FileFormat {
    id: 74_595_847,
    source_type: SourceType::Httpd,
    name: "bmp",
    extensions: &["bmp"],
    media_types: &["image/bmp"],
    internal_signatures: &[],
    related_formats: &[],
};
