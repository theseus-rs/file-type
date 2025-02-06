use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1764878474: FileFormat = FileFormat {
    id: 1_764_878_474,
    source_type: SourceType::Httpd,
    name: "dwf",
    extensions: &["dwf"],
    media_types: &["model/vnd.dwf"],
    signatures: &[],
    related_formats: &[],
};
