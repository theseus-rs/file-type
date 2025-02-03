use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1722616698: FileFormat = FileFormat {
    id: 1_722_616_698,
    source_type: SourceType::Httpd,
    name: "ms shortcut",
    extensions: &["lnk"],
    media_types: &["application/x-ms-shortcut"],
    internal_signatures: &[],
    related_formats: &[],
};
