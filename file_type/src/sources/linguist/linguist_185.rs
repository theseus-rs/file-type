use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_185: FileType = FileType {
    file_format: &FileFormat {
        id: 185,
        source_type: SourceType::Linguist,
        name: "Jupyter Notebook",
        extensions: &["ipynb"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
