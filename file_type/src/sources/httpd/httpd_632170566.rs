use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_632170566: FileFormat = FileFormat {
    id: 632_170_566,
    source_type: SourceType::Httpd,
    name: "handheld entertainment xml",
    extensions: &["zmm"],
    media_types: &["application/vnd.handheld-entertainment+xml"],
    signatures: &[],
    related_formats: &[],
};
