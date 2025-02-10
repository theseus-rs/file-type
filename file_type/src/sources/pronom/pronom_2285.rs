use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2285: FileType = FileType {
    file_format: &FileFormat {
        id: 2_285,
        source_type: SourceType::Pronom,
        name: "Comic Book Archive",
        extensions: &["cb7", "cba", "cbr", "cbt", "cbz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
