use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1929: FileType = FileType {
    file_format: &FileFormat {
        id: 1_929,
        source_type: SourceType::Pronom,
        name: "Jupyter Python Notebook",
        extensions: &["ipynb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
