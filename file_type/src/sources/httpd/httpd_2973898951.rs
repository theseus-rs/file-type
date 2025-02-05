use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2973898951: FileFormat = FileFormat {
    id: 2_973_898_951,
    source_type: SourceType::Httpd,
    name: "portable pixmap",
    extensions: &["ppm"],
    media_types: &["image/x-portable-pixmap"],
    signatures: &[],
    related_formats: &[],
};
