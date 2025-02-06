use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2890360434: FileFormat = FileFormat {
    id: 2_890_360_434,
    source_type: SourceType::Httpd,
    name: "curl car",
    extensions: &["car"],
    media_types: &["application/vnd.curl.car"],
    signatures: &[],
    related_formats: &[],
};
