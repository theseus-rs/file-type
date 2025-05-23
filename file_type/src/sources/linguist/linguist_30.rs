use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_30: FileType = FileType {
    file_format: &FileFormat {
        id: 30,
        source_type: SourceType::Linguist,
        name: "Befunge",
        extensions: &["befunge", "bf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
