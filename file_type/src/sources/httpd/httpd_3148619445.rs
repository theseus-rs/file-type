use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3148619445: FileFormat = FileFormat {
    id: 3_148_619_445,
    source_type: SourceType::Httpd,
    name: "msclip",
    extensions: &["clp"],
    media_types: &["application/x-msclip"],
    internal_signatures: &[],
    related_formats: &[],
};
