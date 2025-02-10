use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2075: FileType = FileType {
    file_format: &FileFormat {
        id: 2_075,
        source_type: SourceType::Pronom,
        name: "AutoCAD Temporary File",
        extensions: &["ac$"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
