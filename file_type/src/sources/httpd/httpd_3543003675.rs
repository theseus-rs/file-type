use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3543003675: FileFormat = FileFormat {
    id: 3_543_003_675,
    source_type: SourceType::Httpd,
    name: "cache manifest",
    extensions: &["appcache"],
    media_types: &["text/cache-manifest"],
    internal_signatures: &[],
    related_formats: &[],
};
