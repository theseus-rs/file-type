use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_112: FileType = FileType {
    file_format: &FileFormat {
        id: 112,
        source_type: SourceType::Pronom,
        name: "AutoCAD Source Menu File",
        extensions: &["mns"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
