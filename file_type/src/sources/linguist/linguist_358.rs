use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_358: FileType = FileType {
    file_format: &FileFormat {
        id: 358,
        source_type: SourceType::Linguist,
        name: "Stata",
        extensions: &["ado", "do", "doh", "ihlp", "mata", "matah", "sthlp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
