use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1126349986: FileFormat = FileFormat {
    id: 1_126_349_986,
    source_type: SourceType::Httpd,
    name: "pcx",
    extensions: &["pcx"],
    media_types: &["image/x-pcx"],
    signatures: &[],
    related_formats: &[],
};
