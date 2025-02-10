use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_943571030: FileType = FileType {
    file_format: &FileFormat {
        id: 943_571_030,
        source_type: SourceType::Linguist,
        name: "BrighterScript",
        extensions: &["bs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
