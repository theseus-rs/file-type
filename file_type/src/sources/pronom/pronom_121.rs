use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_121: FileType = FileType {
    file_format: &FileFormat {
        id: 121,
        source_type: SourceType::Pronom,
        name: "AutoCAD Plot Configuration File",
        extensions: &["pcp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
