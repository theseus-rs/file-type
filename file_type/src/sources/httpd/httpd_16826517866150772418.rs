use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16826517866150772418: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "cache manifest",
    extensions: &["appcache"],
    media_types: &["text/cache-manifest"],
    internal_signatures: &[],
    related_formats: &[],
};
