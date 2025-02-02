use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16819287819186782796: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dolby mlp",
    extensions: &["mlp"],
    media_types: &["application/vnd.dolby.mlp"],
    internal_signatures: &[],
    related_formats: &[],
};
