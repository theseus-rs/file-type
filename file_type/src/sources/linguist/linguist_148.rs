use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_148: FileType = FileType {
    file_format: &FileFormat {
        id: 148,
        source_type: SourceType::Linguist,
        name: "HTML+ECR",
        extensions: &["ecr"],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
