use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_344: FileType = FileType {
    file_format: &FileFormat {
        id: 344,
        source_type: SourceType::Linguist,
        name: "Scilab",
        extensions: &["sce", "sci", "tst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
