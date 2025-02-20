use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
