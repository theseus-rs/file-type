use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_992375436: FileType = FileType {
    file_format: &FileFormat {
        id: 992_375_436,
        source_type: SourceType::Linguist,
        name: "cURL Config",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
