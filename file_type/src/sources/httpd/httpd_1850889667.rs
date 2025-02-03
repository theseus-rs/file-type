use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1850889667: FileFormat = FileFormat {
    id: 1_850_889_667,
    source_type: SourceType::Httpd,
    name: "freearc",
    extensions: &["arc"],
    media_types: &["application/x-freearc"],
    internal_signatures: &[],
    related_formats: &[],
};
