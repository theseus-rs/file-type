use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4228335605: FileFormat = FileFormat {
    id: 4_228_335_605,
    source_type: SourceType::Httpd,
    name: "osgi dp",
    extensions: &["dp"],
    media_types: &["application/vnd.osgi.dp"],
    internal_signatures: &[],
    related_formats: &[],
};
