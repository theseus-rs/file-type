use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_527928029: FileFormat = FileFormat {
    id: 527_928_029,
    source_type: SourceType::Httpd,
    name: "java archive",
    extensions: &["jar"],
    media_types: &["application/java-archive"],
    signatures: &[],
    related_formats: &[],
};
