use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14857126558942962564: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "koan",
    extensions: &["skp", "skd", "skt", "skm"],
    media_types: &["application/vnd.koan"],
    internal_signatures: &[],
    related_formats: &[],
};
