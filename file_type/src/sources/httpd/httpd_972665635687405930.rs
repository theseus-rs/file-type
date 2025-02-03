use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_972665635687405930: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "msvideo",
    extensions: &["avi"],
    media_types: &["video/x-msvideo"],
    internal_signatures: &[],
    related_formats: &[],
};
