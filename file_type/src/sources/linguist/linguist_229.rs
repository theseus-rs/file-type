use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_229: FileType = FileType {
    file_format: &FileFormat {
        id: 229,
        source_type: SourceType::Linguist,
        name: "Mercury",
        extensions: &["m", "moo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
