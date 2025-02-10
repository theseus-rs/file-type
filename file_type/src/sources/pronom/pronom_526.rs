use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_526: FileType = FileType {
    file_format: &FileFormat {
        id: 526,
        source_type: SourceType::Pronom,
        name: "StarOffice Impress",
        extensions: &["sdd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
