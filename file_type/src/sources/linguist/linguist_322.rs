use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_322: FileType = FileType {
    file_format: &FileFormat {
        id: 322,
        source_type: SourceType::Linguist,
        name: "Ren'Py",
        extensions: &["rpy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
