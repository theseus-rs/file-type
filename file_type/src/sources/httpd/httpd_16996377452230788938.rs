use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16996377452230788938: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "cdx",
    extensions: &["cdx"],
    media_types: &["chemical/x-cdx"],
    internal_signatures: &[],
    related_formats: &[],
};
