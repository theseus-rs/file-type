use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_98: FileType = FileType {
    file_format: &FileFormat {
        id: 98,
        source_type: SourceType::Linguist,
        name: "Ecere Projects",
        extensions: &["epj"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
