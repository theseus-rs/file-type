use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_107: FileType = FileType {
    file_format: &FileFormat {
        id: 107,
        source_type: SourceType::Linguist,
        name: "Fortran",
        extensions: &["f", "f77", "for", "fpp"],
        media_types: &["text/x-fortran"],
        signatures: &[],
        related_formats: &[],
    },
};
