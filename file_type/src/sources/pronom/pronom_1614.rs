use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1614: FileType = FileType {
    file_format: &FileFormat {
        id: 1_614,
        source_type: SourceType::Pronom,
        name: "StarOffice Impress",
        extensions: &["sdd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
