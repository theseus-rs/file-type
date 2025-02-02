use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6032337803232040227: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "tiff",
    extensions: &["tiff", "tif"],
    media_types: &["image/tiff"],
    internal_signatures: &[],
    related_formats: &[],
};
