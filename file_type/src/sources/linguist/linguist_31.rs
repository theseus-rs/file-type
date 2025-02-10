use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_31: FileType = FileType {
    file_format: &FileFormat {
        id: 31,
        source_type: SourceType::Linguist,
        name: "Bison",
        extensions: &["bison"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
