use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2179273523: FileFormat = FileFormat {
    id: 2_179_273_523,
    source_type: SourceType::Httpd,
    name: "hal xml",
    extensions: &["hal"],
    media_types: &["application/vnd.hal+xml"],
    signatures: &[],
    related_formats: &[],
};
