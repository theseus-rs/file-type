use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_940373278: FileFormat = FileFormat {
    id: 940_373_278,
    source_type: SourceType::Httpd,
    name: "cgm",
    extensions: &["cgm"],
    media_types: &["image/cgm"],
    signatures: &[],
    related_formats: &[],
};
