use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_848: FileType = FileType {
    file_format: &FileFormat {
        id: 848,
        source_type: SourceType::Pronom,
        name: "CATIA Project",
        extensions: &["project"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
