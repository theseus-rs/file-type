use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_783851439: FileFormat = FileFormat {
    id: 783_851_439,
    source_type: SourceType::Httpd,
    name: "uri list",
    extensions: &["uri", "uris", "urls"],
    media_types: &["text/uri-list"],
    internal_signatures: &[],
    related_formats: &[],
};
