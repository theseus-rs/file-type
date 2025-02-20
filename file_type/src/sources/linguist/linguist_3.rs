use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_3: FileType = FileType {
    file_format: &FileFormat {
        id: 3,
        source_type: SourceType::Linguist,
        name: "AMPL",
        extensions: &["ampl", "mod"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
