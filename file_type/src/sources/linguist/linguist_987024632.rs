use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_987024632: FileType = FileType {
    file_format: &FileFormat {
        id: 987_024_632,
        source_type: SourceType::Linguist,
        name: "Singularity",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
