use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13485419110853384641: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "atomsvc xml",
    extensions: &["atomsvc"],
    media_types: &["application/atomsvc+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
