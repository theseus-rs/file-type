use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_528: FileType = FileType {
    file_format: &FileFormat {
        id: 528,
        source_type: SourceType::Pronom,
        name: "StratGraphics Data File",
        extensions: &["asf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
