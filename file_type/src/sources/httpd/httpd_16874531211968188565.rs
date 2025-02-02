use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16874531211968188565: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms wax",
    extensions: &["wax"],
    media_types: &["audio/x-ms-wax"],
    internal_signatures: &[],
    related_formats: &[],
};
