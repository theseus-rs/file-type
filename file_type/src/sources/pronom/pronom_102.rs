use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_102: FileType = FileType {
    file_format: &FileFormat {
        id: 102,
        source_type: SourceType::Pronom,
        name: "AutoCAD Landscape Library",
        extensions: &["lli"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
