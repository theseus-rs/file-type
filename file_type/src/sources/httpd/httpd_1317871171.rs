use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1317871171: FileFormat = FileFormat {
    id: 1_317_871_171,
    source_type: SourceType::Httpd,
    name: "fmi flexstor",
    extensions: &["flx"],
    media_types: &["text/vnd.fmi.flexstor"],
    signatures: &[],
    related_formats: &[],
};
