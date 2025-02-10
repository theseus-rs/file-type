use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_472: FileType = FileType {
    file_format: &FileFormat {
        id: 472,
        source_type: SourceType::Pronom,
        name: "DesignCAD for Windows Drawing",
        extensions: &["dw2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
