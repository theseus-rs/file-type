use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1642758992: FileFormat = FileFormat {
    id: 1_642_758_992,
    source_type: SourceType::Httpd,
    name: "flographit",
    extensions: &["gph"],
    media_types: &["application/vnd.flographit"],
    signatures: &[],
    related_formats: &[],
};
