use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6911424546062404041: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "tga",
    extensions: &["tga"],
    media_types: &["image/x-tga"],
    internal_signatures: &[],
    related_formats: &[],
};
