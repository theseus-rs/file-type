use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_435: FileType = FileType {
    file_format: &FileFormat {
        id: 435,
        source_type: SourceType::Pronom,
        name: "IBM DisplayWrite Final Form Text File",
        extensions: &["fft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
