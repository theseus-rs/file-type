use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9056205338064453253: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "kinar",
    extensions: &["kne", "knp"],
    media_types: &["application/vnd.kinar"],
    internal_signatures: &[],
    related_formats: &[],
};
