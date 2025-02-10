use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_167: FileType = FileType {
    file_format: &FileFormat {
        id: 167,
        source_type: SourceType::Linguist,
        name: "Inno Setup",
        extensions: &["isl", "iss"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
