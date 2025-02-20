use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_470: FileType = FileType {
    file_format: &FileFormat {
        id: 470,
        source_type: SourceType::Pronom,
        name: "DesignCAD Drawing",
        extensions: &["dc2", "dc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
