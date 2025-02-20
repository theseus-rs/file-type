use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_159: FileType = FileType {
    file_format: &FileFormat {
        id: 159,
        source_type: SourceType::Linguist,
        name: "Hy",
        extensions: &["hy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
