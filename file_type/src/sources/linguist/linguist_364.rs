use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_364: FileType = FileType {
    file_format: &FileFormat {
        id: 364,
        source_type: SourceType::Linguist,
        name: "TLA",
        extensions: &["tla"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
