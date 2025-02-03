use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14392858622867149957: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "groove identity message",
    extensions: &["gim"],
    media_types: &["application/vnd.groove-identity-message"],
    internal_signatures: &[],
    related_formats: &[],
};
