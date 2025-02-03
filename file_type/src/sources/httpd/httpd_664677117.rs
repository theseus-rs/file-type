use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_664677117: FileFormat = FileFormat {
    id: 664_677_117,
    source_type: SourceType::Httpd,
    name: "nokia radio preset",
    extensions: &["rpst"],
    media_types: &["application/vnd.nokia.radio-preset"],
    internal_signatures: &[],
    related_formats: &[],
};
