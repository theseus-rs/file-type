use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_527438264: FileType = FileType {
    file_format: &FileFormat {
        id: 527_438_264,
        source_type: SourceType::Linguist,
        name: "Debian Package Control File",
        extensions: &["dsc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
