use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1422188388: FileFormat = FileFormat {
    id: 1_422_188_388,
    source_type: SourceType::Httpd,
    name: "dolby mlp",
    extensions: &["mlp"],
    media_types: &["application/vnd.dolby.mlp"],
    internal_signatures: &[],
    related_formats: &[],
};
