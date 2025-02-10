use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_959889508: FileType = FileType {
    file_format: &FileFormat {
        id: 959_889_508,
        source_type: SourceType::Linguist,
        name: "Talon",
        extensions: &["talon"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
