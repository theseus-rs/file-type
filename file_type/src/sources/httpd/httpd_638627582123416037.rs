use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_638627582123416037: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "portable bitmap",
    extensions: &["pbm"],
    media_types: &["image/x-portable-bitmap"],
    internal_signatures: &[],
    related_formats: &[],
};
