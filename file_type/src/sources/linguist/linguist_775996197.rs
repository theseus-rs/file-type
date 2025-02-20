use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_775996197: FileType = FileType {
    file_format: &FileFormat {
        id: 775_996_197,
        source_type: SourceType::Linguist,
        name: "nanorc",
        extensions: &["nanorc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
