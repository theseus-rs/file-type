use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3956050975: FileFormat = FileFormat {
    id: 3_956_050_975,
    source_type: SourceType::Httpd,
    name: "java jnlp file",
    extensions: &["jnlp"],
    media_types: &["application/x-java-jnlp-file"],
    internal_signatures: &[],
    related_formats: &[],
};
