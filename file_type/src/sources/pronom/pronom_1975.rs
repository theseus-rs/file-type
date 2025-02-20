use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1975: FileType = FileType {
    file_format: &FileFormat {
        id: 1_975,
        source_type: SourceType::Pronom,
        name: "Praat Script File",
        extensions: &["praat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
