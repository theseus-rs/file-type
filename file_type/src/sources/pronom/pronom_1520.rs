use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1520: FileType = FileType {
    file_format: &FileFormat {
        id: 1_520,
        source_type: SourceType::Pronom,
        name: "VLW Font File",
        extensions: &["vlw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
