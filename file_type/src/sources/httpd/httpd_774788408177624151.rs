use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_774788408177624151: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "java source",
    extensions: &["java"],
    media_types: &["text/x-java-source"],
    internal_signatures: &[],
    related_formats: &[],
};
