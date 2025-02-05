use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2449335360: FileFormat = FileFormat {
    id: 2_449_335_360,
    source_type: SourceType::Httpd,
    name: "kinar",
    extensions: &["kne", "knp"],
    media_types: &["application/vnd.kinar"],
    signatures: &[],
    related_formats: &[],
};
