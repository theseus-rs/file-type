use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_28923963: FileType = FileType {
    file_format: &FileFormat {
        id: 28_923_963,
        source_type: SourceType::Linguist,
        name: "BASIC",
        extensions: &["bas"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
