use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_193: FileType = FileType {
    file_format: &FileFormat {
        id: 193,
        source_type: SourceType::Pronom,
        name: "AutoCAD Device-Independent Binary Plotter File",
        extensions: &["adi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
