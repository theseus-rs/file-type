use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_498022874: FileType = FileType {
    file_format: &FileFormat {
        id: 498_022_874,
        source_type: SourceType::Linguist,
        name: "Rez",
        extensions: &["r"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
