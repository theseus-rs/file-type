use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_254: FileType = FileType {
    file_format: &FileFormat {
        id: 254,
        source_type: SourceType::Linguist,
        name: "NumPy",
        extensions: &["numpy", "numpyw", "numsc"],
        media_types: &["text/x-python"],
        signatures: &[],
        related_formats: &[],
    },
};
