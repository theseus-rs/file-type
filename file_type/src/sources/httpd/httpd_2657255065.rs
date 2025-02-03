use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2657255065: FileFormat = FileFormat {
    id: 2_657_255_065,
    source_type: SourceType::Httpd,
    name: "icon",
    extensions: &["ico"],
    media_types: &["image/x-icon"],
    internal_signatures: &[],
    related_formats: &[],
};
