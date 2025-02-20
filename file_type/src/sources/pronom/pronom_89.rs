use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_89: FileType = FileType {
    file_format: &FileFormat {
        id: 89,
        source_type: SourceType::Pronom,
        name: "Frame Vector Metafile",
        extensions: &["fmv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
