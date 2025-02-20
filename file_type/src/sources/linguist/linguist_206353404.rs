use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_206353404: FileType = FileType {
    file_format: &FileFormat {
        id: 206_353_404,
        source_type: SourceType::Linguist,
        name: "Fluent",
        extensions: &["ftl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
