use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_79: FileType = FileType {
    file_format: &FileFormat {
        id: 79,
        source_type: SourceType::Linguist,
        name: "Cython",
        extensions: &["pxd", "pxi", "pyx"],
        media_types: &["text/x-cython"],
        signatures: &[],
        related_formats: &[],
    },
};
