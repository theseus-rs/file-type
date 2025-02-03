use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16906692640029232947: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "gca compressed",
    extensions: &["gca"],
    media_types: &["application/x-gca-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
