use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_628719206: FileFormat = FileFormat {
    id: 628_719_206,
    source_type: SourceType::Httpd,
    name: "tiff",
    extensions: &["tiff", "tif"],
    media_types: &["image/tiff"],
    internal_signatures: &[],
    related_formats: &[],
};
