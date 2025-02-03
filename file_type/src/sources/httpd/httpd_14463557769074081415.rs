use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14463557769074081415: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "uri list",
    extensions: &["uri", "uris", "urls"],
    media_types: &["text/uri-list"],
    internal_signatures: &[],
    related_formats: &[],
};
