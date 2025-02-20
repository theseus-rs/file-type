use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2698: FileType = FileType {
    file_format: &FileFormat {
        id: 2_698,
        source_type: SourceType::Pronom,
        name: "Fountain Markup Language File",
        extensions: &["spmd", "fountain"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
