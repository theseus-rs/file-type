use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_400689417: FileFormat = FileFormat {
    id: 400_689_417,
    source_type: SourceType::Httpd,
    name: "csv",
    extensions: &["csv"],
    media_types: &["text/csv"],
    internal_signatures: &[],
    related_formats: &[],
};
