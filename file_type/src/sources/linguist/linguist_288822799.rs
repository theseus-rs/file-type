use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_288822799: FileType = FileType {
    file_format: &FileFormat {
        id: 288_822_799,
        source_type: SourceType::Linguist,
        name: "Pkl",
        extensions: &["pkl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
