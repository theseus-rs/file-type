use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2215: FileType = FileType {
    file_format: &FileFormat {
        id: 2_215,
        source_type: SourceType::Pronom,
        name: "FARO Laser Scan File",
        extensions: &["fls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
