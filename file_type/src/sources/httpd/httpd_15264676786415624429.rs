use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15264676786415624429: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "c",
    extensions: &["c", "cc", "cxx", "cpp", "h", "hh", "dic"],
    media_types: &["text/x-c"],
    internal_signatures: &[],
    related_formats: &[],
};
