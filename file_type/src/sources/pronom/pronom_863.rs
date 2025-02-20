use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_863: FileType = FileType {
    file_format: &FileFormat {
        id: 863,
        source_type: SourceType::Pronom,
        name: "Revit Workspace",
        extensions: &["rws"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
