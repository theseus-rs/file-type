use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12448632667719045031: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "7z compressed",
    extensions: &["7z"],
    media_types: &["application/x-7z-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
