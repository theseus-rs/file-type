use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3923222572: FileFormat = FileFormat {
    id: 3_923_222_572,
    source_type: SourceType::Httpd,
    name: "groove identity message",
    extensions: &["gim"],
    media_types: &["application/vnd.groove-identity-message"],
    internal_signatures: &[],
    related_formats: &[],
};
