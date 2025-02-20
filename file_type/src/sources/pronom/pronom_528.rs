use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
