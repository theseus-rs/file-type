use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_18329094975064823008: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "data vision rdz",
    extensions: &["rdz"],
    media_types: &["application/vnd.data-vision.rdz"],
    internal_signatures: &[],
    related_formats: &[],
};
