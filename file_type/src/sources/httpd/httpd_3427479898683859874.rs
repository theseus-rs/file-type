use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3427479898683859874: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "portable graymap",
    extensions: &["pgm"],
    media_types: &["image/x-portable-graymap"],
    internal_signatures: &[],
    related_formats: &[],
};
