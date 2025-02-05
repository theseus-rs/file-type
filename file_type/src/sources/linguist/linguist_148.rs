use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_148: FileFormat = FileFormat {
    id: 148,
    source_type: SourceType::Linguist,
    name: "HTML+ECR",
    extensions: &["ecr"],
    media_types: &["text/html"],
    signatures: &[],
    related_formats: &[],
};
