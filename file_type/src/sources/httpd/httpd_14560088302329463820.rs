use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14560088302329463820: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dynageo",
    extensions: &["geo"],
    media_types: &["application/vnd.dynageo"],
    internal_signatures: &[],
    related_formats: &[],
};
