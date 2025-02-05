use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_399630772: FileFormat = FileFormat {
    id: 399_630_772,
    source_type: SourceType::Httpd,
    name: "ezpix package",
    extensions: &["ez3"],
    media_types: &["application/vnd.ezpix-package"],
    signatures: &[],
    related_formats: &[],
};
