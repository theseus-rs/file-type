use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17481297107178477904: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms wmx",
    extensions: &["wmx"],
    media_types: &["video/x-ms-wmx"],
    internal_signatures: &[],
    related_formats: &[],
};
