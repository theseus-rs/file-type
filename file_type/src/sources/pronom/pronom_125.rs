use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_125: FileType = FileType {
    file_format: &FileFormat {
        id: 125,
        source_type: SourceType::Pronom,
        name: "Inkwriter/Notetaker Template",
        extensions: &["pdt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
