use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_220: FileType = FileType {
    file_format: &FileFormat {
        id: 220,
        source_type: SourceType::Linguist,
        name: "Makefile",
        extensions: &["d", "mak", "make", "makefile", "mk", "mkfile"],
        media_types: &["text/x-cmake"],
        signatures: &[],
        related_formats: &[],
    },
};
