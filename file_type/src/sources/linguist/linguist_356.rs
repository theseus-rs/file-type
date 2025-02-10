use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_356: FileType = FileType {
    file_format: &FileFormat {
        id: 356,
        source_type: SourceType::Linguist,
        name: "Stan",
        extensions: &["stan"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
