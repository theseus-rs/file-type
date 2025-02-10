use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_372: FileType = FileType {
    file_format: &FileFormat {
        id: 372,
        source_type: SourceType::Linguist,
        name: "Text",
        extensions: &["fr", "nb", "ncl", "no", "txt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
