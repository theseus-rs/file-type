use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_235: FileType = FileType {
    file_format: &FileFormat {
        id: 235,
        source_type: SourceType::Pronom,
        name: "NAP Metafile",
        extensions: &["nap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
