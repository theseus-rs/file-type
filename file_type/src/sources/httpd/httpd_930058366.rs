use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_930058366: FileFormat = FileFormat {
    id: 930_058_366,
    source_type: SourceType::Httpd,
    name: "in3d spot",
    extensions: &["spot"],
    media_types: &["text/vnd.in3d.spot"],
    internal_signatures: &[],
    related_formats: &[],
};
