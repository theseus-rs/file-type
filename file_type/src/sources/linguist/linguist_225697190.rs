use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_225697190: FileType = FileType {
    file_format: &FileFormat {
        id: 225_697_190,
        source_type: SourceType::Linguist,
        name: "Kusto",
        extensions: &["csl", "kql"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
