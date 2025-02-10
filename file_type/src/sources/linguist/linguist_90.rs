use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_90: FileType = FileType {
    file_format: &FileFormat {
        id: 90,
        source_type: SourceType::Linguist,
        name: "Dogescript",
        extensions: &["djs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
