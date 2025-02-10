use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2567: FileType = FileType {
    file_format: &FileFormat {
        id: 2_567,
        source_type: SourceType::Pronom,
        name: "Wordcraft Chapter Files",
        extensions: &["001"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
