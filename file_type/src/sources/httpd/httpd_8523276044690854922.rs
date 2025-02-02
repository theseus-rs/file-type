use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_8523276044690854922: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "webp",
    extensions: &["webp"],
    media_types: &["image/webp"],
    internal_signatures: &[],
    related_formats: &[],
};
