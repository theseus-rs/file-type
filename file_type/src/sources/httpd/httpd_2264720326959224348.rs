use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2264720326959224348: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "kodak descriptor",
    extensions: &["sse"],
    media_types: &["application/vnd.kodak-descriptor"],
    internal_signatures: &[],
    related_formats: &[],
};
