use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1414658833: FileFormat = FileFormat {
    id: 1_414_658_833,
    source_type: SourceType::Httpd,
    name: "blorb",
    extensions: &["blb", "blorb"],
    media_types: &["application/x-blorb"],
    signatures: &[],
    related_formats: &[],
};
