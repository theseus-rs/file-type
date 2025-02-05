use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2349685866: FileFormat = FileFormat {
    id: 2_349_685_866,
    source_type: SourceType::Httpd,
    name: "groove account",
    extensions: &["gac"],
    media_types: &["application/vnd.groove-account"],
    signatures: &[],
    related_formats: &[],
};
