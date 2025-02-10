use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_606708469: FileType = FileType {
    file_format: &FileFormat {
        id: 606_708_469,
        source_type: SourceType::Linguist,
        name: "Tact",
        extensions: &["tact"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
