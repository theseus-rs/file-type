use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4008680505: FileFormat = FileFormat {
    id: 4_008_680_505,
    source_type: SourceType::Httpd,
    name: "crick clicker",
    extensions: &["clkx"],
    media_types: &["application/vnd.crick.clicker"],
    signatures: &[],
    related_formats: &[],
};
