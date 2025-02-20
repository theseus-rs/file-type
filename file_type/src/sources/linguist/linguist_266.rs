use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_266: FileType = FileType {
    file_format: &FileFormat {
        id: 266,
        source_type: SourceType::Linguist,
        name: "OpenSCAD",
        extensions: &["scad"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
